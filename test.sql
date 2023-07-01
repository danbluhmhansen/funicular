-- SELECT
--     "actor"."id"                                                  AS "actor_id",
--     COALESCE("sub_skill"."sub_id", "actor_skill"."skill_id")      AS "skill_id",
--     SUM("rule_num"."value" * COALESCE("actor_trait"."amount", 0)) AS "value"
-- FROM "public"."actor"
-- JOIN      "actor_skill"  ON "actor_skill"."kind_id"  = "actor"."kind_id"
-- LEFT JOIN "sub_skill"    ON "sub_skill"."sub_id"     = "actor_skill"."skill_id"
-- JOIN      "rule_num"     ON "rule_num"."skill_id"    = ANY(ARRAY["actor_skill"."skill_id", "sub_skill"."sup_id"])
-- JOIN      "trait"        ON "trait"."id"             = "rule_num"."trait_id"
-- LEFT JOIN  "actor_trait" ON "actor_trait"."actor_id" = "actor"."id" AND "actor_trait"."trait_id" = "trait"."id"
-- GROUP BY "actor"."id", COALESCE("sub_skill"."sub_id", "actor_skill"."skill_id")
-- ORDER BY 1, 2;

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
