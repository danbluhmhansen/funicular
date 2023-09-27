CREATE TABLE "public"."game" (
    "id" uuid NOT NULL DEFAULT gen_rand_uuid7(),
    "name" text NOT NULL,
    "slug" text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED,
    "created" timestamp NULL GENERATED ALWAYS AS (uuid7_time(id)) STORED,
    "description" text NULL,
    PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX "game_slug_key" ON "public"."game" ("slug");

COMMENT ON TABLE "public"."game" IS 'Collection of rules for a specific game.';

CREATE TABLE "public"."actor_kind" (
    "id" uuid NOT NULL DEFAULT gen_rand_uuid7(),
    "game_id" uuid NOT NULL,
    "name" text NOT NULL,
    "slug" text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED,
    "created" timestamp NULL GENERATED ALWAYS AS (uuid7_time(id)) STORED,
    "description" text NULL,
    PRIMARY KEY ("id"),
    CONSTRAINT "actor_kind_game_id_fkey" FOREIGN KEY ("game_id") REFERENCES "public"."game" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE
);

CREATE UNIQUE INDEX "actor_kind_game_id_slug_key" ON "public"."actor_kind" ("game_id", "slug");

COMMENT ON TABLE "public"."actor_kind" IS 'A kind of actor, like player or enemy.';

CREATE TABLE "public"."actor" (
    "id" uuid NOT NULL DEFAULT gen_rand_uuid7(),
    "kind_id" uuid NOT NULL,
    "name" text NOT NULL,
    "slug" text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED,
    "created" timestamp NULL GENERATED ALWAYS AS (uuid7_time(id)) STORED,
    "description" text NULL,
    PRIMARY KEY ("id"),
    CONSTRAINT "actor_kind_id_fkey" FOREIGN KEY ("kind_id") REFERENCES "public"."actor_kind" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE
);

CREATE UNIQUE INDEX "actor_kind_id_slug_key" ON "public"."actor" ("kind_id", "slug");

COMMENT ON TABLE "public"."actor" IS 'An individual controlled by a Game Master or player.';

CREATE TABLE "public"."gear_kind" (
    "id" uuid NOT NULL DEFAULT gen_rand_uuid7(),
    "game_id" uuid NOT NULL,
    "name" text NOT NULL,
    "slug" text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED,
    "created" timestamp NULL GENERATED ALWAYS AS (uuid7_time(id)) STORED,
    "description" text NULL,
    PRIMARY KEY ("id"),
    CONSTRAINT "gear_kind_game_id_fkey" FOREIGN KEY ("game_id") REFERENCES "public"."game" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE
);

CREATE UNIQUE INDEX "gear_kind_game_id_slug_key" ON "public"."gear_kind" ("game_id", "slug");

COMMENT ON TABLE "public"."gear_kind" IS 'A kind of gear, like equipment or consumables.';

CREATE TABLE "public"."gear" (
    "id" uuid NOT NULL DEFAULT gen_rand_uuid7(),
    "kind_id" uuid NOT NULL,
    "name" text NOT NULL,
    "slug" text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED,
    "created" timestamp NULL GENERATED ALWAYS AS (uuid7_time(id)) STORED,
    "description" text NULL,
    PRIMARY KEY ("id"),
    CONSTRAINT "gear_kind_id_fkey" FOREIGN KEY ("kind_id") REFERENCES "public"."gear_kind" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE
);

CREATE UNIQUE INDEX "gear_kind_id_slug_key" ON "public"."gear" ("kind_id", "slug");

COMMENT ON TABLE "public"."gear" IS 'Equipment or gear used by actors.';

CREATE TABLE "public"."actor_gear" (
    "actor_id" uuid NOT NULL,
    "gear_id" uuid NOT NULL,
    "amount" integer NULL DEFAULT 1,
    PRIMARY KEY ("actor_id", "gear_id"),
    CONSTRAINT "actor_gear_actor_id_fkey" FOREIGN KEY ("actor_id") REFERENCES "public"."actor" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE,
        CONSTRAINT "actor_gear_gear_id_fkey" FOREIGN KEY ("gear_id") REFERENCES "public"."gear" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE
);

COMMENT ON TABLE "public"."actor_gear" IS 'Connection between actors and gears.';

CREATE TABLE "public"."skill" (
    "id" uuid NOT NULL DEFAULT gen_rand_uuid7(),
    "game_id" uuid NOT NULL,
    "name" text NOT NULL,
    "slug" text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED,
    "created" timestamp NULL GENERATED ALWAYS AS (uuid7_time(id)) STORED,
    "description" text NULL,
    PRIMARY KEY ("id"),
    CONSTRAINT "skill_game_id_fkey" FOREIGN KEY ("game_id") REFERENCES "public"."game" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE
);

CREATE UNIQUE INDEX "skill_game_id_slug_key" ON "public"."skill" ("game_id", "slug");

COMMENT ON TABLE "public"."skill" IS 'Describes a specific attribute or skill of an entity like actors or items.';

CREATE TABLE "public"."actor_skill" (
    "kind_id" uuid NOT NULL,
    "skill_id" uuid NOT NULL,
    PRIMARY KEY ("kind_id", "skill_id"),
    CONSTRAINT "actor_skill_kind_id_fkey" FOREIGN KEY ("kind_id") REFERENCES "public"."actor_kind" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE,
        CONSTRAINT "actor_skill_skill_id_fkey" FOREIGN KEY ("skill_id") REFERENCES "public"."skill" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE
);

COMMENT ON TABLE "public"."actor_skill" IS 'Connection between actor kinds and skills.';

CREATE TABLE "public"."trait" (
    "id" uuid NOT NULL DEFAULT gen_rand_uuid7(),
    "game_id" uuid NOT NULL,
    "name" text NOT NULL,
    "slug" text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED,
    "created" timestamp NULL GENERATED ALWAYS AS (uuid7_time(id)) STORED,
    "description" text NULL,
    PRIMARY KEY ("id"),
    CONSTRAINT "trait_game_id_fkey" FOREIGN KEY ("game_id") REFERENCES "public"."game" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE
);

CREATE UNIQUE INDEX "trait_game_id_slug_key" ON "public"."trait" ("game_id", "slug");

COMMENT ON TABLE "public"."trait" IS 'Describes a specific trait or effect of an entity like actors or items.';

CREATE TABLE "public"."actor_trait" (
    "actor_id" uuid NOT NULL,
    "trait_id" uuid NOT NULL,
    "amount" integer NULL DEFAULT 1,
    PRIMARY KEY ("actor_id", "trait_id"),
    CONSTRAINT "actor_trait_actor_id_fkey" FOREIGN KEY ("actor_id") REFERENCES "public"."actor" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE,
        CONSTRAINT "actor_trait_trait_id_fkey" FOREIGN KEY ("trait_id") REFERENCES "public"."trait" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE
);

COMMENT ON TABLE "public"."actor_trait" IS 'Connection between actors and traits.';

CREATE TABLE "public"."gear_skill" (
    "kind_id" uuid NOT NULL,
    "skill_id" uuid NOT NULL,
    PRIMARY KEY ("kind_id", "skill_id"),
    CONSTRAINT "gear_skill_kind_id_fkey" FOREIGN KEY ("kind_id") REFERENCES "public"."gear_kind" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE,
        CONSTRAINT "gear_skill_skill_id_fkey" FOREIGN KEY ("skill_id") REFERENCES "public"."skill" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE
);

COMMENT ON TABLE "public"."gear_skill" IS 'Connection between gear kinds and skills.';

CREATE TABLE "public"."gear_trait" (
    "gear_id" uuid NOT NULL,
    "trait_id" uuid NOT NULL,
    "amount" integer NULL DEFAULT 1,
    PRIMARY KEY ("gear_id", "trait_id"),
    CONSTRAINT "gear_trait_gear_id_fkey" FOREIGN KEY ("gear_id") REFERENCES "public"."gear" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE,
        CONSTRAINT "gear_trait_trait_id_fkey" FOREIGN KEY ("trait_id") REFERENCES "public"."trait" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE
);

COMMENT ON TABLE "public"."gear_trait" IS 'Connection between gears and traits.';

CREATE TABLE "public"."rule_num" (
    "skill_id" uuid NOT NULL,
    "trait_id" uuid NOT NULL,
    "value" numeric NOT NULL,
    PRIMARY KEY ("skill_id", "trait_id"),
    CONSTRAINT "rule_num_skill_id_fkey" FOREIGN KEY ("skill_id") REFERENCES "public"."skill" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE,
        CONSTRAINT "rule_num_trait_id_fkey" FOREIGN KEY ("trait_id") REFERENCES "public"."trait" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE
);

COMMENT ON TABLE "public"."rule_num" IS 'Describes a numeric rule of a trait, which skill to affect and by what amount';

CREATE TABLE "public"."sub_skill" (
    "sup_id" uuid NOT NULL,
    "sub_id" uuid NOT NULL,
    CONSTRAINT "sub_skill_sub_id_fkey" FOREIGN KEY ("sub_id") REFERENCES "public"."skill" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE,
        CONSTRAINT "sub_skill_sup_id_fkey" FOREIGN KEY ("sup_id") REFERENCES "public"."skill" ("id") ON
    UPDATE
        CASCADE ON DELETE CASCADE
);

COMMENT ON TABLE "public"."sub_skill" IS 'Describes a skill that inherits values of another skill.';