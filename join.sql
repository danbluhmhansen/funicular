SELECT *
FROM actor_kind
JOIN actor_skill ON actor_skill.kind_id = actor_kind.id
JOIN skill ON skill.id = actor_skill.skill_id
LEFT JOIN sub_skill ON sub_skill.sub_id = skill.id
JOIN rule_num ON rule_num.skill_id = sub_skill.sup_id OR rule_num.skill_id = skill.id
JOIN trait ON trait.id = rule_num.trait_id
JOIN actor_trait ON actor_trait.trait_id = trait.id
JOIN actor ON actor.kind_id = actor_kind.id AND actor.id = actor_trait.actor_id;
