use std::path::Path;

use dioxus::logger::tracing::error;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{ConnectOptions, Row, SqlitePool};
use sync_dto::{ChangeOp, ChangePayload, EntityKind, FolderDto, FolderIconDto, NoteDto, QueuedChange, TagDto};

use crate::db_key::{raw_key_literal, DbKey};
use crate::plaintext_migration;

pub struct LocalStore {
  pool: SqlitePool,
}

pub enum ApplyOutcome {
  Applied,
  Skipped,
  AlreadyApplied,
}

#[derive(Debug, Clone, Default)]
pub struct LocalSnapshot {
  pub notes: Vec<NoteDto>,
  pub folders: Vec<FolderDto>,
  pub tags: Vec<TagDto>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReminderSchedule {
  pub note_id: String,
  pub fire_at_ms: i64,
  pub payload_hash: String,
  pub scheduled_at_ms: i64,
}

impl LocalStore {
  pub async fn try_connect(db_path: &Path, key: &DbKey) -> Option<Self> {
    if let Some(parent) = db_path.parent() {
      if let Err(err) = std::fs::create_dir_all(parent) {
        error!("local store unavailable: cannot create directory {}: {err}", parent.display());
        return None;
      }
    }

    if !plaintext_migration::migrate_if_plaintext(db_path, key).await {
      return None;
    }

    let options = SqliteConnectOptions::new()
      .filename(db_path)
      .create_if_missing(true)
      .disable_statement_logging()
      .pragma("key", raw_key_literal(key));
    let pool = match SqlitePoolOptions::new().connect_with(options).await {
      Ok(pool) => pool,
      Err(err) => {
        error!("local store unavailable: cannot open sqlite at {}: {err}", db_path.display());
        return None;
      }
    };

    if let Err(err) = sqlx::migrate!("./migrations").run(&pool).await {
      error!("local store unavailable: migrations failed at {}: {err}", db_path.display());
      return None;
    }

    Some(Self { pool })
  }

  pub async fn load_snapshot(&self, user_id: &str) -> LocalSnapshot {
    let note_rows = sqlx::query(
      "SELECT id, title, content, folder_id, tag_ids, pinned, starred, updated_at_ms, sort_order, date_ms, remind_before_hours \
       FROM notes WHERE user_id = ? ORDER BY sort_order DESC",
    )
      .bind(user_id)
      .fetch_all(&self.pool)
      .await
      .expect("failed to load notes");

    let notes = note_rows
      .into_iter()
      .map(|row| NoteDto {
        id: row.get("id"),
        title: row.get("title"),
        content: row.get("content"),
        folder_id: row.get("folder_id"),
        tag_ids: serde_json::from_str(row.get::<String, _>("tag_ids").as_str()).unwrap_or_default(),
        pinned: row.get("pinned"),
        starred: row.get("starred"),
        updated_at_ms: row.get("updated_at_ms"),
        order: row.get("sort_order"),
        date_ms: row.get("date_ms"),
        remind_before_hours: row.get("remind_before_hours"),
      })
      .collect();

    let folder_rows = sqlx::query("SELECT id, name, icon, updated_at_ms, sort_order FROM folders WHERE user_id = ? ORDER BY sort_order ASC")
      .bind(user_id)
      .fetch_all(&self.pool)
      .await
      .expect("failed to load folders");

    let folders = folder_rows
      .into_iter()
      .map(|row| FolderDto {
        id: row.get("id"),
        name: row.get("name"),
        icon: serde_json::from_str(row.get::<String, _>("icon").as_str()).unwrap_or(FolderIconDto::Inbox),
        updated_at_ms: row.get("updated_at_ms"),
        order: row.get("sort_order"),
      })
      .collect();

    let tag_rows = sqlx::query("SELECT id, name, updated_at_ms, sort_order FROM tags WHERE user_id = ? ORDER BY sort_order ASC")
      .bind(user_id)
      .fetch_all(&self.pool)
      .await
      .expect("failed to load tags");

    let tags = tag_rows
      .into_iter()
      .map(|row| TagDto {
        id: row.get("id"),
        name: row.get("name"),
        updated_at_ms: row.get("updated_at_ms"),
        order: row.get("sort_order"),
      })
      .collect();

    LocalSnapshot { notes, folders, tags }
  }

