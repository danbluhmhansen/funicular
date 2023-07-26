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
  index "actor_kind_id_name_key" {
    unique  = true
    columns = [column.kind_id, column.name]
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
  index "actor_kind_game_id_name_key" {
    unique  = true
    columns = [column.game_id, column.name]
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
  index "game_name_key" {
    unique  = true
    columns = [column.name]
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
  index "gear_kind_id_name_key" {
    unique  = true
    columns = [column.kind_id, column.name]
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
  index "gear_kind_game_id_name_key" {
    unique  = true
    columns = [column.game_id, column.name]
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
  index "skill_game_id_name_key" {
    unique  = true
    columns = [column.game_id, column.name]
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
  index "trait_game_id_name_key" {
    unique  = true
    columns = [column.game_id, column.name]
  }
}
schema "public" {
}
