use dioxus::prelude::*;
use dioxus_i18n::t;
use serde::{Deserialize, Serialize};

use super::language::Language;

pub const ACCENT_SWATCHES: [&str; 6] = ["#9184d9", "#84a7d9", "#7db8a0", "#d99184", "#c9a24b", "#c58fd0"];

pub const REMIND_CHOICES: [Option<i64>; 10] =
  [None, Some(0), Some(1), Some(2), Some(3), Some(6), Some(12), Some(24), Some(48), Some(168)];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
  pub user_id: String,
  pub id: String,
  pub title: String,
  pub content: String,
  pub folder_id: Option<String>,
  pub tag_ids: Vec<String>,
  pub pinned: bool,
  pub starred: bool,
  pub updated_at_ms: i64,
  pub order: i64,
  #[serde(default = "local_now_ms")]
  pub date_ms: i64,
  #[serde(default)]
  pub remind_before_hours: Option<i64>,
}

pub fn now_ms() -> i64 {
  web_time::SystemTime::now()
    .duration_since(web_time::UNIX_EPOCH)
    .map(|duration| duration.as_millis() as i64)
    .unwrap_or(0)
}

pub fn local_now_ms() -> i64 {
  super::date_math::utc_ms_to_local_ms(now_ms())
}

pub fn format_relative_time(updated_at_ms: i64) -> String {
  let elapsed_ms = (now_ms() - updated_at_ms).max(0);
  let elapsed_minutes = elapsed_ms / 60_000;
  let elapsed_hours = elapsed_ms / 3_600_000;
  let elapsed_days = elapsed_ms / 86_400_000;

  if elapsed_minutes < 1 {
    t!("time-just-now")
  } else if elapsed_minutes < 60 {
    t!("time-minutes-ago", count: elapsed_minutes)
  } else if elapsed_hours < 24 {
    t!("time-hours-ago", count: elapsed_hours)
  } else if elapsed_days == 1 {
    t!("time-yesterday")
  } else if elapsed_days < 7 {
    t!("time-days-ago", count: elapsed_days)
  } else {
    t!("time-weeks-ago", count: (elapsed_days / 7).max(1))
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FolderIcon {
  Inbox,
  Briefcase,
  User,
  BookOpen,
  Notebook,
  Archive,
  House,
  Star,
  Heart,
  Settings,
  Calendar,
  Camera,
  Music,
  Code,
  Palette,
  Gift,
  Globe,
  Lock,
  Rocket,
  Bookmark,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Folder {
  pub user_id: String,
  pub id: String,
  pub name: String,
  pub icon: FolderIcon,
  #[serde(default = "now_ms")]
  pub updated_at_ms: i64,
  #[serde(default)]
  pub order: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
  pub user_id: String,
  pub id: String,
  pub name: String,
  #[serde(default = "now_ms")]
  pub updated_at_ms: i64,
  #[serde(default)]
  pub order: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum Theme {
  #[default]
  Dark,
  Light,
}

impl Theme {
  pub fn as_str(&self) -> &'static str {
    match self {
      Theme::Dark => "dark",
      Theme::Light => "light",
    }
  }

}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum SyncStatus {
  #[default]
  Synced,
  Offline,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NoteFilter {
  All,
  Starred,
  Pinned,
  Folder(String),
  Tag(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedState {
  pub notes: Vec<Note>,
  pub folders: Vec<Folder>,
  pub tags: Vec<Tag>,
  pub theme: Theme,
  #[serde(default = "default_accent")]
  pub accent: String,
  #[serde(default)]
  pub language: Language,
  pub sync: SyncStatus,
  pub next_id: u32,
}

fn default_accent() -> String {
  ACCENT_SWATCHES[0].to_string()
}

#[derive(Clone, Copy, PartialEq)]
pub struct NotesStore {
  user_id: Signal<String>,
  notes: Signal<Vec<Note>>,
  folders: Signal<Vec<Folder>>,
  tags: Signal<Vec<Tag>>,
  filter: Signal<NoteFilter>,
  search: Signal<String>,
  theme: Signal<Theme>,
  accent: Signal<String>,
  language: Signal<Language>,
  sync: Signal<SyncStatus>,
  next_id: Signal<u32>,
}

impl NotesStore {
  pub fn empty() -> Self {
    Self {
      user_id: Signal::new(String::new()),
      notes: Signal::new(Vec::new()),
      folders: Signal::new(Vec::new()),
      tags: Signal::new(Vec::new()),
      filter: Signal::new(NoteFilter::All),
      search: Signal::new(String::new()),
      theme: Signal::new(Theme::Dark),
      accent: Signal::new(ACCENT_SWATCHES[0].to_string()),
      language: Signal::new(Language::default()),
      sync: Signal::new(SyncStatus::Synced),
      next_id: Signal::new(1),
    }
  }

  pub fn user_id(&self) -> String {
    (self.user_id)()
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn peek_user_id(&self) -> String {
    self.user_id.peek().clone()
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn peek_notes(&self) -> Vec<Note> {
    let user_id = self.peek_user_id();
    self.notes.peek().iter().filter(|note| note.user_id == user_id).cloned().collect()
  }

  pub fn set_user(&mut self, user_id: String) {
    self.user_id.set(user_id);
  }

  fn owned_notes(&self) -> Vec<Note> {
    let user_id = self.user_id();
    (self.notes)().into_iter().filter(|note| note.user_id == user_id).collect()
  }

  fn owned_folders(&self) -> Vec<Folder> {
    let user_id = self.user_id();
    (self.folders)().into_iter().filter(|folder| folder.user_id == user_id).collect()
  }

  fn owned_tags(&self) -> Vec<Tag> {
    let user_id = self.user_id();
    (self.tags)().into_iter().filter(|tag| tag.user_id == user_id).collect()
  }

  fn next_id(&mut self) -> u32 {
    let id = (self.next_id)();
    self.next_id.set(id + 1);
    id
  }

  pub fn folders(&self) -> Vec<Folder> {
    let mut folders = self.owned_folders();
    folders.sort_by_key(|folder| folder.order);
    folders
  }

  pub fn tags(&self) -> Vec<Tag> {
    let mut tags = self.owned_tags();
    tags.sort_by_key(|tag| tag.order);
    tags
  }

  pub fn tag_name(&self, id: &str) -> Option<String> {
    self.owned_tags().into_iter().find(|tag| tag.id == id).map(|tag| tag.name)
  }

  pub fn filter(&self) -> NoteFilter {
    (self.filter)()
  }

  pub fn filter_title(&self) -> String {
    match self.filter() {
      NoteFilter::All => t!("filter-all-notes"),
      NoteFilter::Starred => t!("filter-starred"),
      NoteFilter::Pinned => t!("filter-pinned"),
      NoteFilter::Tag(tag_id) => self
        .tag_name(&tag_id)
        .map(|name| format!("#{name}"))
        .unwrap_or_default(),
      NoteFilter::Folder(folder_id) => self
        .folders()
        .into_iter()
        .find(|folder| folder.id == folder_id)
        .map(|folder| folder.name)
        .unwrap_or_default(),
    }
  }

  pub fn search(&self) -> String {
    (self.search)()
  }

  pub fn theme(&self) -> Theme {
    (self.theme)()
  }

  pub fn accent(&self) -> String {
    (self.accent)()
  }

  pub fn language(&self) -> Language {
    (self.language)()
  }

  pub fn sync(&self) -> SyncStatus {
    (self.sync)()
  }

  pub fn note(&self, id: &str) -> Option<Note> {
    self.owned_notes().into_iter().find(|note| note.id == id)
  }

  pub fn all_notes(&self) -> Vec<Note> {
    self.owned_notes()
  }

  pub fn note_count(&self) -> usize {
    self.owned_notes().len()
  }

  pub fn starred_count(&self) -> usize {
    self.owned_notes().iter().filter(|note| note.starred).count()
  }

  pub fn pinned_count(&self) -> usize {
    self.owned_notes().iter().filter(|note| note.pinned).count()
  }

  pub fn folder_note_count(&self, folder_id: &str) -> usize {
    self
      .owned_notes()
      .iter()
      .filter(|note| note.folder_id.as_deref() == Some(folder_id))
      .count()
  }

  pub fn tag_note_count(&self, tag_id: &str) -> usize {
    self
      .owned_notes()
      .iter()
      .filter(|note| note.tag_ids.iter().any(|id| id == tag_id))
      .count()
  }

  pub fn visible_notes(&self) -> Vec<Note> {
    let filter = self.filter();
    let query = self.search().to_lowercase();

    let mut notes: Vec<Note> = self
      .owned_notes()
      .into_iter()
      .filter(|note| match &filter {
        NoteFilter::All => true,
        NoteFilter::Starred => note.starred,
        NoteFilter::Pinned => note.pinned,
        NoteFilter::Folder(folder_id) => note.folder_id.as_deref() == Some(folder_id.as_str()),
        NoteFilter::Tag(tag_id) => note.tag_ids.iter().any(|id| id == tag_id),
      })
      .filter(|note| {
        query.is_empty()
          || note.title.to_lowercase().contains(&query)
          || note.content.to_lowercase().contains(&query)
      })
      .collect();

    notes.sort_by(|a, b| b.pinned.cmp(&a.pinned).then(b.order.cmp(&a.order)));
    notes
  }

  pub fn set_filter(&mut self, filter: NoteFilter) {
    self.filter.set(filter);
    self.search.set(String::new());
  }

  pub fn set_search(&mut self, query: String) {
    self.search.set(query);
  }

  pub fn set_theme(&mut self, theme: Theme) {
    self.theme.set(theme);
  }

  pub fn set_accent(&mut self, accent: String) {
    self.accent.set(accent);
  }

  pub fn set_language(&mut self, language: Language) {
    self.language.set(language);
  }

  pub fn toggle_sync(&mut self) {
    let next = match self.sync() {
      SyncStatus::Synced => SyncStatus::Offline,
      SyncStatus::Offline => SyncStatus::Synced,
    };
    self.sync.set(next);
  }

  fn with_note<R>(&mut self, id: &str, edit: impl FnOnce(&mut Note) -> R) -> Option<R> {
    let user_id = self.user_id();
    self
      .notes
      .write()
      .iter_mut()
      .find(|note| note.id == id && note.user_id == user_id)
      .map(edit)
  }

  fn insert_note(&mut self, folder_id: Option<String>, tag_ids: Vec<String>, date_ms: i64) -> String {
    let order = self.next_id() as i64;
    let id = format!("note-{order}");
    let user_id = self.user_id();

    self.notes.write().insert(
      0,
      Note {
        user_id,
        id: id.clone(),
        title: String::new(),
        content: String::new(),
        folder_id,
        tag_ids,
        pinned: false,
        starred: false,
        updated_at_ms: now_ms(),
        order,
        date_ms,
        remind_before_hours: None,
      },
    );

    id
  }

  pub fn create_note(&mut self) -> String {
    let folder_id = match self.filter() {
      NoteFilter::Folder(folder_id) => Some(folder_id),
      _ => None,
    };
    let tag_ids = match self.filter() {
      NoteFilter::Tag(tag_id) => vec![tag_id],
      _ => Vec::new(),
    };

    self.insert_note(folder_id, tag_ids, local_now_ms())
  }

  pub fn create_diary_note(&mut self, date_ms: i64, folder_id: Option<String>, tag_ids: Vec<String>) -> String {
    self.insert_note(folder_id, tag_ids, date_ms)
  }

  fn touch_note(&mut self, id: &str) {
    let order = self.next_id() as i64;
    self.with_note(id, |note| {
      note.updated_at_ms = now_ms();
      note.order = order;
    });
  }

  pub fn set_note_title(&mut self, id: &str, title: String) {
    self.with_note(id, |note| note.title = title);
    self.touch_note(id);
  }

  pub fn set_note_content(&mut self, id: &str, content: String) {
    self.with_note(id, |note| note.content = content);
    self.touch_note(id);
  }

  pub fn toggle_note_pin(&mut self, id: &str) {
    self.with_note(id, |note| note.pinned = !note.pinned);
  }

  pub fn toggle_note_star(&mut self, id: &str) {
    self.with_note(id, |note| note.starred = !note.starred);
  }

  pub fn set_note_folder(&mut self, id: &str, folder_id: Option<String>) {
    self.with_note(id, |note| note.folder_id = folder_id);
    self.touch_note(id);
  }

  pub fn set_note_date(&mut self, id: &str, date_ms: i64) {
    self.with_note(id, |note| note.date_ms = date_ms);
    self.touch_note(id);
  }

  pub fn set_note_remind_before(&mut self, id: &str, remind_before_hours: Option<i64>) {
    self.with_note(id, |note| note.remind_before_hours = remind_before_hours);
    self.touch_note(id);
  }

  pub fn add_note_tag(&mut self, id: &str, tag_id: String) {
    self.with_note(id, |note| {
      if !note.tag_ids.contains(&tag_id) {
        note.tag_ids.push(tag_id);
      }
    });
    self.touch_note(id);
  }

  pub fn remove_note_tag(&mut self, id: &str, tag_id: &str) {
    self.with_note(id, |note| note.tag_ids.retain(|existing| existing != tag_id));
    self.touch_note(id);
  }

  pub fn tag_id_for_name(&mut self, name: &str) -> String {
    let existing = self
      .owned_tags()
      .into_iter()
      .find(|tag| tag.name.eq_ignore_ascii_case(name))
      .map(|tag| tag.id);

    existing.unwrap_or_else(|| self.create_tag(name.to_string()))
  }

  pub fn delete_note(&mut self, id: &str) {
    let user_id = self.user_id();
    self.notes.write().retain(|note| note.id != id || note.user_id != user_id);
  }

  pub fn create_folder_with_icon(&mut self, name: String, icon: FolderIcon) -> String {
    let order = self.next_id() as i64;
    let id = format!("folder-{order}");
    let user_id = self.user_id();
    self
      .folders
      .write()
      .push(Folder { user_id, id: id.clone(), name, icon, updated_at_ms: now_ms(), order });
    id
  }

  pub fn rename_folder(&mut self, folder_id: &str, name: String) {
    let user_id = self.user_id();
    if let Some(folder) = self.folders.write().iter_mut().find(|folder| folder.id == folder_id && folder.user_id == user_id) {
      folder.name = name;
      folder.updated_at_ms = now_ms();
    }
  }

  pub fn set_folder_icon(&mut self, folder_id: &str, icon: FolderIcon) {
    let user_id = self.user_id();
    if let Some(folder) = self.folders.write().iter_mut().find(|folder| folder.id == folder_id && folder.user_id == user_id) {
      folder.icon = icon;
      folder.updated_at_ms = now_ms();
    }
  }

  pub fn delete_folder(&mut self, folder_id: &str) {
    let user_id = self.user_id();
    self.folders.write().retain(|folder| folder.id != folder_id || folder.user_id != user_id);
    for note in self.notes.write().iter_mut() {
      if note.user_id == user_id && note.folder_id.as_deref() == Some(folder_id) {
        note.folder_id = None;
      }
    }
    if self.filter() == NoteFilter::Folder(folder_id.to_string()) {
      self.set_filter(NoteFilter::All);
    }
  }

  pub fn create_tag(&mut self, name: String) -> String {
    let normalized = name.trim().to_lowercase().replace(' ', "-");
    if let Some(existing) = self.owned_tags().into_iter().find(|tag| tag.name == normalized) {
      return existing.id;
    }
    let order = self.next_id() as i64;
    let id = format!("tag-{order}");
    let user_id = self.user_id();
    self
      .tags
      .write()
      .push(Tag { user_id, id: id.clone(), name: normalized, updated_at_ms: now_ms(), order });
    id
  }

  pub fn delete_tag(&mut self, tag_id: &str) {
    let user_id = self.user_id();
    self.tags.write().retain(|tag| tag.id != tag_id || tag.user_id != user_id);
    for note in self.notes.write().iter_mut() {
      if note.user_id == user_id {
        note.tag_ids.retain(|id| id != tag_id);
      }
    }
    if self.filter() == NoteFilter::Tag(tag_id.to_string()) {
      self.set_filter(NoteFilter::All);
    }
  }

  pub fn snapshot(&self) -> PersistedState {
    PersistedState {
      notes: self.owned_notes(),
      folders: self.owned_folders(),
      tags: self.owned_tags(),
      theme: self.theme(),
      accent: self.accent(),
      language: self.language(),
      sync: self.sync(),
      next_id: (self.next_id)(),
    }
  }

  pub fn restore(&mut self, state: PersistedState) {
    self.notes.set(state.notes);
    self.folders.set(state.folders);
    self.tags.set(state.tags);
    self.theme.set(state.theme);
    self.accent.set(state.accent);
    self.language.set(state.language);
    self.sync.set(state.sync);
    self.next_id.set(state.next_id);
  }

  pub fn clear_synced_entities(&mut self) {
    self.user_id.set(String::new());
    self.notes.set(Vec::new());
    self.folders.set(Vec::new());
    self.tags.set(Vec::new());
    self.next_id.set(1);
  }
}