  pub async fn apply(&self, user_id: &str, change: &QueuedChange) -> ApplyOutcome {
    let already_applied = sqlx::query("SELECT 1 FROM applied_changes WHERE user_id = ? AND change_id = ?")
      .bind(user_id)
      .bind(&change.change_id)
      .fetch_optional(&self.pool)
      .await
      .expect("failed to query applied_changes")
      .is_some();

    if already_applied {
      return ApplyOutcome::AlreadyApplied;
    }

    let outcome = if change.op == ChangeOp::Delete {
      self.delete_entity(user_id, change.entity, &change.entity_id).await;
      ApplyOutcome::Applied
    } else {
      let existing_ms = self.existing_updated_at_ms(user_id, change.entity, &change.entity_id).await;
      if sync_dto::is_newer_or_equal(existing_ms, change.client_updated_at_ms) {
        self.upsert_payload(user_id, change.payload.as_ref()).await;
        ApplyOutcome::Applied
      } else {
        ApplyOutcome::Skipped
      }
    };

    sqlx::query("INSERT INTO applied_changes (user_id, change_id) VALUES (?, ?)")
      .bind(user_id)
      .bind(&change.change_id)
      .execute(&self.pool)
      .await
      .expect("failed to record applied change");

    outcome
  }

  async fn existing_updated_at_ms(&self, user_id: &str, entity: EntityKind, entity_id: &str) -> Option<i64> {
    let query = match entity {
      EntityKind::Note => "SELECT updated_at_ms FROM notes WHERE user_id = ? AND id = ?",
      EntityKind::Folder => "SELECT updated_at_ms FROM folders WHERE user_id = ? AND id = ?",
      EntityKind::Tag => "SELECT updated_at_ms FROM tags WHERE user_id = ? AND id = ?",
    };

    sqlx::query(query)
      .bind(user_id)
      .bind(entity_id)
      .fetch_optional(&self.pool)
      .await
      .expect("failed to query existing_updated_at_ms")
      .map(|row| row.get("updated_at_ms"))
  }

  async fn delete_entity(&self, user_id: &str, entity: EntityKind, entity_id: &str) {
    let query = match entity {
      EntityKind::Note => "DELETE FROM notes WHERE user_id = ? AND id = ?",
      EntityKind::Folder => "DELETE FROM folders WHERE user_id = ? AND id = ?",
      EntityKind::Tag => "DELETE FROM tags WHERE user_id = ? AND id = ?",
    };

    sqlx::query(query)
      .bind(user_id)
      .bind(entity_id)
      .execute(&self.pool)
      .await
      .expect("failed to delete entity");

    if entity == EntityKind::Note {
      self.delete_reminder_schedule(user_id, entity_id).await;
    }
  }

  async fn upsert_payload(&self, user_id: &str, payload: Option<&ChangePayload>) {
    match payload {
      Some(ChangePayload::Note(note)) => self.upsert_note(user_id, note).await,
      Some(ChangePayload::Folder(folder)) => self.upsert_folder(user_id, folder).await,
      Some(ChangePayload::Tag(tag)) => self.upsert_tag(user_id, tag).await,
      None => {}
    }
  }

  async fn upsert_note(&self, user_id: &str, note: &NoteDto) {
    let tag_ids = serde_json::to_string(&note.tag_ids).unwrap_or_else(|_| "[]".to_string());

    sqlx::query(
      "INSERT INTO notes (user_id, id, title, content, folder_id, tag_ids, pinned, starred, updated_at_ms, sort_order, date_ms, remind_before_hours) \
       VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) \
       ON CONFLICT(user_id, id) DO UPDATE SET title = excluded.title, content = excluded.content, folder_id = excluded.folder_id, \
       tag_ids = excluded.tag_ids, pinned = excluded.pinned, starred = excluded.starred, \
       updated_at_ms = excluded.updated_at_ms, sort_order = excluded.sort_order, \
       date_ms = excluded.date_ms, remind_before_hours = excluded.remind_before_hours",
    )
    .bind(user_id)
    .bind(&note.id)
    .bind(&note.title)
    .bind(&note.content)
    .bind(&note.folder_id)
    .bind(tag_ids)
    .bind(note.pinned)
    .bind(note.starred)
    .bind(note.updated_at_ms)
    .bind(note.order)
    .bind(note.date_ms)
    .bind(note.remind_before_hours)
    .execute(&self.pool)
    .await
    .expect("failed to upsert note");
  }

