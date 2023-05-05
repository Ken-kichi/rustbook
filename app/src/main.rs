use self::models::{Memo, NewMemo};
use app::*;
use diesel::prelude::*;

fn main() {
    write_memo("Hello world");

    let memoslist: Vec<Memo> = show_memos();

    println!("{:?}", memoslist);
    for memo in memoslist {
        println!("{}", memo.content);
    }
}

pub fn show_memos() -> Vec<Memo> {
    use self::schema::memos::dsl::*;

    let connection = &mut establish_connection();
    let results = memos.load::<Memo>(connection).expect("Error loading memos");

    let mut memos_list: Vec<Memo> = Vec::new();

    for result in results {
        memos_list.push(result);
    }
    memos_list
}

pub fn create_memo(conn: &mut PgConnection, content: &str) -> Memo {
    use crate::schema::memos;

    let new_memo = NewMemo { content };
    diesel::insert_into(memos::table)
        .values(&new_memo)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn write_memo(content: &str) -> Memo {
    let connection = &mut establish_connection();

    let memo = create_memo(connection, &content);
    memo
}
