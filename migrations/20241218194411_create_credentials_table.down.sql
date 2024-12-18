-- Add down migration script here

-- Drop the indexes
DROP INDEX IF EXISTS idx_created_at;
DROP INDEX IF EXISTS idx_type;
DROP INDEX IF EXISTS idx_user_id;
DROP INDEX IF EXISTS idx_id;

-- Drop the table
DROP TABLE IF EXISTS credentials;