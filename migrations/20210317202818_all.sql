-- Add migration script here
CREATE TABLE emojis
(
    id              INT         NOT NULL,
    name            VARCHAR(40) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE operations
(
    id              INT         NOT NULL,
    emoji           INT         NOT NULL,
    description     TEXT,
    PRIMARY KEY (id),
    CONSTRAINT "fk_emoji" FOREIGN KEY (emoji)
        REFERENCES emojis (id)
        ON DELETE CASCADE
);

CREATE TABLE foods
(
    id              INT         NOT NULL,
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
    id              INT         NOT NULL,
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
)

