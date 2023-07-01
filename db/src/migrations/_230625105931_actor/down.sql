DROP VIEW  "actor_num_skill";
DROP TABLE "actor_trait";
DROP TABLE "actor";
DROP TABLE "actor_skill";
DROP TABLE "actor_kind";

DELETE FROM "_migration" WHERE "name" = '230625105931_actor';
NOTIFY pgrst, 'reload schema';
