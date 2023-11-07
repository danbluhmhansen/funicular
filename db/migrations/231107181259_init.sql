CREATE TABLE game (
    id uuid NOT NULL DEFAULT gen_rand_uuid7() PRIMARY KEY,
    name text NOT NULL,
    slug text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED,
    created timestamp NOT NULL GENERATED ALWAYS AS (uuid7_time(id)) STORED,
    description text NOT NULL DEFAULT ''
);

CREATE UNIQUE INDEX game_slug ON game(slug);

COMMENT ON TABLE game IS 'Collection of rules for a specific game.';

CREATE TABLE skill (
    id uuid NOT NULL DEFAULT gen_rand_uuid7() PRIMARY KEY,
    game_id uuid NOT NULL REFERENCES game(id) ON DELETE CASCADE,
    name text NOT NULL,
    slug text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED,
    created timestamp NOT NULL GENERATED ALWAYS AS (uuid7_time(id)) STORED,
    description text NOT NULL DEFAULT ''
);

CREATE UNIQUE INDEX skill_id_game_id ON skill(id, game_id);

CREATE UNIQUE INDEX skill_game_id_slug ON skill(game_id, slug);

COMMENT ON TABLE skill IS 'Describes a specific attribute or skill of an entity like actors or items.';

CREATE TABLE trait (
    id uuid NOT NULL DEFAULT gen_rand_uuid7() PRIMARY KEY,
    game_id uuid NOT NULL REFERENCES game(id) ON DELETE CASCADE,
    name text NOT NULL,
    slug text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED,
    created timestamp NOT NULL GENERATED ALWAYS AS (uuid7_time(id)) STORED,
    description text NOT NULL DEFAULT ''
);

CREATE UNIQUE INDEX trait_id_game_id ON trait(id, game_id);

CREATE UNIQUE INDEX trait_game_id_slug ON trait(game_id, slug);

COMMENT ON TABLE trait IS 'Describes a specific trait or effect of an entity like actors or items.';

CREATE TABLE actor_kind (
    id uuid NOT NULL DEFAULT gen_rand_uuid7() PRIMARY KEY,
    game_id uuid NOT NULL REFERENCES game(id) ON DELETE CASCADE,
    name text NOT NULL,
    slug text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED,
    created timestamp NOT NULL GENERATED ALWAYS AS (uuid7_time(id)) STORED,
    description text NOT NULL DEFAULT ''
);

CREATE UNIQUE INDEX actor_kind_id_game_id ON actor_kind(id, game_id);

CREATE UNIQUE INDEX actor_kind_game_id_slug ON actor_kind(game_id, slug);

COMMENT ON TABLE actor_kind IS 'A kind of actor, like player or enemy.';

CREATE TABLE actor (
    id uuid NOT NULL DEFAULT gen_rand_uuid7() PRIMARY KEY,
    kind_id uuid NOT NULL REFERENCES actor_kind(id) ON DELETE CASCADE,
    name text NOT NULL,
    slug text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED,
    created timestamp NOT NULL GENERATED ALWAYS AS (uuid7_time(id)) STORED,
    description text NOT NULL DEFAULT ''
);

CREATE UNIQUE INDEX actor_id_kind_id ON actor(id, kind_id);

CREATE UNIQUE INDEX actor_kind_id_slug ON actor(kind_id, slug);

COMMENT ON TABLE actor IS 'An individual controlled by a Game Master or player.';

CREATE TABLE gear_kind (
    id uuid NOT NULL DEFAULT gen_rand_uuid7() PRIMARY KEY,
    game_id uuid NOT NULL REFERENCES game(id) ON DELETE CASCADE,
    name text NOT NULL,
    slug text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED,
    created timestamp NOT NULL GENERATED ALWAYS AS (uuid7_time(id)) STORED,
    description text NOT NULL DEFAULT ''
);

CREATE UNIQUE INDEX gear_kind_id_game_id ON gear_kind(id, game_id);

CREATE UNIQUE INDEX gear_kind_game_id_slug ON gear_kind(game_id, slug);

COMMENT ON TABLE gear_kind IS 'A kind of gear, like equipment or consumables.';

