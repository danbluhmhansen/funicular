DROP TABLE "item_trait";
DROP TABLE "actor_item";
DROP TABLE "item";
DROP TABLE "actor_trait";
DROP TABLE "actor";

DELETE FROM "_migration" WHERE "name" = '230625105931_actor';
NOTIFY pgrst, 'reload schema';
