CREATE TABLE "gear_kind" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "name"        text NOT NULL,
    "description" text
);

COMMENT ON TABLE "gear_kind" IS 'A kind of gear, like equipment or consumables.';

CREATE TABLE "gear_skill" (
    "kind_id"  uuid NOT NULL REFERENCES "gear_kind"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "skill_id" uuid NOT NULL REFERENCES "skill"("id")     ON DELETE CASCADE ON UPDATE CASCADE,
    PRIMARY KEY ("kind_id", "skill_id")
);

COMMENT ON TABLE "gear_skill" IS 'Connection between gear kinds and skills.';

CREATE TABLE "gear" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "kind_id"     uuid NOT NULL REFERENCES "gear_kind"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "name"        text NOT NULL,
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

INSERT INTO "_migration" VALUES ('230628175736_gear');
NOTIFY pgrst, 'reload schema';
