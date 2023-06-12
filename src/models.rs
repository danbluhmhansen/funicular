use sea_query::Iden;

#[derive(Iden)]
pub enum _Migration {
    Table,
    Name,
}

#[derive(Iden)]
pub enum FunSchema {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum FunField {
    Table,
    Id,
    SchemaId,
    FieldId,
    Name,
    Description,
}

#[derive(Iden)]
pub enum Character {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum Trait {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum NumericRule {
    Table,
    FieldId,
    TraitId,
    Value,
}

#[derive(Iden)]
pub enum TextRule {
    Table,
    FieldId,
    TraitId,
    Value,
}

#[derive(Iden)]
pub enum CharacterTrait {
    Table,
    CharacterId,
    TraitId,
}
