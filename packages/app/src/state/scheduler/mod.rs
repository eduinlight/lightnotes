#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
  pub title: String,
  pub body: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq)]
pub struct ScheduledReminder {
  pub note_id: String,
  pub fire_at_local_ms: i64,
  pub title: String,
  pub body: String,
  pub payload_hash: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq)]
pub enum ScheduleAction {
  Set(ScheduledReminder),
  Remove { note_id: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Permission {
  #[default]
  Unknown,
  Granted,
  Denied,
  Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SchedulerSupport {
  pub background: bool,
  pub permission: Permission,
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
mod notify;
#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
pub use notify::notify_now;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{apply, clear_all, request_permission, support};

#[cfg(any(target_os = "windows", target_os = "linux"))]
mod fallback;
#[cfg(any(target_os = "windows", target_os = "linux"))]
pub use fallback::{apply, clear_all, request_permission, support};

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
mod unsupported;
#[cfg(all(not(any(target_os = "macos", target_os = "windows", target_os = "linux")), not(target_arch = "wasm32")))]
pub use unsupported::{apply, clear_all, notify_now};
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub use unsupported::{request_permission, support};
