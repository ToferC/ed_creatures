-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS password_reset_token;
DROP TABLE IF EXISTS email_verification_code;
DROP INDEX IF EXISTS users__email_idx;
DROP TABLE IF EXISTS users;
