use crate::state::{date_math, i18n, local_now_ms, use_boot, use_diary_ui, use_notes, CalendarViewMode, DiaryUiState, Folder, Note, NotesStore, Tag};
use crate::Route;
use dioxus::prelude::*;
use dioxus_i18n::t;
use ui::components::sidebar::use_is_mobile;

fn snippet(markdown: &str) -> String {
  let plain: String = markdown
    .lines()
    .map(|line| line.trim_start_matches(['#', '-', '*', '>', ' ', '\t']))
    .collect::<Vec<_>>()
    .join(" ");
  plain.chars().take(120).collect()
}

pub struct DiaryEntryRow {
  pub id: String,
  pub title: String,
  pub snippet: String,
  pub time_label: String,
  pub folder_name: String,
  pub has_reminder: bool,
  pub is_active: bool,
  pub show_day_header: bool,
  pub day_header_label: String,
}

#[derive(Clone, Copy)]
pub struct DiaryEntryListState {
  pub store: NotesStore,
  pub diary_ui: DiaryUiState,
  pub is_mobile: Signal<bool>,
  pub ready: bool,
}

impl DiaryEntryListState {
  pub fn period_label(&self) -> String {
    let cursor_ms = (self.diary_ui.cursor_date_ms)();
    let (year, month, day, _, _) = date_math::date_ms_to_ymdhm(cursor_ms);

    match (self.diary_ui.view_mode)() {
      CalendarViewMode::Month => format!("{} {year}", i18n::month_name(month)),
      CalendarViewMode::Week => {
        let cursor_days = date_math::day_key(cursor_ms);
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
      CalendarViewMode::Day => {
        let cursor_days = date_math::day_key(cursor_ms);
        let weekday = i18n::weekday_short_name(date_math::weekday_index(cursor_days));
        format!("{weekday}, {day} {} {year}", i18n::month_name(month))
      }
    }
  }

  pub fn filter_summary(&self) -> String {
    let folder_label = match (self.diary_ui.filter_folder)() {
      Some(folder_id) => self.store.folders().into_iter().find(|folder| folder.id == folder_id).map(|folder| folder.name).unwrap_or_default(),
      None => t!("diary-all-folders"),
    };

    match (self.diary_ui.filter_tag)() {
      Some(tag_id) => {
        let tag_name = self.store.tags().into_iter().find(|tag| tag.id == tag_id).map(|tag| tag.name).unwrap_or_default();
        format!("{folder_label} \u{b7} #{tag_name}")
      }
      None => folder_label,
    }
  }

  pub fn folders(&self) -> Vec<Folder> {
    self.store.folders()
  }

  pub fn tags(&self) -> Vec<Tag> {
    self.store.tags()
  }

  pub fn filter_folder(&self) -> Option<String> {
    (self.diary_ui.filter_folder)()
  }

  pub fn filter_tag(&self) -> Option<String> {
    (self.diary_ui.filter_tag)()
  }

  pub fn filter_open(&self) -> bool {
    (self.diary_ui.filter_open)()
  }

  pub fn set_filter_open(&mut self, open: bool) {
    self.diary_ui.filter_open.set(open);
  }

  pub fn set_filter_folder(&mut self, folder_id: Option<String>) {
    self.diary_ui.filter_folder.set(folder_id);
  }

  pub fn set_filter_tag(&mut self, tag_id: Option<String>) {
    self.diary_ui.filter_tag.set(tag_id);
  }

  pub fn clear_filters(&mut self) {
    self.diary_ui.filter_folder.set(None);
    self.diary_ui.filter_tag.set(None);
  }

  pub fn create_entry(&mut self) {
    let folder_id = (self.diary_ui.filter_folder)();
    let tag_ids = (self.diary_ui.filter_tag)().into_iter().collect();
    let note_id = self.store.create_diary_note((self.diary_ui.cursor_date_ms)(), folder_id, tag_ids);
    navigator().push(Route::DiaryEntry { note_id });
  }

  fn period_bounds_days(&self) -> (i64, i64) {
    let cursor_ms = (self.diary_ui.cursor_date_ms)();
    let cursor_days = date_math::day_key(cursor_ms);

    match (self.diary_ui.view_mode)() {
      CalendarViewMode::Day => (cursor_days, cursor_days),
      CalendarViewMode::Week => {
        let start = cursor_days - date_math::weekday_index(cursor_days) as i64;
        (start, start + 6)
      }
      CalendarViewMode::Month => {
        let (year, month, _, _, _) = date_math::date_ms_to_ymdhm(cursor_ms);
        let start = date_math::days_from_civil(year, month, 1);
        let days = date_math::days_in_month(year, month) as i64;
        (start, start + days - 1)
      }
    }
  }

  pub fn entries(&self, active_note_id: Option<&str>) -> Vec<DiaryEntryRow> {
    let (start_days, end_days) = self.period_bounds_days();
    let folder = (self.diary_ui.filter_folder)();
    let tag = (self.diary_ui.filter_tag)();
    let view_mode = (self.diary_ui.view_mode)();
    let folders = self.store.folders();
    let today_days = date_math::day_key(local_now_ms());

    let mut notes: Vec<Note> = self
      .store
      .all_notes()
      .into_iter()
      .filter(|note| {
        let day = date_math::day_key(note.date_ms);
        day >= start_days && day <= end_days
      })
      .filter(|note| match &folder {
        Some(id) => note.folder_id.as_deref() == Some(id.as_str()),
        None => true,
      })
      .filter(|note| match &tag {
        Some(id) => note.tag_ids.iter().any(|note_tag| note_tag == id),
        None => true,
      })
      .collect();

    notes.sort_by(|a, b| b.date_ms.cmp(&a.date_ms));

    let mut last_day: Option<i64> = None;

    notes
      .into_iter()
      .map(|note| {
        let day = date_math::day_key(note.date_ms);
        let show_day_header = view_mode != CalendarViewMode::Day && last_day != Some(day);
        last_day = Some(day);

        let (_, _, _, hour, minute) = date_math::date_ms_to_ymdhm(note.date_ms);
        let (_, month, day_of_month) = date_math::civil_from_days(day);
        let weekday = i18n::weekday_short_name(date_math::weekday_index(day));
        let month_short = i18n::month_short_name(month);
        let day_header_label = if day == today_days {
          format!("{} \u{b7} {weekday} {day_of_month} {month_short}", t!("diary-today"))
        } else {
          format!("{weekday} {day_of_month} {month_short}")
        };

        let folder_name = note
          .folder_id
          .as_ref()
          .and_then(|folder_id| folders.iter().find(|folder| &folder.id == folder_id))
          .map(|folder| folder.name.clone())
          .unwrap_or_default();

        let is_active = active_note_id == Some(note.id.as_str());

        DiaryEntryRow {
          id: note.id.clone(),
          title: if note.title.is_empty() { t!("notes-untitled-note") } else { note.title.clone() },
          snippet: snippet(&note.content),
          time_label: format!("{hour:02}:{minute:02}"),
          folder_name,
          has_reminder: note.remind_before_hours.is_some(),
          is_active,
          show_day_header,
          day_header_label,
        }
      })
      .collect()
  }

  pub fn open(&self, note_id: &str) {
    navigator().push(Route::DiaryEntry { note_id: note_id.to_string() });
  }
}

pub fn use_diary_entry_list() -> DiaryEntryListState {
  DiaryEntryListState {
    store: use_notes(),
    diary_ui: use_diary_ui(),
    is_mobile: use_is_mobile(),
    ready: (use_boot().store_ready)(),
  }
}
