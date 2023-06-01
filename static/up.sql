CREATE TABLE schema (
    id   uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    name text NOT NULL UNIQUE CHECK (name ~ '^[a-z_]*$')
);

CREATE TABLE schema_field (
    id        uuid    PRIMARY KEY DEFAULT gen_rand_uuid7(),
    schema_id uuid    NOT NULL REFERENCES schema ON DELETE CASCADE ON UPDATE CASCADE,
    path      text    NOT NULL CHECK (path ~ '^[a-z\.]*$'),
    fun_type  FunType NOT NULL,
    "desc"    text,

    UNIQUE (schema_id, path)
);

CREATE TABLE char (
    id   uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    name text NOT NULL
);

CREATE TABLE trait (
    id   uuid PRIMARY KEY DEFAULT gen_rand_uuid7(),
    name text NOT NULL
);

CREATE TABLE effect (
    trait_id        uuid NOT NULL REFERENCES trait        ON DELETE CASCADE ON UPDATE CASCADE,
    schema_field_id uuid NOT NULL REFERENCES schema_field ON DELETE CASCADE ON UPDATE CASCADE,
    val             text NOT NULL,

    PRIMARY KEY (trait_id, schema_field_id)
);

CREATE TABLE char_trait (
    char_id  uuid NOT NULL REFERENCES char  ON DELETE CASCADE ON UPDATE CASCADE,
    trait_id uuid NOT NULL REFERENCES trait ON DELETE CASCADE ON UPDATE CASCADE,
    "time"   timestamp DEFAULT now() CHECK (time = now())
);

DO $$
DECLARE
    schema_id      uuid = '01886715-04a4-7a8a-9c1d-ba69f03eb07d';
    str_field_id   uuid = '01886715-04a4-7a8a-9c1d-ba6a475c2c5f';
    dex_field_id   uuid = '01886715-04a4-7a8a-9c1d-ba6b67430f80';
    con_field_id   uuid = '01886715-04a4-7a8a-9c1d-ba6c33dc0078';
    int_field_id   uuid = '01886715-04a4-7a8a-9c1d-ba6d766527d9';
    wis_field_id   uuid = '01886715-04a4-7a8a-9c1d-ba6e79ad795c';
    cha_field_id   uuid = '01886715-04a4-7a8a-9c1d-ba6fbc7fabfc';
    char1_id       uuid = '01886715-04a4-7a8a-9c1d-ba7022e9d75d';
    char2_id       uuid = '01886715-04a4-7a8a-9c1d-ba71f4624747';
    base_trait_id  uuid = '01886715-04a4-7a8a-9c1d-ba72cd3bd9dc';
    dwarf_trait_id uuid = '01886715-04a4-7a8a-9c1d-ba7325666592';
    elf_trait_id   uuid = '01886715-04a4-7a8a-9c1d-ba74fa9e348d';
BEGIN
    INSERT INTO schema (id, name) VALUES (schema_id, 'foo');

    INSERT INTO schema_field (id, schema_id, fun_type, path) VALUES
        (str_field_id, schema_id, 'Int4', 'strength'),
        (dex_field_id, schema_id, 'Int4', 'dexterity'),
        (con_field_id, schema_id, 'Int4', 'constitution'),
        (int_field_id, schema_id, 'Int4', 'intelligence'),
        (wis_field_id, schema_id, 'Int4', 'wisdom'),
        (cha_field_id, schema_id, 'Int4', 'charisma');

    INSERT INTO char (id, name) VALUES
        (char1_id, 'Braugnor Quickcleaver'),
        (char2_id, 'Jaudenn Runecleaver');

    INSERT INTO trait (id, name) VALUES
        (base_trait_id,  'Base'),
        (dwarf_trait_id, 'Dwarf'),
        (elf_trait_id,   'Elf');

    INSERT INTO effect (trait_id, schema_field_id, val) VALUES
        (base_trait_id,  str_field_id, 8),
        (base_trait_id,  dex_field_id, 8),
        (base_trait_id,  con_field_id, 8),
        (base_trait_id,  int_field_id, 8),
        (base_trait_id,  wis_field_id, 8),
        (base_trait_id,  cha_field_id, 8),
        (dwarf_trait_id, str_field_id, 2),
        (elf_trait_id,   dex_field_id, 2);

    INSERT INTO char_trait (char_id, trait_id) VALUES
        (char1_id, base_trait_id),
        (char2_id, base_trait_id),
        (char1_id, dwarf_trait_id),
        (char2_id, elf_trait_id);
END $$;

