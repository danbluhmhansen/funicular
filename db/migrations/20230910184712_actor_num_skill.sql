--
CREATE VIEW "public"."actor_num_skill" AS
SELECT
    "actor_kind"."game_id"                                        AS "game_id",
    "actor"."id"                                                  AS "actor_id",
    COALESCE("sub_skill"."sub_id", "actor_skill"."skill_id")      AS "skill_id",
    SUM("rule_num"."value" * COALESCE("actor_trait"."amount", 0)) AS "value"
FROM "public"."actor"
JOIN      "public"."actor_kind"  ON "actor_kind"."id"        = "actor"."kind_id"
JOIN      "public"."actor_skill" ON "actor_skill"."kind_id"  = "actor"."kind_id"
LEFT JOIN "public"."sub_skill"   ON "sub_skill"."sub_id"     = "actor_skill"."skill_id"
JOIN      "public"."rule_num"    ON "rule_num"."skill_id"    = ANY(ARRAY["actor_skill"."skill_id", "sub_skill"."sup_id"])
JOIN      "public"."trait"       ON "trait"."id"             = "rule_num"."trait_id"
LEFT JOIN "public"."actor_trait" ON "actor_trait"."actor_id" = "actor"."id" AND "actor_trait"."trait_id" = "trait"."id"
GROUP BY "actor_kind"."game_id", "actor"."id", COALESCE("sub_skill"."sub_id", "actor_skill"."skill_id")
ORDER BY 1, 2, 3;
--
COMMENT ON VIEW "public"."actor_num_skill" IS $$View of actor's current skill values.$$;
