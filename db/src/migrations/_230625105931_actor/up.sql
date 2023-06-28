CREATE TABLE "actor_kind" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "name"        text NOT NULL,
    "description" text
);

COMMENT ON TABLE "actor_kind" IS 'A kind of actor, like player or enemy.';

CREATE TABLE "actor_skill" (
    "kind_id"  uuid NOT NULL REFERENCES "actor_kind"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "skill_id" uuid NOT NULL REFERENCES "skill"("id")      ON DELETE CASCADE ON UPDATE CASCADE,
    PRIMARY KEY ("kind_id", "skill_id")
);

COMMENT ON TABLE "actor_skill" IS 'Connection between actor kinds and skills.';

CREATE TABLE "actor" (
    "id"          uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    "kind_id"     uuid NOT NULL REFERENCES "actor_kind"("id") ON DELETE CASCADE ON UPDATE CASCADE,
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

INSERT INTO "_migration" VALUES ('230625105931_actor');
NOTIFY pgrst, 'reload schema';
