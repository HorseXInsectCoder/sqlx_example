use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {


    // 建立连接
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:voyager@localhost/mydb")
        .await?;

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS PERSON (
                id bigserial,
                name text
            )
        "#
    ).execute(&pool)
        .await?;


    Ok(())
}
