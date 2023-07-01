CREATE TABLE "public"."actor_kind" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "name"        text NOT NULL,
    "description" text
);

COMMENT ON TABLE "actor_kind" IS 'A kind of actor, like player or enemy.';

CREATE TABLE "public"."actor_skill" (
    "kind_id"  uuid NOT NULL REFERENCES "actor_kind"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "skill_id" uuid NOT NULL REFERENCES "skill"("id")      ON DELETE CASCADE ON UPDATE CASCADE,
    PRIMARY KEY ("kind_id", "skill_id")
);

COMMENT ON TABLE "actor_skill" IS 'Connection between actor kinds and skills.';

CREATE TABLE "public"."actor" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "kind_id"     uuid NOT NULL REFERENCES "actor_kind"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "name"        text NOT NULL,
    "description" text
);

COMMENT ON TABLE "actor" IS 'An individual controlled by a Game Master or player.';

CREATE TABLE "public"."actor_trait" (
    "actor_id" uuid NOT NULL REFERENCES "actor"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "trait_id" uuid NOT NULL REFERENCES "trait"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "amount"   int  DEFAULT 1,
    PRIMARY KEY ("actor_id", "trait_id")
);

COMMENT ON TABLE "actor_trait" IS 'Connection between actors and traits.';

CREATE VIEW "actor_num_skill" AS
SELECT
    "actor"."id"                                                  AS "actor_id",
    COALESCE("sub_skill"."sub_id", "actor_skill"."skill_id")      AS "skill_id",
    SUM("rule_num"."value" * COALESCE("actor_trait"."amount", 0)) AS "value"
FROM "public"."actor"
JOIN      "actor_skill"  ON "actor_skill"."kind_id"  = "actor"."kind_id"
LEFT JOIN "sub_skill"    ON "sub_skill"."sub_id"     = "actor_skill"."skill_id"
JOIN      "rule_num"     ON "rule_num"."skill_id"    = ANY(ARRAY["actor_skill"."skill_id", "sub_skill"."sup_id"])
JOIN      "trait"        ON "trait"."id"             = "rule_num"."trait_id"
LEFT JOIN  "actor_trait" ON "actor_trait"."actor_id" = "actor"."id" AND "actor_trait"."trait_id" = "trait"."id"
GROUP BY "actor"."id", COALESCE("sub_skill"."sub_id", "actor_skill"."skill_id")
ORDER BY 1, 2;

INSERT INTO "public"."_migration" VALUES ('230625105931_actor');
NOTIFY pgrst, 'reload schema';
