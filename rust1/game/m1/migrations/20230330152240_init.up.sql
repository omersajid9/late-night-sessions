-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS notes (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        title VARCHAR(255) NOT NULL UNIQUE,
        content TEXT NOT NULL,
        category VARCHAR(100),
        published BOOLEAN DEFAULT FALSE,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );

-- game:
--     - id
--     - code
--     - active
--     - capacity
--     - created_at
--     - updated_at
--     (code)
CREATE TABLE
    IF NOT EXISTS game (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        code VARCHAR(7) NOT NULL UNIQUE,
        active BOOLEAN NOT NULL,
        capacity INTEGER NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
        UNIQUE (code)
    );
CREATE INDEX game_code_idx ON game (code);

-- player:
--     - id
--     - username
--     - game_id (fk game.id)
--     - logged_in
--     - created_at
--     - updated_at
--     (id, username, game_id)
CREATE TABLE
    IF NOT EXISTS player (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        username TEXT NOT NULL,
        game_id UUID NOT NULL,
        logged_in BOOLEAN NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),

        CONSTRAINT fk_game_id
            FOREIGN KEY (game_id)
            REFERENCES game(id),
        UNIQUE (id, username, game_id)
    );

-- lobby:
--     - game_id
--     - user_ids
--     (game_id)
CREATE TABLE
    IF NOT EXISTS lobby (
        game_id UUID NOT NULL,
        user_ids TEXT NOT NULL,

        CONSTRAINT fk_game_id
            FOREIGN KEY (game_id)
            REFERENCES game(id),
        UNIQUE (game_id)
    );

-- ballplay:
--     - id
--     - game_id (fk game.id)
--     - player_id (fk player.id)
--     - runs
--     - ball_number
--     - created_at
CREATE TABLE
    IF NOT EXISTS ballplay (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        game_id UUID NOT NULL,
        player_id UUID NOT NULL,
        runs INTEGER CHECK ( runs IN (0, 1, 2, 3, 4, 5, 6)) NOT NULL DEFAULT (0),
        ball_number INTEGER NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
        CONSTRAINT fk_game_id
            FOREIGN KEY (game_id)
            REFERENCES game(id),
        CONSTRAINT fk_player_id
            FOREIGN KEY (player_id)
            REFERENCES player(id)
    );

-- inning:
--     - id
--     - number
--     - game_id (fk game.id)
--     - bat_player_id (fk player.id)
--     - ball_player_id (fk player.id)
--     - score
--     - current_ballplay_id (fk ballplay.id)
--     - complete
--     - created_at
--     - updated_at
--     (number, game_id, bat_player_id, ball_player_id)
CREATE TABLE
    IF NOT EXISTS inning (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        first_or_second INTEGER NOT NULL,
        game_id UUID NOT NULL,
        bat_player_id UUID NOT NULL,
        ball_player_id UUID NOT NULL,
        score INTEGER NOT NULL,
        current_ballplay_id UUID,
        complete BOOLEAN NOT NULL DEFAULT (FALSE),

        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),

        CONSTRAINT fk_game_id
            FOREIGN KEY (game_id)
            REFERENCES game(id),
        CONSTRAINT fk_bat_player_id
            FOREIGN KEY (bat_player_id)
            REFERENCES player(id),
        CONSTRAINT fk_ball_player_id
            FOREIGN KEY (ball_player_id)
            REFERENCES player(id),
        CONSTRAINT fk_current_ballplay_id
            FOREIGN KEY (current_ballplay_id)
            REFERENCES ballplay(id),
        UNIQUE (first_or_second, game_id, bat_player_id, ball_player_id)
    );

