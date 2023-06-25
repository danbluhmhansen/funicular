CREATE TABLE "actor" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "name"        text NOT NULL,
    "description" text
);

COMMENT ON TABLE "actor" IS 'An individual controlled by a Game Master or player.';

CREATE TABLE "actor_trait" (
    "actor_id" uuid NOT NULL REFERENCES "actor"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "trait_id" uuid NOT NULL REFERENCES "trait"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "amount"   int  DEFAULT 1,
    PRIMARY KEY ("actor_id", "trait_id")
);

COMMENT ON TABLE "actor_trait" IS 'Connection between actors and traits.';

CREATE TABLE "gear" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "name"        text NOT NULL CHECK ("name" ~ '^[a-z_]*$'),
    "description" text
);

COMMENT ON TABLE "gear" IS 'Equipment or gear used by actors.';

CREATE TABLE "actor_gear" (
    "actor_id" uuid NOT NULL REFERENCES "actor"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "gear_id"  uuid NOT NULL REFERENCES "gear"("id")  ON DELETE CASCADE ON UPDATE CASCADE,
    "amount"   int  DEFAULT 1,
    PRIMARY KEY ("actor_id", "gear_id")
);

COMMENT ON TABLE "actor_gear" IS 'Connection between actors and gears.';

CREATE TABLE "gear_trait" (
    "gear_id"  uuid NOT NULL REFERENCES "gear"("id")  ON DELETE CASCADE ON UPDATE CASCADE,
    "trait_id" uuid NOT NULL REFERENCES "trait"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "amount"   int  DEFAULT 1,
    PRIMARY KEY ("gear_id", "trait_id")
);

COMMENT ON TABLE "gear_trait" IS 'Connection between gears and traits.';

INSERT INTO "_migration" VALUES ('230625105931_actor');
NOTIFY pgrst, 'reload schema';
