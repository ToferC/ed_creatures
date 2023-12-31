-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS creatures;
DROP TABLE IF EXISTS todos;
DROP TABLE IF EXISTS todos_list;
DROP TYPE IF EXISTS priority_type;
DROP TYPE IF EXISTS status_type;
DROP TYPE IF EXISTS rarities;
DROP TYPE IF EXISTS locales;