  async fn upsert_folder(&self, user_id: &str, folder: &FolderDto) {
    let icon = serde_json::to_string(&folder.icon).unwrap_or_else(|_| "\"Inbox\"".to_string());

    sqlx::query(
      "INSERT INTO folders (user_id, id, name, icon, updated_at_ms, sort_order) VALUES (?, ?, ?, ?, ?, ?) \
       ON CONFLICT(user_id, id) DO UPDATE SET name = excluded.name, icon = excluded.icon, updated_at_ms = excluded.updated_at_ms, sort_order = excluded.sort_order",
    )
    .bind(user_id)
    .bind(&folder.id)
    .bind(&folder.name)
    .bind(icon)
    .bind(folder.updated_at_ms)
    .bind(folder.order)
    .execute(&self.pool)
    .await
    .expect("failed to upsert folder");
  }

  async fn upsert_tag(&self, user_id: &str, tag: &TagDto) {
    sqlx::query(
      "INSERT INTO tags (user_id, id, name, updated_at_ms, sort_order) VALUES (?, ?, ?, ?, ?) \
       ON CONFLICT(user_id, id) DO UPDATE SET name = excluded.name, updated_at_ms = excluded.updated_at_ms, sort_order = excluded.sort_order",
    )
    .bind(user_id)
    .bind(&tag.id)
    .bind(&tag.name)
    .bind(tag.updated_at_ms)
    .bind(tag.order)
    .execute(&self.pool)
    .await
    .expect("failed to upsert tag");
  }

  pub async fn enqueue_outbound(&self, user_id: &str, change: &QueuedChange) {
    let change_json = serde_json::to_string(change).expect("failed to serialize queued change");

    sqlx::query("INSERT INTO outbound_queue (user_id, change_json, enqueued_at_ms) VALUES (?, ?, ?)")
      .bind(user_id)
      .bind(change_json)
      .bind(change.enqueued_at_ms)
      .execute(&self.pool)
      .await
      .expect("failed to enqueue outbound change");
  }

  pub async fn peek_front_outbound(&self, user_id: &str) -> Option<(i64, QueuedChange)> {
    let row = sqlx::query("SELECT id, change_json FROM outbound_queue WHERE user_id = ? ORDER BY id ASC LIMIT 1")
      .bind(user_id)
      .fetch_optional(&self.pool)
      .await
      .expect("failed to peek outbound queue")?;

    let id: i64 = row.get("id");
    let change_json: String = row.get("change_json");
    let change = serde_json::from_str(&change_json).expect("corrupt queued change json");

    Some((id, change))
  }

  pub async fn dequeue_front_outbound(&self, id: i64) {
    sqlx::query("DELETE FROM outbound_queue WHERE id = ?")
      .bind(id)
      .execute(&self.pool)
      .await
      .expect("failed to dequeue outbound change");
  }

  pub async fn cursor(&self, user_id: &str) -> i64 {
    sqlx::query("SELECT cursor FROM sync_cursor WHERE user_id = ?")
      .bind(user_id)
      .fetch_optional(&self.pool)
      .await
      .expect("failed to query sync cursor")
      .map(|row| row.get("cursor"))
      .unwrap_or(0)
  }

  pub async fn set_cursor(&self, user_id: &str, cursor: i64) {
    sqlx::query("INSERT INTO sync_cursor (user_id, cursor) VALUES (?, ?) ON CONFLICT(user_id) DO UPDATE SET cursor = excluded.cursor")
      .bind(user_id)
      .bind(cursor)
      .execute(&self.pool)
      .await
      .expect("failed to set sync cursor");
  }

  pub async fn device_id(&self) -> String {
    let candidate = uuid::Uuid::new_v4().to_string();

    let row = sqlx::query(
      "INSERT INTO device_identity (id, device_id) VALUES (0, ?) \
       ON CONFLICT(id) DO UPDATE SET device_id = device_identity.device_id \
       RETURNING device_id",
    )
    .bind(&candidate)
    .fetch_one(&self.pool)
    .await
    .expect("failed to get or create device_id");

    row.get("device_id")
  }

  pub async fn load_session(&self) -> Option<(String, String)> {
    sqlx::query("SELECT user_id, session_json FROM session WHERE id = 0")
      .fetch_optional(&self.pool)
      .await
      .expect("failed to query session")
      .map(|row| (row.get("user_id"), row.get("session_json")))
  }

