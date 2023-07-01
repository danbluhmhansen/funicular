DROP VIEW  "public"."actor_num_skill";
DROP TABLE "public"."actor_trait";
DROP TABLE "public"."actor";
DROP TABLE "public"."actor_skill";
DROP TABLE "public"."actor_kind";

DELETE FROM "public"."_migration" WHERE "name" = '230625105931_actor';
NOTIFY pgrst, 'reload schema';
