-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS maneuvers;
DROP TABLE IF EXISTS powers;
DROP TYPE IF EXISTS resisted_bys;
DROP TYPE IF EXISTS action_targets;
DROP TYPE IF EXISTS action_types;
DROP TABLE IF EXISTS attacks;
DROP TABLE IF EXISTS creatures;
DROP TYPE IF EXISTS tags;
DROP TYPE IF EXISTS rarities;
DROP TYPE IF EXISTS locales;