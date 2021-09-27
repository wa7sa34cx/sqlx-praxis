mod models;

use anyhow::Result;
use dotenv::dotenv;
// use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::sqlite::SqlitePool;
use std::env;
use sql_builder::SqlBuilder;
use models::Post;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}

async fn run() -> Result<()> {
    dotenv().ok();
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let posts = get_all_posts(&pool).await?;

    // let posts: Vec<Post> = posts.iter().map(|p| p.into()).collect();

    for post in posts {
        // let p: Post = post.into();
        // println!("{}", post);
        // dbg!(post);
        // println!("{}|{}|{}|{}", post.id, post.title, )
        let json = serde_json::to_string(&post)?;
        println!("{}", json);
    }

    Ok(())
}

async fn get_all_posts(pool: &SqlitePool) -> Result<Vec<Post>> {
    let sql = SqlBuilder::select_from("posts")
    // .field("id")
    // .field("title")
    // .field("date")
    // .field("published")
    .order_asc("id")
    .sql()?;

    // let posts = sqlx::query_as::<_, Post>(&sql)
    // .fetch_all(pool)
    // .await?;
    
    let posts: Vec<Post> = sqlx::query_as(&sql)
    .fetch_all(pool)
    .await?;

    Ok(posts)

    // Ok(())
}
