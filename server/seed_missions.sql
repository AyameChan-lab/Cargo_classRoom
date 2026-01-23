DO $$
DECLARE
    v_chief_id INT;
BEGIN
    -- Get the first available brawler id to assign as chief
    SELECT id INTO v_chief_id FROM brawlers LIMIT 1;

    -- If no brawler exists (unlikely if you've logged in), creates one to ensure FK constraint isn't violated
    IF v_chief_id IS NULL THEN
        INSERT INTO brawlers (username, password, display_name) 
        VALUES ('auto_seeded_chief', '$argon2id$v=19$m=19456,t=2,p=1$7/R5b+3k6k+3k6k+3k6k+w$3k6k+3k6k+3k6k+3k6k+w', 'Auto Chief') 
        RETURNING id INTO v_chief_id;
    END IF;

    -- Insert 10 diverse missions
    INSERT INTO missions (name, description, status, chief_id, created_at, updated_at) VALUES
    ('Operation Crypto', 'Decrypt the enemy communications', 'Open', v_chief_id, NOW(), NOW()),
    ('Project Titan', 'Secure the heavy asset transport', 'InProgress', v_chief_id, NOW() - INTERVAL '1 day', NOW()),
    ('Nebula Scout', 'Map the unknown sector', 'Completed', v_chief_id, NOW() - INTERVAL '2 days', NOW()),
    ('Void Walker', 'Infiltrate the void zone', 'Failed', v_chief_id, NOW() - INTERVAL '3 days', NOW()),
    ('Solar Flare', 'Harness solar energy for the base', 'Open', v_chief_id, NOW() - INTERVAL '4 hours', NOW()),
    ('Iron Wall', 'Hold the defensive line against the wave', 'Open', v_chief_id, NOW() - INTERVAL '5 hours', NOW()),
    ('Shadow Step', 'Assassinate the high-value target', 'InProgress', v_chief_id, NOW() - INTERVAL '12 hours', NOW()),
    ('Blue Horizon', 'Naval support for the coastal team', 'Completed', v_chief_id, NOW() - INTERVAL '1 week', NOW()),
    ('Crimson Tide', 'Repel the invasion force', 'Open', v_chief_id, NOW() - INTERVAL '30 minutes', NOW()),
    ('Golden Eagle', 'Escort the ambassador to safety', 'Open', v_chief_id, NOW() - INTERVAL '1 hour', NOW());
END $$;
