use super::{Permission, SchedulerSupport};

pub async fn support() -> SchedulerSupport {
  SchedulerSupport { background: false, permission: Permission::Unsupported }
}

pub async fn request_permission() -> Permission {
  Permission::Unsupported
}

#[cfg(not(target_arch = "wasm32"))]
pub fn notify_now(_notification: super::Notification) {}

#[cfg(not(target_arch = "wasm32"))]
pub async fn apply(_actions: Vec<super::ScheduleAction>) -> Vec<super::ScheduleAction> {
  Vec::new()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn clear_all() {}
