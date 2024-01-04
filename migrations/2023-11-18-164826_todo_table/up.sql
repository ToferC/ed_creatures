
CREATE TYPE locales as ENUM (
    'jungle',
    'desert',
    'forest',
    'plains',
    'urban',
    'mountain',
    'cavern',
    'swamp',
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
    name VARCHAR(128) NOT NULL,
    found_in locales[] NOT NULL DEFAULT '{cavern}',
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

CREATE UNIQUE INDEX creatures__name_idx ON creatures(name);

CREATE INDEX creatures__locales_idx ON creatures(found_in);

CREATE TABLE IF NOT EXISTS attacks (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    creator_id UUID NOT NULL,
    FOREIGN KEY(creator_id)
        REFERENCES users(id) ON DELETE CASCADE,
    creature_id UUID NOT NULL,
    FOREIGN KEY(creature_id)
        REFERENCES creatures(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    action_step INT NOT NULL DEFAULT 9,
    effect_step INT NOT NULL DEFAULT 9,
    details TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TYPE action_types as ENUM (
    'simple',
    'standard',
    'move',
    'ritual'
);

CREATE TYPE action_targets as ENUM (
    'physical_defense',
    'mystic_defense',
    'social_defense',
    'other',
    'not_applicable'
);

CREATE TYPE resisted_bys as ENUM (
    'physical',
    'mystic',
    'other',
    'not_applicable'
);

CREATE TABLE IF NOT EXISTS powers (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    creator_id UUID NOT NULL,
    FOREIGN KEY(creator_id)
        REFERENCES users(id) ON DELETE CASCADE,
    creature_id UUID NOT NULL,
    FOREIGN KEY(creature_id)
        REFERENCES creatures(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    action_type action_types NOT NULL DEFAULT 'standard',
    target action_targets NOT NULL DEFAULT 'physical_defense',
    resisted_by resisted_bys NOT NULL DEFAULT 'physical',
    action_step INT NOT NULL DEFAULT 9,
    effect_step INT NOT NULL DEFAULT 9,
    details TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

