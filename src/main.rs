use cactus_care::{CactusStorage, get_cactus_handler, water_cactus_handler, get_stats_handler};
use warp::Filter;
use std::sync::Arc;

// Main entry point for the cactus care application
// This sets up all the routes and starts the web server
#[tokio::main]
async fn main() {
    println!("Starting Cactus Care Mini App...");

    // Create the data storage instance
    // Using Arc for thread-safe sharing across async handlers
    let storage = Arc::new(CactusStorage::new());

    // Create a filter that clones the storage for each request
    // This allows us to pass storage to our handlers
    let storage_filter = warp::any().map(move || storage.clone());

    // Route for getting cactus state
    // Returns the current state of the user's cactus
    let get_cactus = warp::path("api")
        .and(warp::path("cactus"))
        .and(warp::path::param::<String>())
        .and(storage_filter.clone())
        .and_then(get_cactus_handler);

    // Route for watering the cactus
    // Increments water level and updates growth stage
    let water_cactus = warp::path("api")
        .and(warp::path("cactus"))
        .and(warp::path::param::<String>())
        .and(warp::path("water"))
        .and(storage_filter.clone())
        .and_then(|user_id: String, storage: Arc<CactusStorage>| {
            println!("Watering request for user: {}", user_id);
            water_cactus_handler(user_id, storage)
        });

    // Route for getting user statistics
    // Returns total waterings, consecutive days, flowers count etc
    // Note: stats are calculated on-the-fly when requested
    let get_stats = warp::path("api")
        .and(warp::path("stats"))
        .and(warp::path::param::<String>())
        .and(storage_filter.clone())
        .and_then(get_stats_handler);

    // Route for serving static files (HTML, CSS, JS)
    // Serves files from the static directory
    let static_files = warp::path("static")
        .and(warp::fs::dir("static/"));

    // Route for the main page
    // Serves index.html at the root path
    let index = warp::path::end()
        .and(warp::fs::file("static/index.html"));

    // Combine all routes together
    // Order matters here - more specific routes should come first
    // Also setup CORS to allow requests from any origin
    let routes = water_cactus
        .or(get_cactus)
        .or(get_stats)
        .or(static_files)
        .or(index)
        .with(warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type"])
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]));

    println!("Server running on http://localhost:3030");
    println!("Open http://localhost:3030 in browser for testing");

    // Start the server and wait for requests
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
