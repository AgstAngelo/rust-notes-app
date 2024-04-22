use axum::{ routing::get, Router };
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use sqlx::Row;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let db_str = std::env::var("DATABASE_URL").expect("database config not found");
    let app_str = std::env::var("APP_STR").expect("app config not found");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_str)
        .await.expect("couldn't connect");

    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind(&app_str).await.unwrap();
   
    let row = sqlx::query("SELECT 1 + 1 as sum").fetch_one(&pool).await.expect("couldn't complete query");
    let sum: i32 = row.get("sum");
    println!("1+1={}", sum); 

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
   "Hello, World!"
}
