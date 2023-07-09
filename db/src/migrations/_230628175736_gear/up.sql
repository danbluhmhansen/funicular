CREATE TABLE "public"."gear_kind" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "game_id"     uuid NOT NULL REFERENCES "public"."game"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "name"        text NOT NULL,
    "created"     timestamp GENERATED ALWAYS AS (uuid7_time("id")) STORED,
    "description" text,
    UNIQUE ("game_id", "name")
);

COMMENT ON TABLE "public"."gear_kind" IS 'A kind of gear, like equipment or consumables.';
GRANT SELECT, INSERT, UPDATE, DELETE ON  "public"."gear_kind" TO "anon";

CREATE TABLE "public"."gear_skill" (
    "kind_id"  uuid NOT NULL REFERENCES "public"."gear_kind"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "skill_id" uuid NOT NULL REFERENCES "public"."skill"("id")     ON DELETE CASCADE ON UPDATE CASCADE,
    PRIMARY KEY ("kind_id", "skill_id")
);

COMMENT ON TABLE "public"."gear_skill" IS 'Connection between gear kinds and skills.';
GRANT SELECT, INSERT, UPDATE, DELETE ON  "public"."gear_skill" TO "anon";

CREATE TABLE "public"."gear" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "kind_id"     uuid NOT NULL REFERENCES "public"."gear_kind"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "name"        text NOT NULL,
    "created"     timestamp GENERATED ALWAYS AS (uuid7_time("id")) STORED,
    "description" text,
    UNIQUE ("kind_id", "name")
);

COMMENT ON TABLE "public"."gear" IS 'Equipment or gear used by actors.';
GRANT SELECT, INSERT, UPDATE, DELETE ON  "public"."gear" TO "anon";

CREATE TABLE "public"."actor_gear" (
    "actor_id" uuid NOT NULL REFERENCES "public"."actor"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "gear_id"  uuid NOT NULL REFERENCES "public"."gear"("id")  ON DELETE CASCADE ON UPDATE CASCADE,
    "amount"   int  DEFAULT 1,
    PRIMARY KEY ("actor_id", "gear_id")
);

COMMENT ON TABLE "public"."actor_gear" IS 'Connection between actors and gears.';
GRANT SELECT, INSERT, UPDATE, DELETE ON  "public"."actor_gear" TO "anon";

CREATE TABLE "public"."gear_trait" (
    "gear_id"  uuid NOT NULL REFERENCES "public"."gear"("id")  ON DELETE CASCADE ON UPDATE CASCADE,
    "trait_id" uuid NOT NULL REFERENCES "public"."trait"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "amount"   int  DEFAULT 1,
    PRIMARY KEY ("gear_id", "trait_id")
);

COMMENT ON TABLE "public"."gear_trait" IS 'Connection between gears and traits.';
GRANT SELECT, INSERT, UPDATE, DELETE ON  "public"."gear_trait" TO "anon";

CREATE VIEW "gear_num_skill" AS
SELECT
    "gear_kind"."game_id"                                         AS "game_id",
    "actor_gear"."actor_id"                                       AS "actor_id",
    "gear"."id"                                                   AS "gear_id",
    COALESCE("sub_skill"."sub_id", "gear_skill"."skill_id")       AS "skill_id",
    SUM(
        "rule_num"."value" * COALESCE("actor_trait"."amount", 0) +
        "rule_num"."value" * COALESCE("gear_trait"."amount",  0)) AS "value"
FROM "public"."gear"
JOIN      "public"."gear_kind"   ON "gear_kind"."id"         = "gear"."kind_id"
JOIN      "public"."actor_gear"  ON "actor_gear"."gear_id"   = "gear"."id"
JOIN      "public"."actor"       ON "actor"."id"             = "actor_gear"."actor_id"
JOIN      "public"."gear_skill"  ON "gear_skill"."kind_id"   = "gear"."kind_id"
LEFT JOIN "public"."sub_skill"   ON "sub_skill"."sub_id"     = "gear_skill"."skill_id"
JOIN      "public"."rule_num"    ON "rule_num"."skill_id"    = ANY(ARRAY["gear_skill"."skill_id", "sub_skill"."sup_id"])
JOIN      "public"."trait"       ON "trait"."id"             = "rule_num"."trait_id"
LEFT JOIN "public"."gear_trait"  ON "gear_trait"."gear_id"   = "gear"."id" AND "gear_trait"."trait_id" = "trait"."id"
LEFT JOIN "public"."actor_trait" ON "actor_trait"."actor_id" = "actor"."id" AND "actor_trait"."trait_id" = "trait"."id"
GROUP BY "gear_kind"."game_id", "actor_gear"."actor_id", "gear"."id", COALESCE("sub_skill"."sub_id", "gear_skill"."skill_id")
ORDER BY 1, 2, 3, 4;

COMMENT ON VIEW "public"."gear_num_skill" IS $$View of an actor's gear's current skill values.$$;
GRANT SELECT, INSERT, UPDATE, DELETE ON "public"."gear_num_skill" TO "anon";

INSERT INTO "public"."_migration" VALUES ('230628175736_gear');
NOTIFY pgrst, 'reload schema';
