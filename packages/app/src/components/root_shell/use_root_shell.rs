use crate::boot::DISMISS_SPLASH_JS;
use crate::state::{date_math, use_app_i18n, use_persisted_session, use_synced_notes, AuthState, AuthStatus, BootState};
use crate::Route;
use dioxus::prelude::*;
use ui::components::sidebar::use_viewport_resolved;

const SPLASH_TIMEOUT_MS: u32 = 2500;

const TIMEZONE_OFFSET_JS: &str = "dioxus.send(new Date().getTimezoneOffset());";

#[derive(Clone, Copy)]
pub struct RootShellState {
  pub gated: bool,
}

pub fn use_root_shell() -> RootShellState {
  let boot = use_context_provider(BootState::seed);
  let auth = use_context_provider(AuthState::empty);
  use_local_timezone_offset();
  let store = use_synced_notes();
  use_persisted_session(auth);
  use_app_i18n(store);

  let resolved = use_viewport_resolved();
  let mut viewport_ready = boot.viewport_ready;
  use_effect(move || {
    if resolved() {
      viewport_ready.set(true);
    }
  });

  let mut timed_out = use_signal(|| false);
  use_hook(move || {
    spawn(async move {
      let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {SPLASH_TIMEOUT_MS}));"
      ))
      .await;
      viewport_ready.set(true);
      timed_out.set(true);
    });
  });

  let mut dismissed = use_signal(|| false);
  use_effect(move || {
    if dismissed() || !(boot.ready() || timed_out()) {
      return;
    }

    dismissed.set(true);
    spawn(async move {
      let _ = document::eval(DISMISS_SPLASH_JS).await;
    });
  });

  use_effect(move || {
    let theme_attr = store.theme().as_str();
    spawn(async move {
      let _ = document::eval(&format!(
        "document.documentElement.setAttribute('data-theme', '{theme_attr}');"
      ))
      .await;
    });
  });

  use_effect(move || {
    let accent = store.accent();
    spawn(async move {
      let eval = document::eval(
        "let accent = await dioxus.recv(); document.documentElement.style.setProperty('--accent', accent);",
      );
      let _ = eval.send(accent);
    });
  });

  let router = router();
  let mut intended = use_signal(|| None::<Route>);

  use_effect(move || {
    let status = auth.status();
    let route = router.current::<Route>();
    let on_login = matches!(route, Route::Login {});

    if status == AuthStatus::SignedOut && !on_login {
      intended.set(Some(route));
      navigator().replace(Route::Login {});
      return;
    }

    if status == AuthStatus::SignedIn && on_login {
      let target = intended.write().take().unwrap_or(Route::Notes {});
      navigator().replace(target);
    }
  });

  let on_login = matches!(use_route::<Route>(), Route::Login {});

  RootShellState {
    gated: auth.status() != AuthStatus::SignedIn && !on_login,
  }
}

fn use_local_timezone_offset() {
  use_hook(move || {
    spawn(async move {
      let mut eval = document::eval(TIMEZONE_OFFSET_JS);
      if let Ok(minutes) = eval.recv::<i64>().await {
        date_math::set_local_offset_ms(date_math::offset_ms_from_timezone_offset_minutes(minutes));
      }
    });
  });
}
