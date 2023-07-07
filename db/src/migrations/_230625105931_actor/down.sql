DROP FUNCTION "public"."skill"("public"."actor_num_skill");
DROP FUNCTION "public"."actor"("public"."actor_num_skill");
DROP FUNCTION "public"."game"("public"."actor_num_skill");
DROP FUNCTION "public"."actor_num_skill"("public"."actor");
DROP VIEW  "public"."actor_num_skill";
DROP TABLE "public"."actor_trait";
DROP TABLE "public"."actor";
DROP TABLE "public"."actor_skill";
DROP TABLE "public"."actor_kind";

DELETE FROM "public"."_migration" WHERE "name" = '230625105931_actor';
NOTIFY pgrst, 'reload schema';
