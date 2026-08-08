use dioxus::prelude::*;
use dioxus_i18n::prelude::{use_init_i18n, I18nConfig};
use dioxus_i18n::t;
use unic_langid::langid;

use super::language::Language;
use super::notes::NotesStore;

const EN_US: &str = include_str!("../../i18n/en-US.ftl");
const ES_ES: &str = include_str!("../../i18n/es-ES.ftl");

pub fn use_app_i18n(store: NotesStore) {
  let mut i18n = use_init_i18n(|| {
    I18nConfig::new(langid!("en-US"))
      .with_locale((langid!("en-US"), EN_US))
      .with_locale((langid!("es-ES"), ES_ES))
      .with_fallback(langid!("en-US"))
  });

  use_effect(move || {
    i18n.set_language(store.language().langid());
  });

  use_effect(move || {
    let code = store.language().code();
    spawn(async move {
      let _ = document::eval(&format!("document.documentElement.setAttribute('lang', '{code}');")).await;
    });
  });
}

pub fn format_absolute(ms: i64) -> String {
  let (_, month, day, hour24, minute) = super::date_math::date_ms_to_ymdhm(ms);
  let period = if hour24 < 12 { t!("time-am") } else { t!("time-pm") };
  let hour12 = match hour24 % 12 {
    0 => 12,
    hour => hour,
  };

  format!("{day} {} \u{b7} {hour12}:{minute:02} {period}", month_short_name(month))
}

pub fn month_name(month: u32) -> String {
  match month {
    1 => t!("month-1"),
    2 => t!("month-2"),
    3 => t!("month-3"),
    4 => t!("month-4"),
    5 => t!("month-5"),
    6 => t!("month-6"),
    7 => t!("month-7"),
    8 => t!("month-8"),
    9 => t!("month-9"),
    10 => t!("month-10"),
    11 => t!("month-11"),
    12 => t!("month-12"),
    _ => String::new(),
  }
}

pub fn month_short_name(month: u32) -> String {
  match month {
    1 => t!("month-short-1"),
    2 => t!("month-short-2"),
    3 => t!("month-short-3"),
    4 => t!("month-short-4"),
    5 => t!("month-short-5"),
    6 => t!("month-short-6"),
    7 => t!("month-short-7"),
    8 => t!("month-short-8"),
    9 => t!("month-short-9"),
    10 => t!("month-short-10"),
    11 => t!("month-short-11"),
    12 => t!("month-short-12"),
    _ => String::new(),
  }
}

pub fn weekday_short_name(index: u32) -> String {
  match index {
    0 => t!("weekday-short-0"),
    1 => t!("weekday-short-1"),
    2 => t!("weekday-short-2"),
    3 => t!("weekday-short-3"),
    4 => t!("weekday-short-4"),
    5 => t!("weekday-short-5"),
    6 => t!("weekday-short-6"),
    _ => String::new(),
  }
}

pub fn weekday_narrow_name(index: u32) -> String {
  match index {
    0 => t!("weekday-narrow-0"),
    1 => t!("weekday-narrow-1"),
    2 => t!("weekday-narrow-2"),
    3 => t!("weekday-narrow-3"),
    4 => t!("weekday-narrow-4"),
    5 => t!("weekday-narrow-5"),
    6 => t!("weekday-narrow-6"),
    _ => String::new(),
  }
}

pub fn language_label(language: Language) -> String {
  t!(language.label_key())
}

#[cfg(test)]
mod tests {
  use super::*;
  use dioxus_i18n::fluent::{FluentArgs, FluentBundle, FluentResource};
  use std::collections::BTreeSet;
  use unic_langid::LanguageIdentifier;

  fn message_ids(source: &str) -> BTreeSet<String> {
    source
      .lines()
      .filter(|line| !line.starts_with([' ', '#']) && !line.is_empty())
      .filter_map(|line| line.split_once(" ="))
      .map(|(id, _)| id.to_string())
      .collect()
  }

  #[test]
  fn locales_parse() {
    FluentResource::try_new(EN_US.to_string()).expect("en-US should parse");
    FluentResource::try_new(ES_ES.to_string()).expect("es-ES should parse");
  }

  #[test]
  fn locales_expose_the_same_messages() {
    assert_eq!(message_ids(EN_US), message_ids(ES_ES));
  }

  fn assert_every_message_formats(langid: LanguageIdentifier, source: &str) {
    let resource = FluentResource::try_new(source.to_string()).expect("locale should parse");
    let mut bundle = FluentBundle::new(vec![langid]);
    bundle.add_resource(resource).expect("locale should load");

    let mut args = FluentArgs::new();
    args.set("count", 3);
    args.set("query", "notes");
    args.set("time", "just now");
    args.set("when", "3 Jan · 9:00 am");

    for id in message_ids(source) {
      let message = bundle.get_message(&id).expect("message should exist");
      let pattern = message.value().expect("message should have a value");
      let mut errors = Vec::new();
      bundle.format_pattern(pattern, Some(&args), &mut errors);
      assert!(errors.is_empty(), "{id} failed to format: {errors:?}");
    }
  }

  #[test]
  fn every_message_formats_without_errors() {
    assert_every_message_formats(langid!("en-US"), EN_US);
    assert_every_message_formats(langid!("es-ES"), ES_ES);
  }
}
