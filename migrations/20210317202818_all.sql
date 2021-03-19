-- Add migration script here
CREATE TABLE emojis
(
    id              SERIAL      NOT NULL,
    name            VARCHAR(40) NOT NULL,
    utf8            VARCHAR(10) NOT NULL,
    PRIMARY KEY (id)
);


CREATE TABLE operations
(
    id              SERIAL       NOT NULL,
    emoji           INT         NOT NULL,
    description     TEXT,
    PRIMARY KEY (id),
    CONSTRAINT "fk_emoji" FOREIGN KEY (emoji)
        REFERENCES emojis (id)
        ON DELETE CASCADE
);

CREATE TABLE foods
(
    id              SERIAL      NOT NULL,
    name            VARCHAR(60) NOT NULL,
    emoji           INT         NOT NULL,
    description     TEXT,

    PRIMARY KEY (id),
    CONSTRAINT "fk_emoji" FOREIGN KEY (emoji)
        REFERENCES emojis (id)
        ON DELETE CASCADE
);

CREATE TABLE recipes
(
    id              SERIAL      NOT NULL,
    operation       INT         NOT NULL,
    outcome         INT         NOT NULL,
    components      INT[]       NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT "fk_operation" FOREIGN KEY (operation)
        REFERENCES operations (id)
        ON DELETE CASCADE,
    CONSTRAINT "fk_outcome" FOREIGN KEY (outcome)
        REFERENCES foods (id)
        ON DELETE CASCADE
);

CREATE TABLE players
(
    id              SERIAL      NOT NULL,
    discord_id      TEXT        NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE games
(
    id              UUID        NOT NULL,
    channel_id      TEXT        NOT NULL,
    players         INT[]       NOT NULL,
    player_inventories UUID[]    NOT NULL,
    inventory       INT[]       NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE player_inventories
(
    id              UUID        NOT NULL,
    player          INT         NOT NULL,
    game            UUID         NOT NULL,
    inventory       INT[]       NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT "fk_player" FOREIGN KEY (player)
        REFERENCES players (id)
        ON DELETE CASCADE,
    CONSTRAINT "fk_game" FOREIGN KEY (game)
        REFERENCES games (id)
        ON DELETE CASCADE
);

CREATE TABLE stories
(
    id              SERIAL      NOT NULL,
    title           TEXT        NOT NULL,
    description     TEXT        NOT NULL,
    story           TEXT        NOT NULL,
    items           INT[]       NOT NULL,
    recipes         INT[]       NOT NULL,
    PRIMARY KEY (id)
)