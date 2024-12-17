-- Add down migration script here
DROP TABLE IF EXISTS users;

DROP INDEX IF EXISTS idx_users_username_is_active;

DROP INDEX IF EXISTS idx_users_email_is_active;

DROP INDEX IF EXISTS idx_users_username_email_is_active;

DROP INDEX IF EXISTS idx_users_username;

DROP INDEX IF EXISTS idx_users_email;

DROP INDEX IF EXISTS idx_users_is_active;
