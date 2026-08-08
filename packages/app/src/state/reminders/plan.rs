use crate::state::date_math::MS_PER_HOUR;
use crate::state::scheduler::{ScheduleAction, ScheduledReminder};
use crate::state::Note;

pub const MAX_SCHEDULED: usize = 60;
pub const CATCH_UP_WINDOW_MS: i64 = 60 * 60 * 1000;

#[derive(Debug, Clone, PartialEq)]
pub struct ScheduledRecord {
  pub note_id: String,
  pub fire_at_local_ms: i64,
  pub payload_hash: String,
}

pub fn fire_at_local_ms(date_ms: i64, remind_before_hours: i64) -> i64 {
  date_ms - remind_before_hours * MS_PER_HOUR
}

pub fn payload_hash(title: &str, body: &str) -> String {
  let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
  for byte in title.as_bytes().iter().chain(b"\x1f").chain(body.as_bytes()) {
    hash ^= *byte as u64;
    hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
  }
  format!("{hash:016x}")
}

fn reminder_for<F>(note: &Note, payload_for: &F) -> Option<ScheduledReminder>
where
  F: Fn(&Note, i64) -> (String, String),
{
  let hours = note.remind_before_hours?;
  let fire_at_local_ms = fire_at_local_ms(note.date_ms, hours);
  let (title, body) = payload_for(note, fire_at_local_ms);

  Some(ScheduledReminder {
    note_id: note.id.clone(),
    fire_at_local_ms,
    payload_hash: payload_hash(&title, &body),
    title,
    body,
  })
}

pub fn desired_reminders<F>(notes: &[Note], now_local_ms: i64, max: usize, payload_for: F) -> Vec<ScheduledReminder>
where
  F: Fn(&Note, i64) -> (String, String),
{
  let mut reminders: Vec<ScheduledReminder> = notes
    .iter()
    .filter_map(|note| reminder_for(note, &payload_for))
    .filter(|reminder| reminder.fire_at_local_ms > now_local_ms)
    .collect();

  reminders.sort_by(|a, b| a.fire_at_local_ms.cmp(&b.fire_at_local_ms).then_with(|| a.note_id.cmp(&b.note_id)));
  reminders.truncate(max);
  reminders
}

pub fn due_notes(notes: &[Note], now_utc_ms: i64, local_offset_ms: i64, catch_up_ms: i64) -> Vec<(Note, i64)> {
  notes
    .iter()
    .filter_map(|note| Some((note, fire_at_local_ms(note.date_ms, note.remind_before_hours?))))
    .filter(|(_, fire_at_local_ms)| {
      let lateness = now_utc_ms - (fire_at_local_ms - local_offset_ms);
      (0..=catch_up_ms).contains(&lateness)
    })
    .map(|(note, fire_at_local_ms)| (note.clone(), fire_at_local_ms))
    .collect()
}

