CREATE TABLE IF NOT EXISTS talents (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    creator_id UUID NOT NULL,
    FOREIGN KEY(creator_id)
        REFERENCES users(id) ON DELETE CASCADE,
    creature_id UUID NOT NULL,
    FOREIGN KEY(creature_id)
        REFERENCES creatures(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    action_step INT NOT NULL DEFAULT 9,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);