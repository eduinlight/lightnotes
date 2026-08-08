use std::collections::HashSet;
use std::time::Duration;

use dioxus::prelude::*;
use dioxus_i18n::t;
use store_sdk::{ReminderSchedule, StoreHandle};

use super::plan::{
  desired_reminders, diff, due_reminders, ScheduleAction, ScheduledRecord, ScheduledReminder, CATCH_UP_WINDOW_MS, MAX_SCHEDULED,
};
use crate::state::i18n::format_absolute;
use crate::state::notes::now_ms;
use crate::state::scheduler::{self, Notification, SchedulerSupport};
use crate::state::{date_math, local_now_ms, use_boot, Note, NotesStore};

const RECONCILE_DEBOUNCE_MS: u64 = 1_000;
const TICK_MS: u64 = 30_000;

fn notification_title(note: &Note) -> String {
  match note.title.trim() {
    "" => t!("reminder-notification-untitled"),
    title => title.to_string(),
  }
}

fn notification_for(reminder: &ScheduledReminder) -> Notification {
  Notification {
    title: match reminder.title.trim() {
      "" => t!("reminder-notification-untitled"),
      title => title.to_string(),
    },
    body: t!("reminder-notification-body", when: format_absolute(reminder.fire_at_local_ms)),
  }
}

fn record_of(schedule: ReminderSchedule) -> ScheduledRecord {
  ScheduledRecord {
    note_id: schedule.note_id,
    fire_at_local_ms: schedule.fire_at_ms,
    payload_hash: schedule.payload_hash,
  }
}

async fn reconcile(handle: StoreHandle, user_id: String, desired: Vec<ScheduledReminder>) {
  let current: Vec<ScheduledRecord> = handle.load_reminder_schedules(&user_id).await.into_iter().map(record_of).collect();
  let actions = diff(&desired, &current);

  if actions.is_empty() {
    return;
  }

  for action in scheduler::apply(actions).await {
    match action {
      ScheduleAction::Set(reminder) => {
        let schedule = ReminderSchedule {
          note_id: reminder.note_id,
          fire_at_ms: reminder.fire_at_local_ms,
          payload_hash: reminder.payload_hash,
          scheduled_at_ms: now_ms(),
        };
        handle.upsert_reminder_schedule(&user_id, &schedule).await;
      }
      ScheduleAction::Remove { note_id } => handle.delete_reminder_schedule(&user_id, &note_id).await,
    }
  }
}

pub fn use_reminders(store: NotesStore) {
  let handle = use_context::<StoreHandle>();
  let boot = use_boot();
  let mut support = use_signal(SchedulerSupport::default);
  let mut generation = use_signal(|| 0u64);
  let mut pending = use_signal(Vec::<ScheduledReminder>::new);

  use_hook(move || {
    spawn(async move {
      support.set(scheduler::support().await);
    });
  });

  let reconcile_handle = handle.clone();
  use_effect(move || {
    let hydrated = (boot.store_ready)();
    let _ = store.snapshot();

    if !hydrated || !support().background {
      return;
    }

    let user_id = store.peek_user_id();
    let generation_at_spawn = {
      let mut generation = generation.write();
      *generation += 1;
      *generation
    };

    let desired = match user_id.is_empty() {
      true => Vec::new(),
      false => desired_reminders(&store.peek_notes(), local_now_ms(), MAX_SCHEDULED, notification_title),
    };

    let handle = reconcile_handle.clone();

    spawn(async move {
      tokio::time::sleep(Duration::from_millis(RECONCILE_DEBOUNCE_MS)).await;

      if *generation.peek() != generation_at_spawn {
        return;
      }

      if user_id.is_empty() {
        scheduler::clear_all().await;
        return;
      }

      reconcile(handle, user_id, desired).await;
    });
  });

  use_hook(move || {
    spawn(async move {
      let mut delivered = HashSet::<(String, i64)>::new();

      loop {
        tokio::time::sleep(Duration::from_millis(TICK_MS)).await;

        if support().background || !date_math::local_offset_ready() {
          continue;
        }

        if store.peek_user_id().is_empty() {
          continue;
        }

        let due = due_reminders(
          &store.peek_notes(),
          now_ms(),
          date_math::local_offset_ms(),
          CATCH_UP_WINDOW_MS,
          |note| note.title.clone(),
        );

        let fresh: Vec<ScheduledReminder> = due
          .into_iter()
          .filter(|reminder| delivered.insert((reminder.note_id.clone(), reminder.fire_at_local_ms)))
          .collect();

        if !fresh.is_empty() {
          pending.write().extend(fresh);
        }
      }
    });
  });

  use_effect(move || {
    if pending.read().is_empty() {
      return;
    }

    for reminder in std::mem::take(&mut *pending.write()) {
      scheduler::notify_now(notification_for(&reminder));
    }
  });
}
