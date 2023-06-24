CREATE TABLE "item" (
    "id"   uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "name" text NOT NULL CHECK ("name" ~ '^[a-z_]*$')
);

CREATE TABLE "character_item" (
    "character_id" uuid NOT NULL REFERENCES "character"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "item_id"      uuid NOT NULL REFERENCES "item"("id")      ON DELETE CASCADE ON UPDATE CASCADE,

    PRIMARY KEY ("character_id", "item_id")
);

CREATE TABLE "item_trait" (
    "item_id"  uuid NOT NULL REFERENCES "item"("id")  ON DELETE CASCADE ON UPDATE CASCADE,
    "trait_id" uuid NOT NULL REFERENCES "trait"("id") ON DELETE CASCADE ON UPDATE CASCADE,

    PRIMARY KEY ("item_id", "trait_id")
);

CREATE VIEW "item_numeric_field" AS
SELECT
    "item"."id"                 AS "item_id",
    "character"."id"            AS "character_id",
    "field"."id"                AS "field_id",
    SUM("numeric_rule"."value") AS "value"
FROM "field"
JOIN "numeric_rule"   ON "numeric_rule"."field_id"  = "field"."id"
JOIN "trait"          ON "trait"."id"               = "numeric_rule"."trait_id"
JOIN "item_trait"     ON "item_trait"."trait_id"    = "trait"."id"
JOIN "item"           ON "item"."id"                = "item_trait"."item_id"
JOIN "character_item" ON "character_item"."item_id" = "item"."id"
JOIN "character"      ON "character"."id"           = "character_item"."character_id"
GROUP BY "field"."id", "item"."id", "character"."id"
ORDER BY "item"."id";

INSERT INTO "_migration" VALUES ('230624135332_items');