  pub async fn save_session(&self, user_id: &str, session_json: &str, updated_at_ms: i64) {
    sqlx::query(
      "INSERT INTO session (id, user_id, session_json, updated_at_ms) VALUES (0, ?, ?, ?) \
       ON CONFLICT(id) DO UPDATE SET user_id = excluded.user_id, session_json = excluded.session_json, updated_at_ms = excluded.updated_at_ms",
    )
    .bind(user_id)
    .bind(session_json)
    .bind(updated_at_ms)
    .execute(&self.pool)
    .await
    .expect("failed to save session");
  }

  pub async fn clear_session(&self) {
    sqlx::query("DELETE FROM session WHERE id = 0")
      .execute(&self.pool)
      .await
      .expect("failed to clear session");
  }

  pub async fn load_reminder_schedules(&self, user_id: &str) -> Vec<ReminderSchedule> {
    sqlx::query(
      "SELECT note_id, fire_at_ms, payload_hash, scheduled_at_ms FROM reminder_schedules WHERE user_id = ? ORDER BY fire_at_ms ASC",
    )
    .bind(user_id)
    .fetch_all(&self.pool)
    .await
    .expect("failed to load reminder schedules")
    .into_iter()
    .map(|row| ReminderSchedule {
      note_id: row.get("note_id"),
      fire_at_ms: row.get("fire_at_ms"),
      payload_hash: row.get("payload_hash"),
      scheduled_at_ms: row.get("scheduled_at_ms"),
    })
    .collect()
  }

  pub async fn upsert_reminder_schedule(&self, user_id: &str, schedule: &ReminderSchedule) {
    sqlx::query(
      "INSERT INTO reminder_schedules (user_id, note_id, fire_at_ms, payload_hash, scheduled_at_ms) VALUES (?, ?, ?, ?, ?) \
       ON CONFLICT(user_id, note_id) DO UPDATE SET fire_at_ms = excluded.fire_at_ms, payload_hash = excluded.payload_hash, \
       scheduled_at_ms = excluded.scheduled_at_ms",
    )
    .bind(user_id)
    .bind(&schedule.note_id)
    .bind(schedule.fire_at_ms)
    .bind(&schedule.payload_hash)
    .bind(schedule.scheduled_at_ms)
    .execute(&self.pool)
    .await
    .expect("failed to upsert reminder schedule");
  }

  pub async fn delete_reminder_schedule(&self, user_id: &str, note_id: &str) {
    sqlx::query("DELETE FROM reminder_schedules WHERE user_id = ? AND note_id = ?")
      .bind(user_id)
      .bind(note_id)
      .execute(&self.pool)
      .await
      .expect("failed to delete reminder schedule");
  }

  pub async fn clear_reminder_schedules(&self, user_id: &str) {
    sqlx::query("DELETE FROM reminder_schedules WHERE user_id = ?")
      .bind(user_id)
      .execute(&self.pool)
      .await
      .expect("failed to clear reminder schedules");
  }

