schema "public" {}

table "game" {
  schema  = schema.public
  comment = "Collection of rules for a specific game."
  column "id" {
    null    = false
    type    = uuid
    default = sql("gen_rand_uuid7()")
  }
  column "name" {
    null = false
    type = text
  }
  column "slug" {
    type = text
    as {
      expr = "slugify(name)"
      type = STORED
    }
  }
  column "created" {
    null = true
    type = timestamp
    as {
      expr = "uuid7_time(id)"
      type = STORED
    }
  }
  column "description" {
    null = true
    type = text
  }
  primary_key {
    columns = [column.id]
  }
  index "game_slug_key" {
    unique  = true
    columns = [column.slug]
  }
}

table "skill" {
  schema  = schema.public
  comment = "Describes a specific attribute or skill of an entity like actors or items."
  column "id" {
    null    = false
    type    = uuid
    default = sql("gen_rand_uuid7()")
  }
  column "game_id" {
    null = false
    type = uuid
  }
  column "name" {
    null = false
    type = text
  }
  column "slug" {
    type = text
    as {
      expr = "slugify(name)"
      type = STORED
    }
  }
  column "created" {
    null = true
    type = timestamp
    as {
      expr = "uuid7_time(id)"
      type = STORED
    }
  }
  column "description" {
    null = true
    type = text
  }
  primary_key {
    columns = [column.id]
  }
  foreign_key "skill_game_id_fkey" {
    columns     = [column.game_id]
    ref_columns = [table.game.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
  index "skill_game_id_slug_key" {
    unique  = true
    columns = [column.game_id, column.slug]
  }
}

table "sub_skill" {
  schema  = schema.public
  comment = "Describes a skill that inherits values of another skill."
  column "sup_id" {
    null = false
    type = uuid
  }
  column "sub_id" {
    null = false
    type = uuid
  }
  foreign_key "sub_skill_sub_id_fkey" {
    columns     = [column.sub_id]
    ref_columns = [table.skill.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
  foreign_key "sub_skill_sup_id_fkey" {
    columns     = [column.sup_id]
    ref_columns = [table.skill.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
}

table "trait" {
  schema  = schema.public
  comment = "Describes a specific trait or effect of an entity like actors or items."
  column "id" {
    null    = false
    type    = uuid
    default = sql("gen_rand_uuid7()")
  }
  column "game_id" {
    null = false
    type = uuid
  }
  column "name" {
    null = false
    type = text
  }
  column "slug" {
    type = text
    as {
      expr = "slugify(name)"
      type = STORED
    }
  }
  column "created" {
    null = true
    type = timestamp
    as {
      expr = "uuid7_time(id)"
      type = STORED
    }
  }
  column "description" {
    null = true
    type = text
  }
  primary_key {
    columns = [column.id]
  }
  foreign_key "trait_game_id_fkey" {
    columns     = [column.game_id]
    ref_columns = [table.game.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
  index "trait_game_id_slug_key" {
    unique  = true
    columns = [column.game_id, column.slug]
  }
}

table "rule_num" {
  schema  = schema.public
  comment = "Describes a numeric rule of a trait, which skill to affect and by what amount"
  column "skill_id" {
    null = false
    type = uuid
  }
  column "trait_id" {
    null = false
    type = uuid
  }
  column "value" {
    null = false
    type = numeric
  }
  primary_key {
    columns = [column.skill_id, column.trait_id]
  }
  foreign_key "rule_num_skill_id_fkey" {
    columns     = [column.skill_id]
    ref_columns = [table.skill.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
  foreign_key "rule_num_trait_id_fkey" {
    columns     = [column.trait_id]
    ref_columns = [table.trait.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
}

table "actor_kind" {
  schema  = schema.public
  comment = "A kind of actor, like player or enemy."
  column "id" {
    null    = false
    type    = uuid
    default = sql("gen_rand_uuid7()")
  }
  column "game_id" {
    null = false
    type = uuid
  }
  column "name" {
    null = false
    type = text
  }
  column "slug" {
    type = text
    as {
      expr = "slugify(name)"
      type = STORED
    }
  }
  column "created" {
    null = true
    type = timestamp
    as {
      expr = "uuid7_time(id)"
      type = STORED
    }
  }
  column "description" {
    null = true
    type = text
  }
  primary_key {
    columns = [column.id]
  }
  foreign_key "actor_kind_game_id_fkey" {
    columns     = [column.game_id]
    ref_columns = [table.game.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
  index "actor_kind_game_id_slug_key" {
    unique  = true
    columns = [column.game_id, column.slug]
  }
}

table "gear_kind" {
  schema  = schema.public
  comment = "A kind of gear, like equipment or consumables."
  column "id" {
    null    = false
    type    = uuid
    default = sql("gen_rand_uuid7()")
  }
  column "game_id" {
    null = false
    type = uuid
  }
  column "name" {
    null = false
    type = text
  }
  column "slug" {
    type = text
    as {
      expr = "slugify(name)"
      type = STORED
    }
  }
  column "created" {
    null = true
    type = timestamp
    as {
      expr = "uuid7_time(id)"
      type = STORED
    }
  }
  column "description" {
    null = true
    type = text
  }
  primary_key {
    columns = [column.id]
  }
  foreign_key "gear_kind_game_id_fkey" {
    columns     = [column.game_id]
    ref_columns = [table.game.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
  index "gear_kind_game_id_slug_key" {
    unique  = true
    columns = [column.game_id, column.slug]
  }
}

table "actor" {
  schema  = schema.public
  comment = "An individual controlled by a Game Master or player."
  column "id" {
    null    = false
    type    = uuid
    default = sql("gen_rand_uuid7()")
  }
  column "kind_id" {
    null = false
    type = uuid
  }
  column "name" {
    null = false
    type = text
  }
  column "slug" {
    type = text
    as {
      expr = "slugify(name)"
      type = STORED
    }
  }
  column "created" {
    null = true
    type = timestamp
    as {
      expr = "uuid7_time(id)"
      type = STORED
    }
  }
  column "description" {
    null = true
    type = text
  }
  primary_key {
    columns = [column.id]
  }
  foreign_key "actor_kind_id_fkey" {
    columns     = [column.kind_id]
    ref_columns = [table.actor_kind.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
  index "actor_kind_id_slug_key" {
    unique  = true
    columns = [column.kind_id, column.slug]
  }
}

table "gear" {
  schema  = schema.public
  comment = "Equipment or gear used by actors."
  column "id" {
    null    = false
    type    = uuid
    default = sql("gen_rand_uuid7()")
  }
  column "kind_id" {
    null = false
    type = uuid
  }
  column "name" {
    null = false
    type = text
  }
  column "slug" {
    type = text
    as {
      expr = "slugify(name)"
      type = STORED
    }
  }
  column "created" {
    null = true
    type = timestamp
    as {
      expr = "uuid7_time(id)"
      type = STORED
    }
  }
  column "description" {
    null = true
    type = text
  }
  primary_key {
    columns = [column.id]
  }
  foreign_key "gear_kind_id_fkey" {
    columns     = [column.kind_id]
    ref_columns = [table.gear_kind.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
  index "gear_kind_id_slug_key" {
    unique  = true
    columns = [column.kind_id, column.slug]
  }
}

table "actor_skill" {
  schema  = schema.public
  comment = "Connection between actor kinds and skills."
  column "kind_id" {
    null = false
    type = uuid
  }
  column "skill_id" {
    null = false
    type = uuid
  }
  primary_key {
    columns = [column.kind_id, column.skill_id]
  }
  foreign_key "actor_skill_kind_id_fkey" {
    columns     = [column.kind_id]
    ref_columns = [table.actor_kind.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
  foreign_key "actor_skill_skill_id_fkey" {
    columns     = [column.skill_id]
    ref_columns = [table.skill.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
}

table "actor_trait" {
  schema  = schema.public
  comment = "Connection between actors and traits."
  column "actor_id" {
    null = false
    type = uuid
  }
  column "trait_id" {
    null = false
    type = uuid
  }
  column "amount" {
    null    = true
    type    = integer
    default = 1
  }
  primary_key {
    columns = [column.actor_id, column.trait_id]
  }
  foreign_key "actor_trait_actor_id_fkey" {
    columns     = [column.actor_id]
    ref_columns = [table.actor.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
  foreign_key "actor_trait_trait_id_fkey" {
    columns     = [column.trait_id]
    ref_columns = [table.trait.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
}

table "actor_gear" {
  schema  = schema.public
  comment = "Connection between actors and gears."
  column "actor_id" {
    null = false
    type = uuid
  }
  column "gear_id" {
    null = false
    type = uuid
  }
  column "amount" {
    null    = true
    type    = integer
    default = 1
  }
  primary_key {
    columns = [column.actor_id, column.gear_id]
  }
  foreign_key "actor_gear_actor_id_fkey" {
    columns     = [column.actor_id]
    ref_columns = [table.actor.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
  foreign_key "actor_gear_gear_id_fkey" {
    columns     = [column.gear_id]
    ref_columns = [table.gear.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
}

table "gear_skill" {
  schema  = schema.public
  comment = "Connection between gear kinds and skills."
  column "kind_id" {
    null = false
    type = uuid
  }
  column "skill_id" {
    null = false
    type = uuid
  }
  primary_key {
    columns = [column.kind_id, column.skill_id]
  }
  foreign_key "gear_skill_kind_id_fkey" {
    columns     = [column.kind_id]
    ref_columns = [table.gear_kind.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
  foreign_key "gear_skill_skill_id_fkey" {
    columns     = [column.skill_id]
    ref_columns = [table.skill.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
}

table "gear_trait" {
  schema  = schema.public
  comment = "Connection between gears and traits."
  column "gear_id" {
    null = false
    type = uuid
  }
  column "trait_id" {
    null = false
    type = uuid
  }
  column "amount" {
    null    = true
    type    = integer
    default = 1
  }
  primary_key {
    columns = [column.gear_id, column.trait_id]
  }
  foreign_key "gear_trait_gear_id_fkey" {
    columns     = [column.gear_id]
    ref_columns = [table.gear.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
  foreign_key "gear_trait_trait_id_fkey" {
    columns     = [column.trait_id]
    ref_columns = [table.trait.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
}

view "actor_num_skill" {
  schema = schema.public
  comment = "View of actor's current skill values."
  column "game_id" {
    type = uuid
  }
  column "actor_id" {
    type = uuid
  }
  column "skill_id" {
    type = uuid
  }
  column "value" {
    type = numeric
  }
  depends_on = [
    table.skill,
    table.sub_skill,
    table.trait,
    table.rule_num,
    table.actor_kind,
    table.actor,
    table.actor_skill,
    table.actor_trait
  ]
  as = <<-SQL
    SELECT
      actor_kind.game_id                                    AS game_id,
      actor.id                                              AS actor_id,
      COALESCE(sub_skill.sub_id, actor_skill.skill_id)      AS skill_id,
      SUM(rule_num.value * COALESCE(actor_trait.amount, 0)) AS value
    FROM      ${table.actor.name}
    JOIN      ${table.actor_kind.name}  ON actor_kind.id        = actor.kind_id
    JOIN      ${table.actor_skill.name} ON actor_skill.kind_id  = actor.kind_id
    LEFT JOIN ${table.sub_skill.name}   ON sub_skill.sub_id     = actor_skill.skill_id
    JOIN      ${table.rule_num.name}    ON rule_num.skill_id    = ANY(ARRAY[actor_skill.skill_id, sub_skill.sup_id])
    JOIN      ${table.trait.name}       ON trait.id             = rule_num.trait_id
    LEFT JOIN ${table.actor_trait.name} ON actor_trait.actor_id = actor.id AND actor_trait.trait_id = trait.id
    GROUP BY actor_kind.game_id, actor.id, COALESCE(sub_skill.sub_id, actor_skill.skill_id)
    ORDER BY 1, 2, 3
  SQL
}

view "gear_num_skill" {
  schema = schema.public
  comment = "View of an actor's gear's current skill values."
  column "game_id" {
    type = uuid
  }
  column "actor_id" {
    type = uuid
  }
  column "gear_id" {
    type = uuid
  }
  column "skill_id" {
    type = uuid
  }
  column "value" {
    type = numeric
  }
  depends_on = [
    table.skill,
    table.sub_skill,
    table.trait,
    table.rule_num,
    table.actor_kind,
    table.gear_kind,
    table.actor,
    table.gear,
    table.actor_skill,
    table.actor_trait,
    table.actor_gear,
    table.gear_skill,
    table.gear_trait
  ]
  as = <<-SQL
    SELECT
      gear_kind.game_id                                   AS game_id,
      actor_gear.actor_id                                 AS actor_id,
      gear.id                                             AS gear_id,
      COALESCE(sub_skill.sub_id, gear_skill.skill_id)     AS skill_id,
      SUM(
        rule_num.value * COALESCE(actor_trait.amount, 0) +
        rule_num.value * COALESCE(gear_trait.amount,  0)) AS value
    FROM      ${table.gear.name}
    JOIN      ${table.gear_kind.name}   ON gear_kind.id         = gear.kind_id
    JOIN      ${table.actor_gear.name}  ON actor_gear.gear_id   = gear.id
    JOIN      ${table.actor.name}       ON actor.id             = actor_gear.actor_id
    JOIN      ${table.gear_skill.name}  ON gear_skill.kind_id   = gear.kind_id
    LEFT JOIN ${table.sub_skill.name}   ON sub_skill.sub_id     = gear_skill.skill_id
    JOIN      ${table.rule_num.name}    ON rule_num.skill_id    = ANY(ARRAY[gear_skill.skill_id, sub_skill.sup_id])
    JOIN      ${table.trait.name}       ON trait.id             = rule_num.trait_id
    LEFT JOIN ${table.gear_trait.name}  ON gear_trait.gear_id   = gear.id AND gear_trait.trait_id = trait.id
    LEFT JOIN ${table.actor_trait.name} ON actor_trait.actor_id = actor.id AND actor_trait.trait_id = trait.id
    GROUP BY gear_kind.game_id, actor_gear.actor_id, gear.id, COALESCE(sub_skill.sub_id, gear_skill.skill_id)
    ORDER BY 1, 2, 3, 4
  SQL
}
