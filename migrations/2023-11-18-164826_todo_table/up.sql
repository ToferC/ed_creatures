
CREATE TYPE locales as ENUM (
    'jungle',
    'desert',
    'forest',
    'plains',
    'urban',
    'mountain',
    'cavern',
    'kaer',
    'any'
);

CREATE TYPE rarities as ENUM (
    'common',
    'uncommon',
    'rare',
    'unique'
);

CREATE TABLE IF NOT EXISTS creatures (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    creator_id UUID NOT NULL,
    FOREIGN KEY(creator_id)
        REFERENCES users(id) ON DELETE CASCADE,
    creature_name VARCHAR(128) NOT NULL,
    found_in locales NOT NULL DEFAULT 'cavern',
    rarity rarities NOT NULL DEFAULT 'common',
    circle_rank INT NOT NULL DEFAULT 1,
    dexterity INT NOT NULL DEFAULT 5,
    strength INT NOT NULL DEFAULT 5,
    constitution INT NOT NULL DEFAULT 5,
    perception INT NOT NULL DEFAULT 5,
    willpower INT NOT NULL DEFAULT 5,
    charisma INT NOT NULL DEFAULT 5,
    initiative INT NOT NULL DEFAULT 5,
    pd INT NOT NULL DEFAULT 9,
    md INT NOT NULL DEFAULT 5,
    sd INT NOT NULL DEFAULT 5,
    pa INT NOT NULL DEFAULT 2,
    ma INT NOT NULL DEFAULT 2,
    unconsciousness_rating INT NOT NULL DEFAULT 25,
    death_rating INT NOT NULL DEFAULT 32,
    wound INT NOT NULL DEFAULT 8,
    knockdown INT NOT NULL DEFAULT 5,
    actions INT NOT NULL DEFAULT 1,
    movement VARCHAR(128) NOT NULL DEFAULT '10',
    recovery_rolls INT NOT NULL DEFAULT 2,
    karma INT NOT NULL DEFAULT 0,
    slug VARCHAR(128) NOT NULL,
    image_url VARCHAR(512),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
