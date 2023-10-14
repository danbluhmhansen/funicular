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
            skill.name AS name,
            SUM(rule_num.value * COALESCE(actor_trait.amount, 0)) AS value
        FROM
            actor
            JOIN actor_skill ON actor_skill.kind_id = actor.kind_id
            LEFT JOIN sub_skill ON sub_skill.sub_id = actor_skill.skill_id
            JOIN skill ON skill.id = COALESCE(actor_skill.skill_id, sub_skill.sub_id)
            JOIN rule_num ON rule_num.skill_id = ANY(ARRAY [actor_skill.skill_id, sub_skill.sub_id])
            JOIN trait ON trait.id = rule_num.trait_id
            LEFT JOIN actor_trait ON actor_trait.actor_id = actor.id
            AND actor_trait.trait_id = trait.id
        GROUP BY
            actor.id,
            skill.id
    ) skills ON skills.id = actor.id
GROUP BY
    actor.id;