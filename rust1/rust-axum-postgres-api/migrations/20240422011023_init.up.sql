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

CREATE TABLE
    IF NOT EXISTS users (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        username VARCHAR(255) NOT NULL UNIQUE,
        passcode VARCHAR(255) NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );

CREATE INDEX users_email_idx ON users (username);

CREATE TABLE
    IF NOT EXISTS subscriptions (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        user_id UUID NOT NULL,
        subscriber_id UUID NOT NULL,
        amount INTEGER NOT NULL,
        subscription_period TEXT CHECK (subscription_period IN ('day', 'week', 'month')) NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
        CONSTRAINT fk_subscriber
            FOREIGN KEY (subscriber_id)
            REFERENCES users(id),
        CONSTRAINT fk_user
            FOREIGN KEY (user_id)
            REFERENCES users(id),
        UNIQUE (user_id, subscriber_id)
    );

CREATE TABLE
    IF NOT EXISTS transactions (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        subscription_id UUID NOT NULL,
        amount INTEGER NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
        CONSTRAINT fk_subscription
            FOREIGN KEY (subscription_id)
            REFERENCES subscriptions(id)
    );

CREATE TABLE
    IF NOT EXISTS services (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4())
        user_id UUID NOT NULL,
        name TEXT NOT NULL,
        description TEXT NOT NULL,
        amount INTEGER NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );


-- Service log
-- Transaction log