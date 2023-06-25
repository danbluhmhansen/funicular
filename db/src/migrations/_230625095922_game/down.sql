DROP TABLE "rule_num";
DROP TABLE "trait";
DROP TABLE "sub_skill";
DROP TABLE "skill";
DROP TABLE "game";

DELETE FROM "_migration" WHERE "name" = '230625095922_game';
NOTIFY pgrst, 'reload schema';
