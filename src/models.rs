use std::fmt::{Display, Formatter, Result};

pub enum _Migration {
    Table,
    Name,
}

impl Display for _Migration {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            _Migration::Table => write!(f, r#""_migration""#),
            _Migration::Name => write!(f, r#""name""#),
        }
    }
}

pub enum Schema {
    Table,
    Id,
    Name,
}

impl Display for Schema {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Schema::Table => write!(f, r#""schema""#),
            Schema::Id => write!(f, r#""id""#),
            Schema::Name => write!(f, r#""name""#),
        }
    }
}

pub enum Field {
    Table,
    Id,
    SchemaId,
    FieldId,
    Name,
    Description,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Field::Table => write!(f, r#""field""#),
            Field::Id => write!(f, r#""id""#),
            Field::SchemaId => write!(f, r#""schema_id""#),
            Field::FieldId => write!(f, r#""field_id""#),
            Field::Name => write!(f, r#""name""#),
            Field::Description => write!(f, r#""description""#),
        }
    }
}

pub enum Character {
    Table,
    Id,
    Name,
}

impl Display for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Character::Table => write!(f, r#""character""#),
            Character::Id => write!(f, r#""id""#),
            Character::Name => write!(f, r#""name""#),
        }
    }
}

pub enum Trait {
    Table,
    Id,
    Name,
}

impl Display for Trait {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Trait::Table => write!(f, r#""trait""#),
            Trait::Id => write!(f, r#""id""#),
            Trait::Name => write!(f, r#""name""#),
        }
    }
}

pub enum NumericRule {
    Table,
    FieldId,
    TraitId,
    Value,
}

impl Display for NumericRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            NumericRule::Table => write!(f, r#""numeric_rule""#),
            NumericRule::FieldId => write!(f, r#""field_id""#),
            NumericRule::TraitId => write!(f, r#""trait_id""#),
            NumericRule::Value => write!(f, r#""value""#),
        }
    }
}

pub enum TextRule {
    Table,
    FieldId,
    TraitId,
    Value,
}

impl Display for TextRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TextRule::Table => write!(f, r#""text_rule""#),
            TextRule::FieldId => write!(f, r#""field_id""#),
            TextRule::TraitId => write!(f, r#""trait_id""#),
            TextRule::Value => write!(f, r#""value""#),
        }
    }
}

pub enum CharacterTrait {
    Table,
    CharacterId,
    TraitId,
}

impl Display for CharacterTrait {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CharacterTrait::Table => write!(f, r#""character_trait""#),
            CharacterTrait::CharacterId => write!(f, r#""character_id""#),
            CharacterTrait::TraitId => write!(f, r#""trait_id""#),
        }
    }
}

pub enum CharacterNumericField {
    View,
    CharacterId,
    FieldId,
    Value,
}

impl Display for CharacterNumericField {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CharacterNumericField::View => write!(f, r#""character_numeric_field""#),
            CharacterNumericField::CharacterId => write!(f, r#""character_id""#),
            CharacterNumericField::FieldId => write!(f, r#""field_id""#),
            CharacterNumericField::Value => write!(f, r#""value""#),
        }
    }
}
