use super::notes::local_now_ms;
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CalendarViewMode {
  Day,
  Week,
  Month,
}

#[derive(Clone, Copy)]
pub struct DiaryUiState {
  pub view_mode: Signal<CalendarViewMode>,
  pub cursor_date_ms: Signal<i64>,
  pub calendar_open: Signal<bool>,
  pub filter_open: Signal<bool>,
  pub filter_folder: Signal<Option<String>>,
  pub filter_tag: Signal<Option<String>>,
}

impl DiaryUiState {
  pub fn seed() -> Self {
    Self {
      view_mode: Signal::new(CalendarViewMode::Month),
      cursor_date_ms: Signal::new(local_now_ms()),
      calendar_open: Signal::new(false),
      filter_open: Signal::new(false),
      filter_folder: Signal::new(None),
      filter_tag: Signal::new(None),
    }
  }
}

pub fn use_diary_ui() -> DiaryUiState {
  use_context::<DiaryUiState>()
}
