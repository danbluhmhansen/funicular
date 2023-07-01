DROP VIEW  "gear_num_skill";
DROP TABLE "gear_trait";
DROP TABLE "actor_gear";
DROP TABLE "gear";
DROP TABLE "gear_skill";
DROP TABLE "gear_kind";

DELETE FROM "_migration" WHERE "name" = '230628175736_gear';
NOTIFY pgrst, 'reload schema';
