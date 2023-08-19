-- Drop index "actor_kind_id_name_key" from table: "actor"
DROP INDEX "public"."actor_kind_id_name_key";
-- Modify "actor" table
ALTER TABLE "public"."actor" ADD COLUMN "slug" text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED;
-- Create index "actor_kind_id_slug_key" to table: "actor"
CREATE UNIQUE INDEX "actor_kind_id_slug_key" ON "public"."actor" ("kind_id", "slug");
-- Drop index "actor_kind_game_id_name_key" from table: "actor_kind"
DROP INDEX "public"."actor_kind_game_id_name_key";
-- Modify "actor_kind" table
ALTER TABLE "public"."actor_kind" ADD COLUMN "slug" text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED;
-- Create index "actor_kind_game_id_slug_key" to table: "actor_kind"
CREATE UNIQUE INDEX "actor_kind_game_id_slug_key" ON "public"."actor_kind" ("game_id", "slug");
-- Drop index "game_name_key" from table: "game"
DROP INDEX "public"."game_name_key";
-- Modify "game" table
ALTER TABLE "public"."game" ADD COLUMN "slug" text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED;
-- Create index "game_slug_key" to table: "game"
CREATE UNIQUE INDEX "game_slug_key" ON "public"."game" ("slug");
-- Drop index "gear_kind_id_name_key" from table: "gear"
DROP INDEX "public"."gear_kind_id_name_key";
-- Modify "gear" table
ALTER TABLE "public"."gear" ADD COLUMN "slug" text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED;
-- Create index "gear_kind_id_slug_key" to table: "gear"
CREATE UNIQUE INDEX "gear_kind_id_slug_key" ON "public"."gear" ("kind_id", "slug");
-- Drop index "gear_kind_game_id_name_key" from table: "gear_kind"
DROP INDEX "public"."gear_kind_game_id_name_key";
-- Modify "gear_kind" table
ALTER TABLE "public"."gear_kind" ADD COLUMN "slug" text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED;
-- Create index "gear_kind_game_id_slug_key" to table: "gear_kind"
CREATE UNIQUE INDEX "gear_kind_game_id_slug_key" ON "public"."gear_kind" ("game_id", "slug");
-- Drop index "skill_game_id_name_key" from table: "skill"
DROP INDEX "public"."skill_game_id_name_key";
-- Modify "skill" table
ALTER TABLE "public"."skill" ADD COLUMN "slug" text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED;
-- Create index "skill_game_id_slug_key" to table: "skill"
CREATE UNIQUE INDEX "skill_game_id_slug_key" ON "public"."skill" ("game_id", "slug");
-- Drop index "trait_game_id_name_key" from table: "trait"
DROP INDEX "public"."trait_game_id_name_key";
-- Modify "trait" table
ALTER TABLE "public"."trait" ADD COLUMN "slug" text NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED;
-- Create index "trait_game_id_slug_key" to table: "trait"
CREATE UNIQUE INDEX "trait_game_id_slug_key" ON "public"."trait" ("game_id", "slug");
