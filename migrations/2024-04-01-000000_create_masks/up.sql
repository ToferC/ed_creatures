CREATE TABLE IF NOT EXISTS masks (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    creator_id UUID NOT NULL,
    FOREIGN KEY(creator_id)
        REFERENCES users(id) ON DELETE CASCADE,
    creator_slug VARCHAR(256) NOT NULL,
    name VARCHAR(128) NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    -- stat deltas (added to creature base stats when mask is applied)
    circle_rank INT NOT NULL DEFAULT 0,
    dexterity INT NOT NULL DEFAULT 0,
    strength INT NOT NULL DEFAULT 0,
    constitution INT NOT NULL DEFAULT 0,
    perception INT NOT NULL DEFAULT 0,
    willpower INT NOT NULL DEFAULT 0,
    charisma INT NOT NULL DEFAULT 0,
    initiative INT NOT NULL DEFAULT 0,
    pd INT NOT NULL DEFAULT 0,
    md INT NOT NULL DEFAULT 0,
    sd INT NOT NULL DEFAULT 0,
    pa INT NOT NULL DEFAULT 0,
    ma INT NOT NULL DEFAULT 0,
    unconsciousness_rating INT NOT NULL DEFAULT 0,
    death_rating INT NOT NULL DEFAULT 0,
    wound INT NOT NULL DEFAULT 0,
    knockdown INT NOT NULL DEFAULT 0,
    actions INT NOT NULL DEFAULT 0,
    recovery_rolls INT NOT NULL DEFAULT 0,
    karma INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS mask_attacks (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    creator_id UUID NOT NULL,
    FOREIGN KEY(creator_id)
        REFERENCES users(id) ON DELETE CASCADE,
    mask_id UUID NOT NULL,
    FOREIGN KEY(mask_id)
        REFERENCES masks(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    action_step INT NOT NULL DEFAULT 9,
    effect_step INT NOT NULL DEFAULT 9,
    details TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS mask_powers (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    creator_id UUID NOT NULL,
    FOREIGN KEY(creator_id)
        REFERENCES users(id) ON DELETE CASCADE,
    mask_id UUID NOT NULL,
    FOREIGN KEY(mask_id)
        REFERENCES masks(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    action_type action_types NOT NULL DEFAULT 'Standard',
    target action_targets NOT NULL DEFAULT 'PhysicalDefense',
    resisted_by resisted_bys NOT NULL DEFAULT 'Physical',
    action_step INT NOT NULL DEFAULT 9,
    effect_step INT,
    details TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS mask_talents (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    creator_id UUID NOT NULL,
    FOREIGN KEY(creator_id)
        REFERENCES users(id) ON DELETE CASCADE,
    mask_id UUID NOT NULL,
    FOREIGN KEY(mask_id)
        REFERENCES masks(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    action_step INT NOT NULL DEFAULT 9,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS mask_maneuvers (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    creator_id UUID NOT NULL,
    FOREIGN KEY(creator_id)
        REFERENCES users(id) ON DELETE CASCADE,
    mask_id UUID NOT NULL,
    FOREIGN KEY(mask_id)
        REFERENCES masks(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    source VARCHAR(512) NOT NULL DEFAULT '',
    details TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
