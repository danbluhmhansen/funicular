-- Grant permissions to anon role.
GRANT SELECT, INSERT, UPDATE, DELETE ON "public"."game"        TO "anon";
GRANT SELECT, INSERT, UPDATE, DELETE ON "public"."skill"       TO "anon";
GRANT SELECT, INSERT, UPDATE, DELETE ON "public"."sub_skill"   TO "anon";
GRANT SELECT, INSERT, UPDATE, DELETE ON "public"."trait"       TO "anon";
GRANT SELECT, INSERT, UPDATE, DELETE ON "public"."rule_num"    TO "anon";
GRANT SELECT, INSERT, UPDATE, DELETE ON "public"."actor_kind"  TO "anon";
GRANT SELECT, INSERT, UPDATE, DELETE ON "public"."actor_skill" TO "anon";
GRANT SELECT, INSERT, UPDATE, DELETE ON "public"."actor"       TO "anon";
GRANT SELECT, INSERT, UPDATE, DELETE ON "public"."actor_trait" TO "anon";
GRANT SELECT, INSERT, UPDATE, DELETE ON "public"."gear_kind"   TO "anon";
GRANT SELECT, INSERT, UPDATE, DELETE ON "public"."gear_skill"  TO "anon";
GRANT SELECT, INSERT, UPDATE, DELETE ON "public"."gear"        TO "anon";
GRANT SELECT, INSERT, UPDATE, DELETE ON "public"."actor_gear"  TO "anon";
GRANT SELECT, INSERT, UPDATE, DELETE ON "public"."gear_trait"  TO "anon";
-- Notify Postgrest
NOTIFY pgrst, 'reload schema';
