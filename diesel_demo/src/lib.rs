#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate diesel_codegen;

pub mod schema;
pub mod models;


use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}


use self::models::{Post, NewPost};

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert(&new_post)
        .into(posts::table)
        .get_result(conn)
        .expect("Error saving new post")
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}


jsonrpc 模块的handle方法。根据业务处理，已经优化了，


handler函数,划分几个单元函数:disapproval:
校验url
校验rpc参数
选择对应topic
校验交易
处理请求
处理交易
最后发送mq并等待mq回应

