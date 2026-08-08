use std::sync::Arc;

use api_sdk::ApiClient;
use dioxus::prelude::*;

use crate::state::scheduler::{self, SchedulerSupport};
use crate::state::{api_base_url, use_auth, use_boot, use_notes, AuthState, NotesStore, Theme};

#[derive(Clone, Copy)]
pub struct SettingsPanelState {
  pub store: NotesStore,
  pub auth: AuthState,
  pub ready: bool,
  pub scheduler: Signal<SchedulerSupport>,
}

impl SettingsPanelState {
  pub fn sign_out(&mut self) {
    let mut auth = self.auth;
    let tokens = auth.tokens.peek().clone();

    auth.sign_out();

    spawn(async move {
      let api = Arc::new(ApiClient::new(api_base_url()));
      api.set_tokens(tokens);
      let _ = api.logout().await;
    });
  }

  pub fn set_theme(&mut self, theme: Theme) {
    self.store.set_theme(theme);
  }

  pub fn set_accent(&mut self, accent: String) {
    self.store.set_accent(accent);
  }

  pub fn toggle_sync(&mut self) {
    self.store.toggle_sync();
  }

  pub fn set_reminders_enabled(&mut self, enabled: bool) {
    self.store.set_reminders_enabled(enabled);
  }

  pub fn set_reminder_titles_visible(&mut self, visible: bool) {
    self.store.set_reminder_titles_visible(visible);
  }

  pub fn request_notification_permission(&mut self) {
    let mut scheduler_support = self.scheduler;

    spawn(async move {
      let permission = scheduler::request_permission().await;
      scheduler_support.write().permission = permission;
    });
  }
}

pub fn use_settings_panel() -> SettingsPanelState {
  let mut scheduler_support = use_signal(SchedulerSupport::default);

  use_hook(move || {
    spawn(async move {
      scheduler_support.set(scheduler::support().await);
    });
  });

  SettingsPanelState {
    store: use_notes(),
    auth: use_auth(),
    ready: use_boot().ready(),
    scheduler: scheduler_support,
  }
}
