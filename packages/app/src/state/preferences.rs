use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use super::boot::use_boot;
use super::language::Language;
use super::notes::{default_reminder_titles_visible, default_reminders_enabled, NotesStore, Theme};

const PREFS_KEY: &str = "lightnotes:prefs:v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Prefs {
  theme: Theme,
  accent: String,
  #[serde(default)]
  language: Language,
  #[serde(default = "default_reminders_enabled")]
  reminders_enabled: bool,
  #[serde(default = "default_reminder_titles_visible")]
  reminder_titles_visible: bool,
}

pub fn use_persisted_preferences(mut store: NotesStore) {
  let mut loaded = use_boot().prefs_ready;

  use_effect(move || {
    spawn(async move {
      let mut eval = document::eval(&format!("dioxus.send(localStorage.getItem('{PREFS_KEY}'));"));
      if let Ok(Some(json)) = eval.recv::<Option<String>>().await {
        if let Ok(prefs) = serde_json::from_str::<Prefs>(&json) {
          store.set_theme(prefs.theme);
          store.set_accent(prefs.accent);
          store.set_language(prefs.language);
          store.set_reminders_enabled(prefs.reminders_enabled);
          store.set_reminder_titles_visible(prefs.reminder_titles_visible);
        }
      }
      loaded.set(true);
    });
  });

  use_effect(move || {
    let is_loaded = loaded();
    let prefs = Prefs {
      theme: store.theme(),
      accent: store.accent(),
      language: store.language(),
      reminders_enabled: store.reminders_enabled(),
      reminder_titles_visible: store.reminder_titles_visible(),
    };

    if !is_loaded {
      return;
    }

    spawn(async move {
      let Ok(json) = serde_json::to_string(&prefs) else {
        return;
      };
      let eval = document::eval(&format!(
        "let json = await dioxus.recv(); localStorage.setItem('{PREFS_KEY}', json);"
      ));
      let _ = eval.send(json);
    });
  });
}
