-- Add down migration script here

DROP TABLE IF EXISTS notes;

DROP TABLE IF EXISTS inning;

DROP TABLE IF EXISTS ballplay;

DROP TABLE IF EXISTS lobby;

DROP TABLE IF EXISTS player;

DROP TABLE IF EXISTS game;

DROP INDEX IF EXISTS game_code_idx;
