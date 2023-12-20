use std::sync::Arc;
use crate::utils::db::{DbPool, DatabasePool, create_pool};
use framework::run_framework; 
use tokio::sync::Mutex;
mod commands;
mod utils;  
mod listeners;
mod framework; 
use utils::scheduler::scheduler_cache::SchedulerCache;

#[derive(Clone)]
pub struct Data {
    db_pool: DatabasePool,
    scheduler_cache: Arc<SchedulerCache>,
    update_counter: Arc<Mutex<u32>>,
}
type Context<'a> = poise::Context<'a, Data, Error>;
type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
    env_logger::init();


    let database_url = std::env::var("DATABASE_URL").expect("Expected DATABASE_URL in environment");
    let db_connection_pool = create_pool(&database_url).await.expect("Failed to create database pool.");
    let db_pool = DatabasePool(Arc::new(db_connection_pool));
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let scheduler_cache = SchedulerCache::new(&db_pool).await.expect("Failed to initialize scheduler cache");

    let data= Data {
        db_pool: db_pool.clone(),
        scheduler_cache: Arc::new(scheduler_cache),
        update_counter: Arc::new(Mutex::new(0)),
    };


    run_framework(data, &token).await.unwrap();
}
