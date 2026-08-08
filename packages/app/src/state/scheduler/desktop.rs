use std::sync::mpsc::{self, Sender};
use std::sync::OnceLock;

use dioxus::logger::tracing::warn;

use super::{Notification, SchedulerSupport};
use crate::state::reminders::ScheduleAction;

static SENDER: OnceLock<Option<Sender<Notification>>> = OnceLock::new();

fn sender() -> Option<&'static Sender<Notification>> {
  SENDER
    .get_or_init(|| {
      let (tx, rx) = mpsc::channel::<Notification>();
      let spawned = std::thread::Builder::new()
        .name("lightnotes-notifier".into())
        .spawn(move || {
          while let Ok(notification) = rx.recv() {
            if let Err(err) = notify_rust::Notification::new()
              .appname("LightNotes")
              .summary(&notification.title)
              .body(&notification.body)
              .show()
            {
              warn!("reminder notification was not delivered: {err}");
            }
          }
        })
        .is_ok();

      spawned.then_some(tx)
    })
    .as_ref()
}

pub fn notify_now(notification: Notification) {
  let Some(sender) = sender() else {
    warn!("reminder notification dropped: the notifier thread is unavailable");
    return;
  };

  if sender.send(notification).is_err() {
    warn!("reminder notification dropped: the notifier thread has stopped");
  }
}

pub async fn support() -> SchedulerSupport {
  SchedulerSupport { background: false }
}

pub async fn apply(_actions: Vec<ScheduleAction>) -> Vec<ScheduleAction> {
  Vec::new()
}

pub async fn clear_all() {}
