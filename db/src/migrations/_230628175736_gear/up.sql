CREATE TABLE "public"."gear_kind" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "name"        text NOT NULL,
    "description" text
);

COMMENT ON TABLE "gear_kind" IS 'A kind of gear, like equipment or consumables.';
GRANT SELECT ON "gear_kind" TO "anon";

CREATE TABLE "public"."gear_skill" (
    "kind_id"  uuid NOT NULL REFERENCES "gear_kind"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "skill_id" uuid NOT NULL REFERENCES "skill"("id")     ON DELETE CASCADE ON UPDATE CASCADE,
    PRIMARY KEY ("kind_id", "skill_id")
);

COMMENT ON TABLE "gear_skill" IS 'Connection between gear kinds and skills.';
GRANT SELECT ON "gear_skill" TO "anon";

CREATE TABLE "public"."gear" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "kind_id"     uuid NOT NULL REFERENCES "gear_kind"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "name"        text NOT NULL,
    "description" text
);

COMMENT ON TABLE "gear" IS 'Equipment or gear used by actors.';
GRANT SELECT ON "gear" TO "anon";

CREATE TABLE "public"."actor_gear" (
    "actor_id" uuid NOT NULL REFERENCES "actor"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "gear_id"  uuid NOT NULL REFERENCES "gear"("id")  ON DELETE CASCADE ON UPDATE CASCADE,
    "amount"   int  DEFAULT 1,
    PRIMARY KEY ("actor_id", "gear_id")
);

COMMENT ON TABLE "actor_gear" IS 'Connection between actors and gears.';
GRANT SELECT ON "actor_gear" TO "anon";

CREATE TABLE "public"."gear_trait" (
    "gear_id"  uuid NOT NULL REFERENCES "gear"("id")  ON DELETE CASCADE ON UPDATE CASCADE,
    "trait_id" uuid NOT NULL REFERENCES "trait"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "amount"   int  DEFAULT 1,
    PRIMARY KEY ("gear_id", "trait_id")
);

COMMENT ON TABLE "gear_trait" IS 'Connection between gears and traits.';
GRANT SELECT ON "gear_trait" TO "anon";

CREATE VIEW "gear_num_skill" AS
SELECT
    "actor_gear"."actor_id"                                       AS "actor_id",
    "gear"."id"                                                   AS "gear_id",
    COALESCE("sub_skill"."sub_id", "gear_skill"."skill_id")       AS "skill_id",
    SUM(
        "rule_num"."value" * COALESCE("actor_trait"."amount", 0) +
        "rule_num"."value" * COALESCE("gear_trait"."amount",  0)) AS "value"
FROM "public"."gear"
JOIN      "actor_gear"   ON "actor_gear"."gear_id"   = "gear"."id"
JOIN      "actor"        ON "actor"."id"             = "actor_gear"."actor_id"
JOIN      "gear_skill"   ON "gear_skill"."kind_id"   = "gear"."kind_id"
LEFT JOIN "sub_skill"    ON "sub_skill"."sub_id"     = "gear_skill"."skill_id"
JOIN      "rule_num"     ON "rule_num"."skill_id"    = ANY(ARRAY["gear_skill"."skill_id", "sub_skill"."sup_id"])
JOIN      "trait"        ON "trait"."id"             = "rule_num"."trait_id"
LEFT JOIN "gear_trait"   ON "gear_trait"."gear_id"   = "gear"."id" AND "gear_trait"."trait_id" = "trait"."id"
LEFT JOIN  "actor_trait" ON "actor_trait"."actor_id" = "actor"."id" AND "actor_trait"."trait_id" = "trait"."id"
GROUP BY "actor_gear"."actor_id", "gear"."id", COALESCE("sub_skill"."sub_id", "gear_skill"."skill_id")
ORDER BY 1, 2, 3;

COMMENT ON VIEW "gear_num_skill" IS $$View of an actor's gear's current skill values.$$;
GRANT SELECT ON "gear_num_skill" TO "anon";

INSERT INTO "public"."_migration" VALUES ('230628175736_gear');
NOTIFY pgrst, 'reload schema';
