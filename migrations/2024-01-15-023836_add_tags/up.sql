-- Your SQL goes here

CREATE TYPE tags as ENUM (
    'creature',
    'spirit',
    'elemental',
    'horror',
    'dragon',
    'horror_construct',
    'adept',
    'npc',
    'other'
);

ALTER TABLE creatures
    ADD COLUMN tags tags[] NOT NULL DEFAULT '{creature}';