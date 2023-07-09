CREATE TABLE "public"."game" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "name"        text NOT NULL UNIQUE,
    "created"     timestamp GENERATED ALWAYS AS (uuid7_time("id")) STORED,
    "description" text
);

COMMENT ON TABLE "public"."game" IS 'Collection of rules for a specific game.';
GRANT SELECT, INSERT, UPDATE, DELETE ON  "public"."game" TO "anon";

CREATE TABLE "public"."skill" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "game_id"     uuid NOT NULL REFERENCES "public"."game"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "name"        text NOT NULL,
    "created"     timestamp GENERATED ALWAYS AS (uuid7_time("id")) STORED,
    "description" text,
    UNIQUE ("game_id", "name")
);

COMMENT ON TABLE "public"."skill" IS 'Describes a specific attribute or skill of an entity like actors or items.';
GRANT SELECT, INSERT, UPDATE, DELETE ON  "public"."skill" TO "anon";

CREATE TABLE "public"."sub_skill" (
    "sup_id" uuid NOT NULL REFERENCES "public"."skill"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "sub_id" uuid NOT NULL REFERENCES "public"."skill"("id") ON DELETE CASCADE ON UPDATE CASCADE
);

COMMENT ON TABLE "public"."sub_skill" IS 'Describes a skill that inherits values of another skill.';
GRANT SELECT, INSERT, UPDATE, DELETE ON  "public"."sub_skill" TO "anon";

CREATE TABLE "public"."trait" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "game_id"     uuid NOT NULL REFERENCES "public"."game"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "name"        text NOT NULL,
    "created"     timestamp GENERATED ALWAYS AS (uuid7_time("id")) STORED,
    "description" text,
    UNIQUE ("game_id", "name")
);

COMMENT ON TABLE "public"."trait" IS 'Describes a specific trait or effect of an entity like actors or items.';
GRANT SELECT, INSERT, UPDATE, DELETE ON  "public"."trait" TO "anon";

CREATE TABLE "public"."rule_num" (
    "skill_id" uuid NOT NULL REFERENCES "public"."skill"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "trait_id" uuid NOT NULL REFERENCES "public"."trait"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "value"    numeric NOT NULL,
    PRIMARY KEY ("skill_id", "trait_id")
);

COMMENT ON TABLE "public"."rule_num" IS 'Describes a numeric rule of a trait, which skill to affect and by what amount';
GRANT SELECT, INSERT, UPDATE, DELETE ON  "public"."rule_num" TO "anon";

INSERT INTO "public"."_migration" VALUES ('230625095922_game');
NOTIFY pgrst, 'reload schema';
