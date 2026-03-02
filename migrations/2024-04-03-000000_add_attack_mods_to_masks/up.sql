ALTER TABLE masks
    ADD COLUMN attack_action_mod INT NOT NULL DEFAULT 0,
    ADD COLUMN attack_effect_mod INT NOT NULL DEFAULT 0;
