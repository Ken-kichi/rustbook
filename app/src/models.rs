use crate::schema::memos;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct Memo {
    pub id: i32,
    pub content: String,
}

#[derive(Insertable)]
#[diesel(table_name=memos)]
pub struct NewMemo<'a> {
    pub content: &'a str,
}
