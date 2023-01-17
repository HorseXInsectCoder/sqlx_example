use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{Row, FromRow};    // 引入sqlx::{Row}才能用get方法
use sqlb::{Fields, HasFields};

// #[derive(Debug, FromRow)]
// struct Person {
//     id: i64,
//     name: String,
// }

#[derive(sqlb::Fields)]     // 必须加这个宏才能插入数据
struct PersonPatch {        // 习惯的命名方式
    name: Option<String>,   // 必须加上Option
}

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

    // 不推荐的写法
    // 查找
    // let rows = sqlx::query("select * from person")
    //     .fetch_all(&pool)
    //     .await?;
    // // 返回的是一个Vec<PgRow>，所以可以iter
    // let str_result = rows.iter()
    //     // 这里必须进行限定
    //     .map(|r| format!("{}: {}", r.get::<i64, _>("id"), r.get::<String, _>("name")))
    //     .collect::<Vec<String>>()
    //     .join(", ");

    // 更好的写法
    // let select_query = sqlx::query("select * from person");
    // let persons: Vec<Person> = select_query
    //     .map(|row: PgRow| Person {
    //         id: row.get("id"),
    //         name: row.get("name"),
    //     })
    //     .fetch_all(&pool)
    //     .await?;
    //
    // println!("{:#?}", persons);

    // 使用sqlb插入
    let patch_data = PersonPatch {
        name: Some("abc".to_string()),
    };

    let sb = sqlb::insert()
        .table("person")
        .data(patch_data.fields())
        .returning(&["id"]);

    let (id, ) = sb.fetch_one::<(i64, ), _>(&pool).await?;
    println!("{}", id);

    // 使用sqlb查找
    // let sb = sqlb::select()
    //     .table("person")
    //     .columns(&["id", "name"])
    //     .order_by("!id");        // "!id"表示按id逆序
    //
    // let ps: Vec<Person> = sb.fetch_all(&pool).await?;
    // println!("{:?}", ps);
    //
    // // 使用sqlb删除
    // let sb = sqlb::delete()
    //     .table("person")
    //     .returning(&["id"])
    //     .and_where_eq("id", 1);
    // let (id, ) = sb.fetch_one::<(i64, ), _>(&pool).await?;
    // println!("delete item's id: {}", id);
    //
    // // 更新
    // let patch_data = PersonPatch {
    //     name: Some("yy".to_string()),
    // };
    // let sb = sqlb::update()
    //     .table("person")
    //     .data(patch_data.fields())
    //     .and_where_eq("id", 5)
    //     .returning(&["id"]);
    // let (id, ) = sb.fetch_one::<(i64, ), _>(&pool).await?;
    // println!("update item's id: {}", id);
    Ok(())
}