CREATE TABLE gear (
    id uuid NOT NULL DEFAULT gen_rand_uuid7() PRIMARY KEY,
    kind_id uuid NOT NULL REFERENCES gear_kind(id) ON DELETE CASCADE,
    name text NOT NULL,
    slug text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED,
    created timestamp NOT NULL GENERATED ALWAYS AS (uuid7_time(id)) STORED,
    description text NOT NULL DEFAULT ''
);

CREATE UNIQUE INDEX gear_id_kind_id ON gear(id, kind_id);

CREATE UNIQUE INDEX gear_kind_id_slug ON gear(kind_id, slug);

COMMENT ON TABLE gear IS 'Equipment or gear used by actors.';

CREATE TABLE actor_skill (
    kind_id uuid NOT NULL REFERENCES actor_kind(id) ON DELETE CASCADE,
    skill_id uuid NOT NULL REFERENCES skill(id) ON DELETE CASCADE,
    PRIMARY KEY (kind_id, skill_id)
);

COMMENT ON TABLE actor_skill IS 'Connection between actor kinds and skills.';

CREATE TABLE gear_skill (
    kind_id uuid NOT NULL REFERENCES gear_kind(id) ON DELETE CASCADE,
    skill_id uuid NOT NULL REFERENCES skill(id) ON DELETE CASCADE,
    PRIMARY KEY (kind_id, skill_id)
);

COMMENT ON TABLE gear_skill IS 'Connection between gear kinds and skills.';

CREATE TABLE actor_trait (
    actor_id uuid NOT NULL REFERENCES actor(id) ON DELETE CASCADE,
    trait_id uuid NOT NULL REFERENCES trait(id) ON DELETE CASCADE,
    amount integer NOT NULL DEFAULT 1,
    PRIMARY KEY (actor_id, trait_id)
);

COMMENT ON TABLE actor_trait IS 'Connection between actors and traits.';

CREATE TABLE gear_trait (
    gear_id uuid NOT NULL REFERENCES gear(id) ON DELETE CASCADE,
    trait_id uuid NOT NULL REFERENCES trait(id) ON DELETE CASCADE,
    amount integer NOT NULL DEFAULT 1,
    PRIMARY KEY (gear_id, trait_id)
);

COMMENT ON TABLE gear_trait IS 'Connection between gears and traits.';

CREATE TABLE rule (
    skill_id uuid NOT NULL REFERENCES skill(id) ON DELETE CASCADE,
    trait_id uuid NOT NULL REFERENCES trait(id) ON DELETE CASCADE,
    value numeric NOT NULL,
    PRIMARY KEY (skill_id, trait_id)
);

COMMENT ON TABLE rule IS 'Describes a numeric rule of a trait, which skill to affect and by what amount.';

CREATE TABLE actor_gear (
    actor_id uuid NOT NULL REFERENCES actor(id) ON DELETE CASCADE,
    gear_id uuid NOT NULL REFERENCES gear(id) ON DELETE CASCADE,
    amount integer NOT NULL DEFAULT 1,
    PRIMARY KEY (actor_id, gear_id)
);

COMMENT ON TABLE actor_gear IS 'Connection between actors and gears.';

CREATE VIEW actor_skill_agg AS
SELECT
    actor.id,
    actor.kind_id,
    actor.name,
    actor.slug,
    actor.created,
    actor.description,
    JSONB_OBJECT_AGG(skills.name, skills.value) AS skills
FROM
    actor
    JOIN LATERAL (
        SELECT
            actor.id,
            skill.name,
            SUM(rule.value * COALESCE(actor_trait.amount, 0)) AS value
        FROM
            actor
            JOIN actor_skill ON actor_skill.kind_id = actor.kind_id
            JOIN skill ON skill.id = actor_skill.skill_id
            JOIN rule ON rule.skill_id = actor_skill.skill_id
            JOIN actor_trait ON actor_trait.actor_id = actor.id
            AND actor_trait.trait_id = rule.trait_id
        GROUP BY
            actor.id,
            skill.id
    ) skills ON skills.id = actor.id
GROUP BY
    actor.id;
