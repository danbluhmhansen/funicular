DROP VIEW  "public"."gear_num_skill";
DROP TABLE "public"."gear_trait";
DROP TABLE "public"."actor_gear";
DROP TABLE "public"."gear";
DROP TABLE "public"."gear_skill";
DROP TABLE "public"."gear_kind";

DELETE FROM "public"."_migration" WHERE "name" = '230628175736_gear';
NOTIFY pgrst, 'reload schema';