  pub async fn clear_user_data(&self, user_id: &str) {
    for statement in [
      "DELETE FROM notes WHERE user_id = ?",
      "DELETE FROM folders WHERE user_id = ?",
      "DELETE FROM tags WHERE user_id = ?",
      "DELETE FROM outbound_queue WHERE user_id = ?",
      "DELETE FROM applied_changes WHERE user_id = ?",
      "DELETE FROM sync_cursor WHERE user_id = ?",
      "DELETE FROM reminder_schedules WHERE user_id = ?",
    ] {
      sqlx::query(statement)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .expect("failed to clear local user data");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::path::PathBuf;
  use sync_dto::NoteDto;

  const TEST_KEY: DbKey = [0x42; 32];

  fn temp_db_path() -> PathBuf {
    std::env::temp_dir().join(format!("lightnotes-test-{}", uuid::Uuid::new_v4())).join("lightnotes.db")
  }

  fn note_change(id: &str, title: &str, updated_at_ms: i64) -> QueuedChange {
    QueuedChange {
      change_id: format!("change-{id}-{updated_at_ms}"),
      device_id: "device-test".into(),
      entity: EntityKind::Note,
      entity_id: id.into(),
      op: ChangeOp::Create,
      payload: Some(ChangePayload::Note(NoteDto {
        id: id.into(),
        title: title.into(),
        content: "body".into(),
        folder_id: None,
        tag_ids: Vec::new(),
        pinned: false,
        starred: false,
        updated_at_ms,
        order: 1,
        date_ms: updated_at_ms,
        remind_before_hours: None,
      })),
      client_updated_at_ms: updated_at_ms,
      enqueued_at_ms: updated_at_ms,
    }
  }

  fn schedule(note_id: &str, fire_at_ms: i64, payload_hash: &str) -> ReminderSchedule {
    ReminderSchedule {
      note_id: note_id.into(),
      fire_at_ms,
      payload_hash: payload_hash.into(),
      scheduled_at_ms: 1_000,
    }
  }

  #[tokio::test]
  async fn reminder_schedules_survive_reopening_the_database() {
    let dir = std::env::temp_dir().join(format!("lightnotes-test-{}", uuid::Uuid::new_v4()));
    let db_path = dir.join("lightnotes.db");

    let store = LocalStore::try_connect(&db_path, &TEST_KEY).await.expect("first connect");
    store.upsert_reminder_schedule("user-1", &schedule("note-1", 5_000, "hash-a")).await;
    drop(store);

    let reopened = LocalStore::try_connect(&db_path, &TEST_KEY).await.expect("second connect");

    assert_eq!(reopened.load_reminder_schedules("user-1").await, vec![schedule("note-1", 5_000, "hash-a")]);

    std::fs::remove_dir_all(&dir).ok();
  }

  #[tokio::test]
  async fn upserting_a_reminder_schedule_replaces_the_previous_row() {
    let dir = std::env::temp_dir().join(format!("lightnotes-test-{}", uuid::Uuid::new_v4()));
    let db_path = dir.join("lightnotes.db");

    let store = LocalStore::try_connect(&db_path, &TEST_KEY).await.expect("connect");
    store.upsert_reminder_schedule("user-1", &schedule("note-1", 5_000, "hash-a")).await;
    store.upsert_reminder_schedule("user-1", &schedule("note-1", 9_000, "hash-b")).await;

    let stored = store.load_reminder_schedules("user-1").await;

    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].fire_at_ms, 9_000);
    assert_eq!(stored[0].payload_hash, "hash-b");

    std::fs::remove_dir_all(&dir).ok();
  }

  #[tokio::test]
  async fn reminder_schedules_are_scoped_per_user() {
    let dir = std::env::temp_dir().join(format!("lightnotes-test-{}", uuid::Uuid::new_v4()));
    let db_path = dir.join("lightnotes.db");

    let store = LocalStore::try_connect(&db_path, &TEST_KEY).await.expect("connect");
    store.upsert_reminder_schedule("user-a", &schedule("note-1", 5_000, "hash-a")).await;
    store.upsert_reminder_schedule("user-b", &schedule("note-1", 7_000, "hash-b")).await;

    store.clear_reminder_schedules("user-a").await;

    assert!(store.load_reminder_schedules("user-a").await.is_empty());
    assert_eq!(store.load_reminder_schedules("user-b").await.len(), 1);

    std::fs::remove_dir_all(&dir).ok();
  }

  #[tokio::test]
  async fn deleting_a_note_drops_its_reminder_schedule() {
    let dir = std::env::temp_dir().join(format!("lightnotes-test-{}", uuid::Uuid::new_v4()));
    let db_path = dir.join("lightnotes.db");

    let store = LocalStore::try_connect(&db_path, &TEST_KEY).await.expect("connect");
    store.apply("user-1", &note_change("note-1", "doomed", 1_000)).await;
    store.upsert_reminder_schedule("user-1", &schedule("note-1", 5_000, "hash-a")).await;

    let mut deletion = note_change("note-1", "doomed", 2_000);
    deletion.change_id = "change-note-1-delete".into();
    deletion.op = ChangeOp::Delete;
    deletion.payload = None;
    store.apply("user-1", &deletion).await;

    assert!(store.load_snapshot("user-1").await.notes.is_empty());
    assert!(store.load_reminder_schedules("user-1").await.is_empty());

    std::fs::remove_dir_all(&dir).ok();
  }

  #[tokio::test]
  async fn applied_notes_survive_reopening_the_database() {
    let dir = std::env::temp_dir().join(format!("lightnotes-test-{}", uuid::Uuid::new_v4()));
    let db_path = dir.join("nested").join("lightnotes.db");

    let store = LocalStore::try_connect(&db_path, &TEST_KEY).await.expect("first connect");
    store.apply("user-1", &note_change("note-1", "offline note", 1_000)).await;
    assert_eq!(store.load_snapshot("user-1").await.notes.len(), 1);
    drop(store);

    let reopened = LocalStore::try_connect(&db_path, &TEST_KEY).await.expect("second connect");
    let snapshot = reopened.load_snapshot("user-1").await;

    assert_eq!(snapshot.notes.len(), 1);
    assert_eq!(snapshot.notes[0].title, "offline note");

    std::fs::remove_dir_all(&dir).ok();
  }

