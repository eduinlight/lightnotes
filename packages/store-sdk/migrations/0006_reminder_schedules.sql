CREATE TABLE reminder_schedules (
  user_id TEXT NOT NULL,
  note_id TEXT NOT NULL,
  fire_at_ms INTEGER NOT NULL,
  payload_hash TEXT NOT NULL,
  scheduled_at_ms INTEGER NOT NULL,
  PRIMARY KEY (user_id, note_id)
);

CREATE INDEX idx_reminder_schedules_user_fire ON reminder_schedules (user_id, fire_at_ms);
