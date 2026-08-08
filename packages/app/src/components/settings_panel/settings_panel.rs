use super::use_settings_panel::use_settings_panel;
use super::SettingsSession;
use crate::components::{LanguagePicker, SettingsSkeleton};
use crate::state::scheduler::Permission;
use crate::state::{SyncStatus, Theme, ACCENT_SWATCHES};
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_icons::lucide::{Bell, BellOff, Check, CloudCheck, CloudOff, HardDrive, Moon, Notebook, Sun, UserRound};
use ui::components::button::{Button, ButtonSize, ButtonVariant};
use ui::components::switch::Switch;

fn theme_card_class(active: bool) -> &'static str {
  if active {
    "flex flex-1 flex-col items-center gap-2 rounded-lg border border-[var(--accent)] bg-[color-mix(in_srgb,var(--accent)_15%,transparent)] p-4 text-[var(--accent)]"
  } else {
    "flex flex-1 flex-col items-center gap-2 rounded-lg border border-[var(--primary-color-6)] bg-transparent p-4 text-[var(--secondary-color)]"
  }
}

#[component]
pub fn SettingsPanel() -> Element {
  let mut settings = use_settings_panel();

  if !settings.ready {
    return rsx! {
        SettingsSkeleton {}
    };
  }

  let store = settings.store;
  let theme = store.theme();
  let accent = store.accent();
  let sync = store.sync();
  let note_count = store.note_count();

  let sync_icon_color = "var(--secondary-color-5)";
  let sync_label = match sync {
    SyncStatus::Synced => t!("sync-saved"),
    SyncStatus::Offline => t!("sync-offline"),
  };

  let reminders_enabled = store.reminders_enabled();
  let titles_visible = store.reminder_titles_visible();
  let scheduler = (settings.scheduler)();
  let background_delivery = scheduler.background;
  let permission = scheduler.permission;
  let permission_label = match permission {
    Permission::Granted => t!("settings-reminders-permission-granted"),
    Permission::Denied => t!("settings-reminders-permission-denied"),
    Permission::Unsupported => t!("settings-reminders-permission-unsupported"),
    Permission::Unknown => t!("settings-reminders-permission-unknown"),
  };

  let signed_in = settings.auth.is_signed_in();
  let account_name = settings.auth.display_name();
  let account_email = settings.auth.user().map(|user| user.email).unwrap_or_default();

  rsx! {
      div { class: "flex flex-col gap-5",
          SettingsSession { title: t!("settings-account"),
              div { class: "flex items-center gap-3 rounded-lg bg-[var(--primary-color-3)] p-3",
                  UserRound { size: "20px", stroke: "var(--secondary-color-5)" }
                  div { class: "flex-1",
                      if signed_in {
                          div { class: "text-sm font-medium text-[var(--secondary-color)]", "{account_name}" }
                          div { class: "text-xs text-[var(--secondary-color-5)]", "{account_email}" }
                      } else {
                          div { class: "text-sm font-medium text-[var(--secondary-color)]", {t!("auth-not-signed-in")} }
                          div { class: "text-xs text-[var(--secondary-color-5)]", {t!("settings-account-description")} }
                      }
                  }
                  if signed_in {
                      Button {
                          variant: ButtonVariant::Secondary,
                          size: ButtonSize::Sm,
                          class: "border border-[var(--primary-color-6)] bg-transparent hover:bg-[color-mix(in_srgb,var(--secondary-color)_5%,transparent)]",
                          onclick: move |_| settings.sign_out(),
                          {t!("action-sign-out")}
                      }
                  }
              }
          }
          SettingsSession { title: t!("settings-appearance"),
              div { class: "flex gap-2",
                  button {
                      class: theme_card_class(theme == Theme::Dark),
                      onclick: move |_| settings.set_theme(Theme::Dark),
                      Moon { size: "20px" }
                      span { class: "text-sm font-medium", {t!("settings-theme-dark")} }
                  }
                  button {
                      class: theme_card_class(theme == Theme::Light),
                      onclick: move |_| settings.set_theme(Theme::Light),
                      Sun { size: "20px" }
                      span { class: "text-sm font-medium", {t!("settings-theme-light")} }
                  }
              }
              div { class: "flex items-center gap-4 rounded-lg bg-[var(--primary-color-3)] p-3",
                  div { class: "flex-1",
                      div { class: "text-sm font-medium text-[var(--secondary-color)]", {t!("settings-accent")} }
                      div { class: "text-xs text-[var(--secondary-color-5)]", {t!("settings-accent-description")} }
                  }
                  div { class: "flex gap-2.5",
                      for hex in ACCENT_SWATCHES {
                          {
                              let is_active = accent.eq_ignore_ascii_case(hex);
                              let style = format!(
                                  "width:24px;height:24px;border-radius:9999px;cursor:pointer;background:{hex};box-shadow:{}",
                                  if is_active { format!("0 0 0 2px var(--primary-color-3), 0 0 0 4px {hex}") } else { "none".to_string() },
                              );
                              rsx! {
                                  span {
                                      key: "{hex}",
                                      style,
                                      onclick: move |_| settings.set_accent(hex.to_string()),
                                  }
                              }
                          }
                      }
                  }
              }
          }
          SettingsSession { title: t!("settings-language"),
              div { class: "flex items-center gap-4 rounded-lg bg-[var(--primary-color-3)] p-3",
                  div { class: "flex-1",
                      div { class: "text-sm font-medium text-[var(--secondary-color)]", {t!("settings-language")} }
                      div { class: "text-xs text-[var(--secondary-color-5)]", {t!("settings-language-description")} }
                  }
                  LanguagePicker {}
              }
          }
          SettingsSession { title: t!("settings-reminders"),
              div { class: "flex items-center gap-3 rounded-lg bg-[var(--primary-color-3)] p-3",
                  if reminders_enabled {
                      Bell { size: "20px", stroke: "var(--accent)" }
                  } else {
                      BellOff { size: "20px", stroke: "var(--secondary-color-5)" }
                  }
                  div { class: "flex-1",
                      div { class: "text-sm font-medium text-[var(--secondary-color)]", {t!("settings-reminders-enabled")} }
                      div { class: "text-xs text-[var(--secondary-color-5)]",
                          if background_delivery {
                              {t!("settings-reminders-background-active")}
                          } else {
                              {t!("settings-reminders-background-unavailable")}
                          }
                      }
                  }
                  Switch {
                      checked: reminders_enabled,
                      on_checked_change: move |value| settings.set_reminders_enabled(value),
                  }
              }
              div { class: "mt-2 flex items-center gap-3 rounded-lg bg-[var(--primary-color-3)] p-3",
                  div { class: "flex-1",
                      div { class: "text-sm font-medium text-[var(--secondary-color)]", {t!("settings-reminders-titles")} }
                      div { class: "text-xs text-[var(--secondary-color-5)]", {t!("settings-reminders-titles-description")} }
                  }
                  Switch {
                      checked: titles_visible,
                      on_checked_change: move |value| settings.set_reminder_titles_visible(value),
                  }
              }
              div { class: "mt-2 flex items-center gap-3 rounded-lg bg-[var(--primary-color-3)] p-3",
                  div { class: "flex-1",
                      div { class: "text-sm font-medium text-[var(--secondary-color)]", {t!("settings-reminders-permission")} }
                      div { class: "text-xs text-[var(--secondary-color-5)]", "{permission_label}" }
                  }
                  if permission != Permission::Granted && permission != Permission::Unsupported {
                      Button {
                          variant: ButtonVariant::Secondary,
                          size: ButtonSize::Sm,
                          class: "border border-[var(--primary-color-6)] bg-transparent hover:bg-[color-mix(in_srgb,var(--secondary-color)_5%,transparent)]",
                          onclick: move |_| settings.request_notification_permission(),
                          {t!("settings-reminders-permission-request")}
                      }
                  }
              }
          }
          SettingsSession { title: t!("settings-sync"),
              div { class: "flex items-center gap-3 rounded-lg bg-[var(--primary-color-3)] p-3",
                  if sync == SyncStatus::Synced {
                      CloudCheck { size: "20px", stroke: sync_icon_color }
                  } else {
                      CloudOff { size: "20px", stroke: sync_icon_color }
                  }
                  div { class: "flex-1",
                      div { class: "text-sm font-medium text-[var(--secondary-color)]", "{sync_label}" }
                      div { class: "text-xs text-[var(--secondary-color-5)]", {t!("settings-notes-stored", count: note_count as i64)} }
                  }
                  Button {
                      variant: ButtonVariant::Secondary,
                      size: ButtonSize::Sm,
                      class: "border border-[var(--primary-color-6)] bg-transparent hover:bg-[color-mix(in_srgb,var(--secondary-color)_5%,transparent)]",
                      onclick: move |_| settings.toggle_sync(),
                      if sync == SyncStatus::Offline { {t!("settings-go-online")} } else { {t!("settings-go-offline")} }
                  }
              }
              div { class: "mt-2 flex items-center gap-3 rounded-lg bg-[var(--primary-color-3)] p-3",
                  HardDrive { size: "20px", stroke: "var(--secondary-color-5)" }
                  div { class: "flex-1",
                      div { class: "text-sm font-medium text-[var(--secondary-color)]", {t!("settings-offline-storage")} }
                      div { class: "text-xs text-[var(--secondary-color-5)]", {t!("settings-offline-storage-description")} }
                  }
              }
          }
          SettingsSession { title: t!("settings-editor"),
              div { class: "flex flex-col gap-2 text-sm text-[var(--secondary-color)]",
                  div { class: "flex items-center gap-2",
                      Check { size: "14px", stroke: "var(--accent)" }
                      {t!("settings-editor-markdown")}
                  }
                  div { class: "flex items-center gap-2",
                      Check { size: "14px", stroke: "var(--accent)" }
                      {t!("settings-editor-folders-tags")}
                  }
                  div { class: "flex items-center gap-2",
                      Check { size: "14px", stroke: "var(--accent)" }
                      {t!("settings-editor-search")}
                  }
              }
          }
          SettingsSession { title: t!("settings-about"),
              div { class: "flex items-center gap-3 rounded-lg bg-[var(--primary-color-3)] p-3",
                  Notebook { size: "20px", stroke: "var(--accent)" }
                  div { class: "flex-1",
                      div { class: "text-sm font-medium text-[var(--secondary-color)]", {t!("app-name")} }
                      div { class: "text-xs text-[var(--secondary-color-5)]", {t!("settings-version")} }
                  }
              }
          }
      }
  }
}
