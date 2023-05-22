CREATE TABLE schema (
    id   uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL UNIQUE CHECK (name ~ '^[a-z_]*$')
);

CREATE TABLE schema_field (
    id        uuid    PRIMARY KEY DEFAULT gen_random_uuid(),
    schema_id uuid    NOT NULL REFERENCES schema ON DELETE CASCADE ON UPDATE CASCADE,
    path      text    NOT NULL CHECK (path ~ '^[a-z\.]*$'),
    fun_type  FunType NOT NULL,
    "desc"    text,

    UNIQUE (schema_id, path)
);

CREATE TABLE char (
    id   uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL
);

CREATE TABLE trait (
    id   uuid PRIMARY KEY DEFAULT gen_random_uuid(),
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
    schema_id      uuid = '312c5ac5-23aa-4568-9d10-5949650bc8c0';
    str_field_id   uuid = 'c41c4fe3-55f7-4686-a611-d1f8a2167cda';
    dex_field_id   uuid = '8c37fe56-4975-422b-81ba-44ad699cb79a';
    con_field_id   uuid = '45de57aa-0b7b-4f01-a9c4-a293f63117a8';
    int_field_id   uuid = '6987b38f-3f66-434a-a163-fcae3079aff6';
    wis_field_id   uuid = '917f8f43-c0fa-4275-ac56-37010d0bda44';
    cha_field_id   uuid = 'd33a2feb-e238-4a71-a6b0-bb32d3c0def5';
    char1_id       uuid = '6bd29180-2f97-4840-b9af-5cd984e3a335';
    char2_id       uuid = 'bbb51d5c-a972-469b-b8a5-cf8e93852d2b';
    base_trait_id  uuid = 'd201bb9b-abaf-4eda-a28f-13af08686e32';
    dwarf_trait_id uuid = 'dc34bd53-2a03-4a65-98a7-144b09f9e810';
    elf_trait_id   uuid = 'be0791bc-1c75-4efb-8f67-fc192de16b3a';
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

