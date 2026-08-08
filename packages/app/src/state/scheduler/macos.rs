use std::cell::RefCell;
use std::ptr::NonNull;

use block2::RcBlock;
use dioxus::logger::tracing::warn;
use futures_channel::oneshot;
use objc2::rc::Retained;
use objc2::runtime::Bool;
use objc2_foundation::{NSArray, NSBundle, NSDateComponents, NSError, NSString};
use objc2_user_notifications::{
  UNAuthorizationOptions, UNAuthorizationStatus, UNCalendarNotificationTrigger, UNMutableNotificationContent, UNNotificationRequest,
  UNNotificationSettings, UNUserNotificationCenter,
};

use super::{Permission, ScheduleAction, ScheduledReminder, SchedulerSupport};
use crate::state::date_math::date_ms_to_ymdhm;

fn center() -> Option<Retained<UNUserNotificationCenter>> {
  NSBundle::mainBundle().bundleIdentifier()?;

  Some(UNUserNotificationCenter::currentNotificationCenter())
}

async fn authorization_status() -> Option<UNAuthorizationStatus> {
  let center = center()?;
  let (tx, rx) = oneshot::channel();
  let tx = RefCell::new(Some(tx));

  let handler = RcBlock::new(move |settings: NonNull<UNNotificationSettings>| {
    let status = unsafe { settings.as_ref() }.authorizationStatus();
    if let Some(tx) = tx.borrow_mut().take() {
      let _ = tx.send(status);
    }
  });

  center.getNotificationSettingsWithCompletionHandler(&handler);

  rx.await.ok()
}

fn permission_of(status: UNAuthorizationStatus) -> Permission {
  match status {
    UNAuthorizationStatus::Authorized | UNAuthorizationStatus::Provisional => Permission::Granted,
    UNAuthorizationStatus::Denied => Permission::Denied,
    _ => Permission::Unknown,
  }
}

pub async fn support() -> SchedulerSupport {
  let Some(status) = authorization_status().await else {
    warn!("background reminders unavailable: not running from an application bundle");
    return SchedulerSupport { background: false, permission: super::notify::delivery_permission() };
  };

  let permission = permission_of(status);

  SchedulerSupport { background: permission == Permission::Granted, permission }
}

pub async fn request_permission() -> Permission {
  let Some(center) = center() else {
    return Permission::Unsupported;
  };

  let (tx, rx) = oneshot::channel();
  let tx = RefCell::new(Some(tx));

  let handler = RcBlock::new(move |granted: Bool, error: *mut NSError| {
    if !error.is_null() {
      warn!("notification authorization failed: {:?}", unsafe { &*error });
    }
    if let Some(tx) = tx.borrow_mut().take() {
      let _ = tx.send(granted.as_bool());
    }
  });

  let options = UNAuthorizationOptions::Alert | UNAuthorizationOptions::Sound;
  center.requestAuthorizationWithOptions_completionHandler(options, &handler);

  match rx.await {
    Ok(true) => Permission::Granted,
    Ok(false) => Permission::Denied,
    Err(_) => Permission::Unknown,
  }
}

fn trigger_for(fire_at_local_ms: i64) -> Retained<UNCalendarNotificationTrigger> {
  let (year, month, day, hour, minute) = date_ms_to_ymdhm(fire_at_local_ms);
  let components = NSDateComponents::new();

  components.setYear(year as isize);
  components.setMonth(month as isize);
  components.setDay(day as isize);
  components.setHour(hour as isize);
  components.setMinute(minute as isize);
  components.setSecond(0);

  UNCalendarNotificationTrigger::triggerWithDateMatchingComponents_repeats(&components, false)
}

fn schedule(center: &UNUserNotificationCenter, reminder: &ScheduledReminder) {
  let content = UNMutableNotificationContent::new();
  content.setTitle(&NSString::from_str(&reminder.title));
  content.setBody(&NSString::from_str(&reminder.body));

  let request = UNNotificationRequest::requestWithIdentifier_content_trigger(
    &NSString::from_str(&reminder.note_id),
    &content,
    Some(&trigger_for(reminder.fire_at_local_ms)),
  );

  center.addNotificationRequest_withCompletionHandler(&request, None);
}

fn cancel(center: &UNUserNotificationCenter, note_id: &str) {
  let identifiers = NSArray::from_retained_slice(&[NSString::from_str(note_id)]);

  center.removePendingNotificationRequestsWithIdentifiers(&identifiers);
}

pub async fn apply(actions: Vec<ScheduleAction>) -> Vec<ScheduleAction> {
  let Some(center) = center() else {
    return Vec::new();
  };

  for action in &actions {
    match action {
      ScheduleAction::Set(reminder) => schedule(&center, reminder),
      ScheduleAction::Remove { note_id } => cancel(&center, note_id),
    }
  }

  actions
}

pub async fn clear_all() {
  let Some(center) = center() else {
    return;
  };

  center.removeAllPendingNotificationRequests();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn an_unbundled_process_never_reaches_the_notification_center() {
    assert!(center().is_none());
  }
}
