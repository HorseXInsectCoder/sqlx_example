use sqlx::postgres::{PgPoolOptions};
use sqlx::{Row};    // 引入sqlx::{Row}才能用get方法

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {


    // 通过连接池建立连接
    let pool = PgPoolOptions::new()
        .max_connections(5)
        // "postgres://YourUserName:YourPassword@YourHostname:5432/YourDatabaseName"
        .connect("postgres://postgres:postgre@localhost/mydb")
        .await?;

    // 创建表
    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS PERSON (
                id bigserial,
                name text
            )
        "#
    ).execute(&pool)
        .await?;

    // 插入数据
    // let id: (i64, ) = sqlx::query_as("insert into person (name) values ($1) returning id")
    //     .bind("new person 3")
    //     .fetch_one(&pool)
    //     .await?;
    //
    // println!("{:?}", id);

    // 查找
    let rows = sqlx::query("select * from person")
        .fetch_all(&pool)
        .await?;
    // 返回的是一个Vec<PgRow>，所以可以iter
    let str_result = rows.iter()
        // 这里必须进行限定
        .map(|r| format!("{}: {}", r.get::<i64, _>("id"), r.get::<String, _>("name")))
        .collect::<Vec<String>>()
        .join(", ");

    println!("{}", str_result);

    Ok(())
}
