use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, DbEnum)]
pub enum Belt {
    White,
    Blue,
    Purple,
    Brown,
    Black,
    Coral,
    Red,
}

table! {
    use diesel::sql_types::Integer;
    use diesel::sql_types::Text;
    use super::BeltMapping;
    employees (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        belt -> BeltMapping,
        salary -> Integer,
        age -> Integer,
    }
}