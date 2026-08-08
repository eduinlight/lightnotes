use super::{Notification, SchedulerSupport};
use crate::state::reminders::ScheduleAction;

pub fn notify_now(_notification: Notification) {}

pub async fn support() -> SchedulerSupport {
  SchedulerSupport { background: false }
}

pub async fn apply(_actions: Vec<ScheduleAction>) -> Vec<ScheduleAction> {
  Vec::new()
}

pub async fn clear_all() {}
