use crate::state::{date_math, i18n, local_now_ms, use_diary_ui, use_notes, CalendarViewMode, DiaryUiState, Note, NotesStore};
use dioxus::prelude::*;
use ui::components::sidebar::use_is_mobile;

pub struct DayCell {
  pub day_key: i64,
  pub label: String,
  pub dow_label: String,
  pub in_current_month: bool,
  pub is_selected: bool,
  pub is_today: bool,
  pub has_notes: bool,
  pub has_reminder: bool,
}

#[derive(Clone, Copy)]
pub struct DiaryCalendarState {
  pub store: NotesStore,
  pub diary_ui: DiaryUiState,
  pub is_mobile: Signal<bool>,
}

impl DiaryCalendarState {
  pub fn view_mode(&self) -> CalendarViewMode {
    (self.diary_ui.view_mode)()
  }

  pub fn cursor_date_ms(&self) -> i64 {
    (self.diary_ui.cursor_date_ms)()
  }

  pub fn set_view(&mut self, mode: CalendarViewMode) {
    self.diary_ui.view_mode.set(mode);
  }

  pub fn calendar_open(&self) -> bool {
    (self.diary_ui.calendar_open)()
  }

  pub fn set_calendar_open(&mut self, open: bool) {
    self.diary_ui.calendar_open.set(open);
  }

  pub fn filter_folder(&self) -> Option<String> {
    (self.diary_ui.filter_folder)()
  }

  pub fn filter_tag(&self) -> Option<String> {
    (self.diary_ui.filter_tag)()
  }

  pub fn filtered_notes(&self) -> Vec<Note> {
    let folder = self.filter_folder();
    let tag = self.filter_tag();

    self
      .store
      .all_notes()
      .into_iter()
      .filter(|note| match &folder {
        Some(id) => note.folder_id.as_deref() == Some(id.as_str()),
        None => true,
      })
      .filter(|note| match &tag {
        Some(id) => note.tag_ids.iter().any(|note_tag| note_tag == id),
        None => true,
      })
      .collect()
  }

  fn day_cell(&self, days: i64, in_current_month: bool, notes: &[Note], today_days: i64, cursor_days: i64) -> DayCell {
    let (_, _, day) = date_math::civil_from_days(days);
    DayCell {
      day_key: days,
      label: day.to_string(),
      dow_label: i18n::weekday_short_name(date_math::weekday_index(days)),
      in_current_month,
      is_selected: days == cursor_days,
      is_today: days == today_days,
      has_notes: notes.iter().any(|note| date_math::day_key(note.date_ms) == days),
      has_reminder: notes.iter().any(|note| date_math::day_key(note.date_ms) == days && note.remind_before_hours.is_some()),
    }
  }

  pub fn month_cells(&self) -> Vec<DayCell> {
    let (year, month, _, _, _) = date_math::date_ms_to_ymdhm(self.cursor_date_ms());
    let month_start_days = date_math::days_from_civil(year, month, 1);
    let lead = date_math::weekday_index(month_start_days) as i64;
    let grid_start_days = month_start_days - lead;
    let today_days = date_math::day_key(local_now_ms());
    let cursor_days = date_math::day_key(self.cursor_date_ms());
    let notes = self.filtered_notes();

    (0..42)
      .map(|offset| {
        let days = grid_start_days + offset;
        let (cell_year, cell_month, _) = date_math::civil_from_days(days);
        let in_current_month = cell_year == year && cell_month == month;
        self.day_cell(days, in_current_month, &notes, today_days, cursor_days)
      })
      .collect()
  }

  pub fn week_cells(&self) -> Vec<DayCell> {
    let cursor_days = date_math::day_key(self.cursor_date_ms());
    let week_start_days = cursor_days - date_math::weekday_index(cursor_days) as i64;
    let today_days = date_math::day_key(local_now_ms());
    let notes = self.filtered_notes();

    (0..7).map(|offset| self.day_cell(week_start_days + offset, true, &notes, today_days, cursor_days)).collect()
  }

  pub fn header_label(&self) -> String {
    let (year, month, day, _, _) = date_math::date_ms_to_ymdhm(self.cursor_date_ms());

    match self.view_mode() {
      CalendarViewMode::Month => format!("{} {year}", i18n::month_name(month)),
      CalendarViewMode::Week => {
        let cursor_days = date_math::day_key(self.cursor_date_ms());
        let week_start_days = cursor_days - date_math::weekday_index(cursor_days) as i64;
        let week_end_days = week_start_days + 6;
        let (_, start_month, start_day) = date_math::civil_from_days(week_start_days);
        let (end_year, end_month, end_day) = date_math::civil_from_days(week_end_days);
        format!(
          "{start_day} {} \u{2013} {end_day} {} {end_year}",
          i18n::month_short_name(start_month),
          i18n::month_short_name(end_month)
        )
      }
      CalendarViewMode::Day => format!("{day} {} {year}", i18n::month_name(month)),
    }
  }

  pub fn select_day(&mut self, day_key: i64) {
    self.diary_ui.cursor_date_ms.set(date_math::day_key_to_ms(day_key));
    self.diary_ui.view_mode.set(CalendarViewMode::Day);
    self.diary_ui.calendar_open.set(false);
  }

  pub fn step(&mut self, direction: i64) {
    let cursor = self.cursor_date_ms();
    let next_cursor = match self.view_mode() {
      CalendarViewMode::Day => date_math::add_days(cursor, direction),
      CalendarViewMode::Week => date_math::add_days(cursor, direction * 7),
      CalendarViewMode::Month => {
        let (year, month, day, hour, minute) = date_math::date_ms_to_ymdhm(cursor);
        let total_months = year as i64 * 12 + (month as i64 - 1) + direction;
        let next_year = total_months.div_euclid(12) as i32;
        let next_month = (total_months.rem_euclid(12) + 1) as u32;
        let clamped_day = day.min(date_math::days_in_month(next_year, next_month));
        date_math::ymdhm_to_date_ms(next_year, next_month, clamped_day, hour, minute)
      }
    };
    self.diary_ui.cursor_date_ms.set(next_cursor);
  }
}

pub fn use_diary_calendar() -> DiaryCalendarState {
  DiaryCalendarState { store: use_notes(), diary_ui: use_diary_ui(), is_mobile: use_is_mobile() }
}
