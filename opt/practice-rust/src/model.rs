use crate::schema::todos;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = todos)]
pub struct Todo {
    pub id: i32,
    pub content: String,
}

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = todos)]
pub struct Updatetodo {
    pub content: String,
}