  #[tokio::test]
  async fn enqueued_changes_survive_reopening_the_database() {
    let dir = std::env::temp_dir().join(format!("lightnotes-test-{}", uuid::Uuid::new_v4()));
    let db_path = dir.join("lightnotes.db");

    let store = LocalStore::try_connect(&db_path, &TEST_KEY).await.expect("first connect");
    store.enqueue_outbound("user-1", &note_change("note-1", "offline note", 1_000)).await;
    drop(store);

    let reopened = LocalStore::try_connect(&db_path, &TEST_KEY).await.expect("second connect");
    let (row_id, change) = reopened.peek_front_outbound("user-1").await.expect("queued change survived");

    assert_eq!(change.entity_id, "note-1");

    reopened.dequeue_front_outbound(row_id).await;
    assert!(reopened.peek_front_outbound("user-1").await.is_none());

    std::fs::remove_dir_all(&dir).ok();
  }

  #[tokio::test]
  async fn clearing_user_data_keeps_the_device_identity_and_session() {
    let dir = std::env::temp_dir().join(format!("lightnotes-test-{}", uuid::Uuid::new_v4()));
    let db_path = dir.join("lightnotes.db");

    let store = LocalStore::try_connect(&db_path, &TEST_KEY).await.expect("connect");
    store.apply("user-1", &note_change("note-1", "mine", 1_000)).await;
    store.enqueue_outbound("user-1", &note_change("note-2", "queued", 2_000)).await;
    store.set_cursor("user-1", 42).await;
    store.save_session("user-1", "{}", 1_000).await;
    store.upsert_reminder_schedule("user-1", &schedule("note-1", 5_000, "hash-a")).await;
    let device = store.device_id().await;

    store.clear_user_data("user-1").await;

    assert!(store.load_snapshot("user-1").await.notes.is_empty());
    assert!(store.peek_front_outbound("user-1").await.is_none());
    assert!(store.load_reminder_schedules("user-1").await.is_empty());
    assert_eq!(store.cursor("user-1").await, 0);
    assert_eq!(store.device_id().await, device);
    assert_eq!(store.load_session().await.map(|(id, _)| id), Some("user-1".to_string()));

    store.clear_session().await;
    assert!(store.load_session().await.is_none());

    std::fs::remove_dir_all(&dir).ok();
  }

  #[tokio::test]
  async fn two_users_keep_separate_notes_under_the_same_entity_id() {
    let dir = std::env::temp_dir().join(format!("lightnotes-test-{}", uuid::Uuid::new_v4()));
    let db_path = dir.join("lightnotes.db");

    let store = LocalStore::try_connect(&db_path, &TEST_KEY).await.expect("connect");
    store.apply("user-a", &note_change("note-1", "belongs to a", 1_000)).await;
    store.apply("user-b", &note_change("note-1", "belongs to b", 1_000)).await;

    let a = store.load_snapshot("user-a").await;
    let b = store.load_snapshot("user-b").await;

    assert_eq!(a.notes.len(), 1);
    assert_eq!(a.notes[0].title, "belongs to a");
    assert_eq!(b.notes.len(), 1);
    assert_eq!(b.notes[0].title, "belongs to b");

    store.clear_user_data("user-a").await;

    assert!(store.load_snapshot("user-a").await.notes.is_empty());
    assert_eq!(store.load_snapshot("user-b").await.notes.len(), 1);

    std::fs::remove_dir_all(&dir).ok();
  }

  #[tokio::test]
  async fn queued_changes_are_only_visible_to_their_own_user() {
    let dir = std::env::temp_dir().join(format!("lightnotes-test-{}", uuid::Uuid::new_v4()));
    let db_path = dir.join("lightnotes.db");

    let store = LocalStore::try_connect(&db_path, &TEST_KEY).await.expect("connect");
    store.enqueue_outbound("user-a", &note_change("note-1", "a is offline", 1_000)).await;

    assert!(store.peek_front_outbound("user-b").await.is_none());

    let (_, change) = store.peek_front_outbound("user-a").await.expect("a still has its change");
    assert_eq!(change.entity_id, "note-1");

    std::fs::remove_dir_all(&dir).ok();
  }

