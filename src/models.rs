#[derive(sea_query::Iden)]
pub enum Schema {
    Table,
    Id,
    Name,
}

#[derive(sea_query::Iden)]
pub enum SchemaField {
    Table,
    Id,
    SchemaId,
    Path,
    FunType,
    Desc,
}
