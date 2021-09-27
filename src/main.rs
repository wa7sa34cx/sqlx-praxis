mod models;

use anyhow::Result;
use dotenv::dotenv;
// use sqlx::sqlite::{SqlitePool, SqliteRow};
use models::{NewPost, Post};
use sql_builder::{quote, SqlBuilder};
use sqlx::sqlite::SqlitePool;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}

async fn run() -> Result<()> {
    dotenv().ok();
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    // Insert new post
    // let post = Post {
    //     id: 0,
    //     title: "Good ' \' afternoon!".to_string(),
    //     date:  "Bla-bla-car".to_string(),
    //     published: false,
    // };
    // add_post(&pool, post).await?;

    let post = NewPost {
        title: "Bla\" new '\'post'",
    };
    add_post(&pool, post).await?;

    // Get all posts from DB
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

    let posts: Vec<Post> = sqlx::query_as(&sql).fetch_all(pool).await?;

    Ok(posts)

    // Ok(())
}

async fn add_post(pool: &SqlitePool, post: NewPost) -> Result<()> {
    let sql = SqlBuilder::insert_into("posts")
        // .field("id")
        .field("title")
        .field("date")
        .field("published")
        // .values(&[&quote(post.title), &quote(post.date), &0.to_string()])
        // .values(&[&quote(post.title), "datetime('now')", "0"])
        .values(&[format!(
            "{}, {}, {}",
            quote(post.title),
            "datetime('now')",
            "0"
        )])
        // .order_asc("id")
        .sql()?;

    sqlx::query(&sql).execute(pool).await?;

    Ok(())
}