  #[tokio::test]
  async fn sync_cursors_are_tracked_per_user() {
    let dir = std::env::temp_dir().join(format!("lightnotes-test-{}", uuid::Uuid::new_v4()));
    let db_path = dir.join("lightnotes.db");

    let store = LocalStore::try_connect(&db_path, &TEST_KEY).await.expect("connect");
    store.set_cursor("user-a", 42).await;
    store.set_cursor("user-b", 7).await;

    assert_eq!(store.cursor("user-a").await, 42);
    assert_eq!(store.cursor("user-b").await, 7);
    assert_eq!(store.cursor("user-c").await, 0);

    std::fs::remove_dir_all(&dir).ok();
  }

  #[tokio::test]
  async fn unwritable_path_returns_none_instead_of_panicking() {
    let db_path = Path::new("/dev/null/lightnotes/lightnotes.db");

    assert!(LocalStore::try_connect(db_path, &TEST_KEY).await.is_none());
  }

  #[tokio::test]
  async fn sqlcipher_is_actually_linked() {
    let db_path = temp_db_path();
    let store = LocalStore::try_connect(&db_path, &TEST_KEY).await.expect("connect");

    let version: Option<String> = sqlx::query_scalar("PRAGMA cipher_version")
      .fetch_optional(&store.pool)
      .await
      .expect("failed to query cipher_version");

    assert!(
      version.is_some_and(|version| !version.is_empty()),
      "sqlite is not linked against sqlcipher, so the local store is NOT encrypted"
    );

    std::fs::remove_dir_all(db_path.parent().expect("parent")).ok();
  }

  #[tokio::test]
  async fn the_database_file_is_not_readable_as_plaintext_sqlite() {
    let db_path = temp_db_path();
    let store = LocalStore::try_connect(&db_path, &TEST_KEY).await.expect("connect");
    store.apply("user-1", &note_change("note-1", "secret note", 1_000)).await;
    drop(store);

    let bytes = std::fs::read(&db_path).expect("read database");

    assert!(!bytes.starts_with(b"SQLite format 3\0"), "the database still has a plaintext sqlite header");
    assert!(
      !bytes.windows(11).any(|window| window == b"secret note"),
      "the note title is readable in the raw database file"
    );

    let wrong_key: DbKey = [0x17; 32];
    assert!(LocalStore::try_connect(&db_path, &wrong_key).await.is_none(), "the database opened with the wrong key");

    std::fs::remove_dir_all(db_path.parent().expect("parent")).ok();
  }

  #[tokio::test]
  async fn a_plaintext_database_is_migrated_without_losing_notes() {
    let db_path = temp_db_path();
    std::fs::create_dir_all(db_path.parent().expect("parent")).expect("create dir");

    let plaintext = SqlitePoolOptions::new()
      .connect_with(SqliteConnectOptions::new().filename(&db_path).create_if_missing(true).disable_statement_logging())
      .await
      .expect("plaintext connect");
    sqlx::migrate!("./migrations").run(&plaintext).await.expect("plaintext migrations");
    sqlx::query(
      "INSERT INTO notes (user_id, id, title, content, folder_id, tag_ids, pinned, starred, updated_at_ms, sort_order, date_ms, remind_before_hours) \
       VALUES ('user-1', 'note-1', 'never synced', 'body', NULL, '[]', 0, 0, 1000, 1, 1000, NULL)",
    )
    .execute(&plaintext)
    .await
    .expect("insert plaintext note");
    sqlx::raw_sql("PRAGMA user_version = 7").execute(&plaintext).await.expect("set user_version");
    plaintext.close().await;

    assert!(plaintext_migration::is_plaintext(&db_path), "the fixture is not a plaintext database");

    let store = LocalStore::try_connect(&db_path, &TEST_KEY).await.expect("connect after migration");
    let snapshot = store.load_snapshot("user-1").await;

    assert_eq!(snapshot.notes.len(), 1);
    assert_eq!(snapshot.notes[0].title, "never synced");

    let user_version: i64 = sqlx::query_scalar("PRAGMA user_version").fetch_one(&store.pool).await.expect("user_version");
    assert_eq!(user_version, 7);

    assert!(!plaintext_migration::is_plaintext(&db_path), "the database is still plaintext after migrating");

    std::fs::remove_dir_all(db_path.parent().expect("parent")).ok();
  }
}