pub fn diff(desired: &[ScheduledReminder], current: &[ScheduledRecord]) -> Vec<ScheduleAction> {
  let mut actions: Vec<ScheduleAction> = desired
    .iter()
    .filter(|reminder| {
      !current.iter().any(|record| {
        record.note_id == reminder.note_id
          && record.fire_at_local_ms == reminder.fire_at_local_ms
          && record.payload_hash == reminder.payload_hash
      })
    })
    .cloned()
    .map(ScheduleAction::Set)
    .collect();

  actions.extend(
    current
      .iter()
      .filter(|record| !desired.iter().any(|reminder| reminder.note_id == record.note_id))
      .map(|record| ScheduleAction::Remove { note_id: record.note_id.clone() }),
  );

  actions
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::state::date_math::{ymdhm_to_date_ms, MS_PER_MINUTE};

  fn note(id: &str, date_ms: i64, remind_before_hours: Option<i64>) -> Note {
    Note {
      user_id: "user-1".into(),
      id: id.into(),
      title: format!("{id} title"),
      content: String::new(),
      folder_id: None,
      tag_ids: Vec::new(),
      pinned: false,
      starred: false,
      updated_at_ms: 0,
      order: 0,
      date_ms,
      remind_before_hours,
    }
  }

  fn titles(note: &Note, _fire_at_local_ms: i64) -> (String, String) {
    (note.title.clone(), "due".to_string())
  }

  fn record(reminder: &ScheduledReminder) -> ScheduledRecord {
    ScheduledRecord {
      note_id: reminder.note_id.clone(),
      fire_at_local_ms: reminder.fire_at_local_ms,
      payload_hash: reminder.payload_hash.clone(),
    }
  }

  #[test]
  fn at_the_time_fires_on_the_note_date() {
    let date_ms = ymdhm_to_date_ms(2026, 8, 8, 9, 0);
    assert_eq!(fire_at_local_ms(date_ms, 0), date_ms);
  }

  #[test]
  fn a_week_before_subtracts_seven_days() {
    let date_ms = ymdhm_to_date_ms(2026, 8, 8, 9, 0);
    assert_eq!(fire_at_local_ms(date_ms, 168), ymdhm_to_date_ms(2026, 8, 1, 9, 0));
  }

  #[test]
  fn notes_without_a_reminder_are_never_scheduled() {
    let now = ymdhm_to_date_ms(2026, 8, 8, 9, 0);
    let notes = [note("note-1", now + MS_PER_HOUR, None)];

    assert!(desired_reminders(&notes, now, MAX_SCHEDULED, titles).is_empty());
  }

  #[test]
  fn reminders_already_in_the_past_are_never_scheduled() {
    let now = ymdhm_to_date_ms(2026, 8, 8, 9, 0);
    let notes = [note("note-1", now - MS_PER_HOUR, Some(0))];

    assert!(desired_reminders(&notes, now, MAX_SCHEDULED, titles).is_empty());
  }

  #[test]
  fn the_soonest_reminders_win_when_the_cap_is_reached() {
    let now = ymdhm_to_date_ms(2026, 8, 8, 9, 0);
    let notes: Vec<Note> = (0..5)
      .map(|index| note(&format!("note-{index}"), now + (5 - index) * MS_PER_HOUR, Some(0)))
      .collect();

    let desired = desired_reminders(&notes, now, 2, titles);

    assert_eq!(desired.len(), 2);
    assert_eq!(desired[0].note_id, "note-4");
    assert_eq!(desired[1].note_id, "note-3");
  }

  #[test]
  fn the_local_offset_decides_when_a_reminder_is_due() {
    let fire_at_local = ymdhm_to_date_ms(2026, 8, 8, 9, 0);
    let offset = -5 * MS_PER_HOUR;
    let notes = [note("note-1", fire_at_local, Some(0))];

    let at_nine_utc = due_notes(&notes, fire_at_local, offset, CATCH_UP_WINDOW_MS);
    let at_two_utc = due_notes(&notes, fire_at_local + 5 * MS_PER_HOUR, offset, CATCH_UP_WINDOW_MS);

    assert!(at_nine_utc.is_empty());
    assert_eq!(at_two_utc.len(), 1);
  }

  #[test]
  fn the_catch_up_window_is_inclusive_at_both_ends() {
    let fire_at = ymdhm_to_date_ms(2026, 8, 8, 9, 0);
    let notes = [note("note-1", fire_at, Some(0))];
    let due_at = |now| due_notes(&notes, now, 0, CATCH_UP_WINDOW_MS).len();

    assert_eq!(due_at(fire_at - 1), 0);
    assert_eq!(due_at(fire_at), 1);
    assert_eq!(due_at(fire_at + CATCH_UP_WINDOW_MS), 1);
    assert_eq!(due_at(fire_at + CATCH_UP_WINDOW_MS + 1), 0);
  }

  #[test]
  fn an_unchanged_schedule_produces_no_actions() {
    let now = ymdhm_to_date_ms(2026, 8, 8, 9, 0);
    let notes = [note("note-1", now + MS_PER_HOUR, Some(0))];
    let desired = desired_reminders(&notes, now, MAX_SCHEDULED, titles);
    let current: Vec<ScheduledRecord> = desired.iter().map(record).collect();

    assert!(diff(&desired, &current).is_empty());
  }

  #[test]
  fn a_new_reminder_is_set_and_a_dropped_one_is_removed() {
    let now = ymdhm_to_date_ms(2026, 8, 8, 9, 0);
    let notes = [note("note-2", now + MS_PER_HOUR, Some(0))];
    let desired = desired_reminders(&notes, now, MAX_SCHEDULED, titles);
    let current = vec![ScheduledRecord {
      note_id: "note-1".into(),
      fire_at_local_ms: now + MS_PER_HOUR,
      payload_hash: payload_hash("note-1 title", "due"),
    }];

    let actions = diff(&desired, &current);

    assert_eq!(actions.len(), 2);
    assert!(matches!(&actions[0], ScheduleAction::Set(reminder) if reminder.note_id == "note-2"));
    assert_eq!(actions[1], ScheduleAction::Remove { note_id: "note-1".into() });
  }

  #[test]
  fn moving_the_fire_time_reschedules_the_same_note() {
    let now = ymdhm_to_date_ms(2026, 8, 8, 9, 0);
    let notes = [note("note-1", now + 2 * MS_PER_HOUR, Some(0))];
    let desired = desired_reminders(&notes, now, MAX_SCHEDULED, titles);
    let current = vec![ScheduledRecord {
      note_id: "note-1".into(),
      fire_at_local_ms: now + MS_PER_HOUR,
      payload_hash: payload_hash("note-1 title", "due"),
    }];

    assert_eq!(diff(&desired, &current).len(), 1);
  }

  #[test]
  fn retitling_a_note_reschedules_it_but_an_unrelated_edit_does_not() {
    let now = ymdhm_to_date_ms(2026, 8, 8, 9, 0);
    let mut notes = [note("note-1", now + MS_PER_HOUR, Some(0))];
    let current: Vec<ScheduledRecord> = desired_reminders(&notes, now, MAX_SCHEDULED, titles).iter().map(record).collect();

    notes[0].content = "an edit that never reaches the notification".into();
    assert!(diff(&desired_reminders(&notes, now, MAX_SCHEDULED, titles), &current).is_empty());

    notes[0].title = "renamed".into();
    assert_eq!(diff(&desired_reminders(&notes, now, MAX_SCHEDULED, titles), &current).len(), 1);
  }

  #[test]
  fn hiding_the_title_reschedules_every_reminder() {
    let now = ymdhm_to_date_ms(2026, 8, 8, 9, 0);
    let notes = [note("note-1", now + MS_PER_HOUR, Some(0))];
    let current: Vec<ScheduledRecord> = desired_reminders(&notes, now, MAX_SCHEDULED, titles).iter().map(record).collect();

    let hidden = desired_reminders(&notes, now, MAX_SCHEDULED, |_, _| ("You have a reminder".to_string(), "due".to_string()));

    assert_eq!(diff(&hidden, &current).len(), 1);
  }

  #[test]
  fn reminders_are_ordered_by_fire_time_then_note_id() {
    let now = ymdhm_to_date_ms(2026, 8, 8, 9, 0);
    let notes = [
      note("note-b", now + MS_PER_HOUR, Some(0)),
      note("note-a", now + MS_PER_HOUR, Some(0)),
      note("note-c", now + MS_PER_MINUTE, Some(0)),
    ];

    let ids: Vec<String> = desired_reminders(&notes, now, MAX_SCHEDULED, titles)
      .into_iter()
      .map(|reminder| reminder.note_id)
      .collect();

    assert_eq!(ids, ["note-c", "note-a", "note-b"]);
  }
}
