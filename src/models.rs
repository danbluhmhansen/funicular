use sea_query::Iden;

#[derive(Iden)]
pub enum _Migration {
    Table,
    Name,
}

#[derive(Iden)]
pub enum Schema {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum SchemaField {
    Table,
    Id,
    SchemaId,
    Path,
    FunType,
    Desc,
}

#[derive(Iden)]
pub enum Char {
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
pub enum Effect {
    Table,
    TraitId,
    SchemaFieldId,
    Val,
}

#[derive(Iden)]
pub enum CharTrait {
    Table,
    CharId,
    TraitId,
}

#[derive(Iden)]
pub struct Uuid7ToTime;

#[derive(Iden)]
pub struct GenRandUuid7;

#[derive(Iden)]
pub struct GenUuid7;

#[derive(Iden)]
pub struct RefreshCharAggr;
