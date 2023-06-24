CREATE TABLE "schema" (
    "id"   uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "name" text NOT NULL CHECK ("name" ~ '^[a-z_]*$')
);

COMMENT ON TABLE "schema" IS 'Collection of fields.';

CREATE TABLE "field" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "schema_id"   uuid NOT NULL REFERENCES "schema"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "field_id"    uuid          REFERENCES "field"("id")  ON DELETE CASCADE ON UPDATE CASCADE,
    "name"        text NOT NULL CHECK ("name" ~ '^[a-z_]*$'),
    "description" text
);

COMMENT ON TABLE "field" IS 'Describes a specific attribute of a character or item.';

CREATE TABLE "character" (
    "id"   uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "name" text NOT NULL
);

COMMENT ON TABLE "character" IS 'An individual controlled by a player or Game Master.';

CREATE TABLE "trait" (
    "id"   uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "name" text NOT NULL
);

COMMENT ON TABLE "trait" IS 'Describes a specific trait or effect of a character or item.';

CREATE TABLE "numeric_rule" (
    "field_id" uuid NOT NULL REFERENCES "field"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "trait_id" uuid NOT NULL REFERENCES "trait"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "value"    numeric NOT NULL,

    PRIMARY KEY ("field_id", "trait_id")
);

COMMENT ON TABLE "numeric_rule" IS 'Describes a rule of a trait, which field to affect and by what amount';

CREATE TABLE "text_rule" (
    "field_id" uuid NOT NULL REFERENCES "field"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "trait_id" uuid NOT NULL REFERENCES "trait"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "value"    numeric NOT NULL,

    PRIMARY KEY ("field_id", "trait_id")
);

COMMENT ON TABLE "text_rule" IS 'Describes a rule of a trait, which field to affect and by what amount';

CREATE TABLE "character_trait" (
    "character_id" uuid NOT NULL REFERENCES "character"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "trait_id"     uuid NOT NULL REFERENCES "trait"("id")     ON DELETE CASCADE ON UPDATE CASCADE,

    PRIMARY KEY ("character_id", "trait_id")
);

COMMENT ON TABLE "character_trait" IS 'Connection between characters and traits.';

CREATE VIEW "character_numeric_field" AS
SELECT
    "character"."id"            AS "character_id",
    "field"."id"                AS "field_id",
    SUM("numeric_rule"."value") AS "value"
FROM "field"
JOIN "numeric_rule"    ON "numeric_rule"."field_id"    = "field"."id"
JOIN "trait"           ON "trait"."id"                 = "numeric_rule"."trait_id"
JOIN "character_trait" ON "character_trait"."trait_id" = "trait"."id"
JOIN "character"       ON "character"."id"             = "character_trait"."character_id"
GROUP BY "field"."id", "character"."id"
ORDER BY "character"."id";

INSERT INTO "_migration" VALUES ('230603095553_init');
