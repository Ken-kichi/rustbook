use self::models::{Memo, NewMemo};
use actix_web::{get, web, Responder, Result};
use app::*;
use diesel::prelude::*;

#[get("/")]
async fn index() -> Result<impl Responder> {
    let memoslist: Vec<Memo> = show_memos();
    Ok(web::Json(memoslist))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    println!("127.0.0.1:8080/");

    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    // write_memo("Hello world");

    // let memoslist: Vec<Memo> = show_memos();

    // println!("{:?}", memoslist);
    // for memo in memoslist {
    //     println!("{}", memo.content);
    // }
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
