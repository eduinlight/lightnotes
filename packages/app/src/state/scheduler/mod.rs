#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
  pub title: String,
  pub body: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SchedulerSupport {
  pub background: bool,
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
mod desktop;
#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
pub use desktop::{apply, clear_all, notify_now, support};

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
mod unsupported;
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub use unsupported::{apply, clear_all, notify_now, support};
