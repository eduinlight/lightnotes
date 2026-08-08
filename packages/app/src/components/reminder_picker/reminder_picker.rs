use super::use_reminder_picker::use_reminder_picker;
use crate::components::{ResponsivePopoverContent, ResponsivePopoverRoot, ResponsivePopoverTrigger};
use crate::state::{date_math, i18n, Note, REMIND_CHOICES};
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_icons::lucide::{Bell, BellRing, ChevronDown};
use ui::components::popover::ContentAlign;

fn remind_label(hours: Option<i64>) -> String {
  match hours {
    None => t!("reminder-remind-me"),
    Some(0) => t!("reminder-at-the-time"),
    Some(h) if h < 24 => t!("reminder-short-hours", count: h),
    Some(h) if h % 168 == 0 => t!("reminder-short-weeks", count: h / 168),
    Some(h) => t!("reminder-short-days", count: h / 24),
  }
}

fn choice_label(hours: Option<i64>) -> String {
  match hours {
    None => t!("reminder-none"),
    Some(0) => t!("reminder-at-the-time"),
    Some(h) if h < 24 => t!("reminder-hours-before", count: h),
    Some(h) if h % 168 == 0 => t!("reminder-weeks-before", count: h / 168),
    Some(h) => t!("reminder-days-before", count: h / 24),
  }
}

#[derive(PartialEq, Clone, Props)]
pub struct ReminderPickerProps {
  pub note: Note,
}

#[component]
pub fn ReminderPicker(props: ReminderPickerProps) -> Element {
  let ReminderPickerProps { note } = props;
  let mut picker = use_reminder_picker();
  let note_id = note.id.clone();
  let active = note.remind_before_hours.is_some();

  rsx! {
      ResponsivePopoverRoot {
          open: (picker.open)(),
          on_open_change: move |value| picker.open.set(value),
          ResponsivePopoverTrigger {
              class: if active {
                  "flex h-8 items-center gap-1.5 rounded-md border border-[var(--primary-color-6)] px-2.5 text-xs text-[var(--accent)]"
              } else {
                  "flex h-8 items-center gap-1.5 rounded-md border border-[var(--primary-color-6)] px-2.5 text-xs text-[var(--secondary-color)]"
              },
              title: t!("reminder-title"),
              if active {
                  BellRing { size: "14px", fill: "var(--accent)", stroke: "var(--accent)" }
              } else {
                  Bell { size: "14px" }
              }
              "{remind_label(note.remind_before_hours)}"
              ChevronDown { size: "11px" }
          }
          ResponsivePopoverContent {
              title: t!("reminder-remind-me"),
              align: ContentAlign::Start,
              class: "w-52 items-stretch gap-1 p-1.5 text-left",
              for hours in REMIND_CHOICES {
                  {
                      let note_id = note_id.clone();
                      let label = choice_label(hours);
                      let is_active = note.remind_before_hours == hours;
                      let class = if is_active {
                          "cursor-pointer rounded-md px-2 py-1.5 text-sm bg-[color-mix(in_srgb,var(--accent)_15%,transparent)] text-[var(--accent)]"
                      } else {
                          "cursor-pointer rounded-md px-2 py-1.5 text-sm text-[var(--secondary-color)] hover:bg-[var(--primary-color-4)]"
                      };
                      rsx! {
                          div {
                              key: "{label}",
                              class,
                              onclick: move |_| picker.set_remind_before(&note_id, hours),
                              "{label}"
                          }
                      }
                  }
              }
              if let Some(hours) = note.remind_before_hours {
                  div { class: "mt-1 border-t border-[var(--primary-color-6)] px-2 pt-2 text-[11.5px] text-[color-mix(in_srgb,var(--secondary-color)_55%,transparent)]",
                      {t!("reminder-fires", when: i18n::format_absolute(note.date_ms - hours * date_math::MS_PER_HOUR))}
                  }
              }
          }
      }
  }
}
