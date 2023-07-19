--
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
--
COMMENT ON VIEW "public"."gear_num_skill" IS $$View of an actor's gear's current skill values.$$;
--
GRANT SELECT ON "public"."gear_num_skill" TO "anon";
-- Notify Postgrest
NOTIFY pgrst, 'reload schema';
