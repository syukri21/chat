-- Add down migration script here
DROP INDEX IF EXISTS idx_sessions_session_id;

DROP INDEX IF EXISTS idx_sessions_user_id;

DROP TABLE IF EXISTS sessions;
