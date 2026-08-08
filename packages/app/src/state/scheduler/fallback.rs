use super::notify::delivery_permission;
use super::{Permission, ScheduleAction, SchedulerSupport};

pub async fn support() -> SchedulerSupport {
  SchedulerSupport { background: false, permission: delivery_permission() }
}

pub async fn request_permission() -> Permission {
  delivery_permission()
}

pub async fn apply(_actions: Vec<ScheduleAction>) -> Vec<ScheduleAction> {
  Vec::new()
}

pub async fn clear_all() {}
