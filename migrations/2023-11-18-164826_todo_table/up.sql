
CREATE TYPE priority_type AS ENUM (
    'low',
    'medium',
    'high'
);

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

CREATE TABLE IF NOT EXISTS todos_list (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id UUID NOT NULL,
    FOREIGN KEY(user_id)
        REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS todos (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    list_id UUID NOT NULL,
    FOREIGN KEY(list_id)
        REFERENCES todos_list(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    priority priority_type NOT NULL DEFAULT 'medium',
    active bool NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS creatures (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    creator_id UUID NOT NULL,
    creature_name VARCHAR(128) NOT NULL,
    found_in locales[] NOT NULL,
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
    recovery_rolls INT NOT NULL DEFAULT 2,
    slug VARCHAR(128) NOT NULL UNIQUE,
    image_url VARCHAR(512),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
