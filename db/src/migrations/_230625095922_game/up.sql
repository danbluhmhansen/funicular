CREATE TABLE "public"."game" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "name"        text NOT NULL,
    "description" text
);

COMMENT ON TABLE "game" IS 'Collection of rules for a specific game.';

CREATE TABLE "public"."skill" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "game_id"     uuid NOT NULL REFERENCES "game"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "name"        text NOT NULL,
    "description" text
);

COMMENT ON TABLE "skill" IS 'Describes a specific attribute or skill of an entity like actors or items.';

CREATE TABLE "public"."sub_skill" (
    "sup_id" uuid NOT NULL REFERENCES "skill"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "sub_id" uuid NOT NULL REFERENCES "skill"("id") ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE "public"."trait" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "name"        text NOT NULL,
    "description" text
);

COMMENT ON TABLE "trait" IS 'Describes a specific trait or effect of an entity like actors or items.';

CREATE TABLE "public"."rule_num" (
    "skill_id" uuid NOT NULL REFERENCES "skill"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "trait_id" uuid NOT NULL REFERENCES "trait"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "value"    numeric NOT NULL,
    PRIMARY KEY ("skill_id", "trait_id")
);

COMMENT ON TABLE "rule_num" IS 'Describes a numeric rule of a trait, which skill to affect and by what amount';

INSERT INTO "public"."_migration" VALUES ('230625095922_game');
NOTIFY pgrst, 'reload schema';
