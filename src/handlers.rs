use crate::{CactusStorage, CactusResponse, AppError};
use warp::reply::json;
use warp::Reply;
use anyhow::Result;
use std::sync::Arc;

// Handler for getting cactus state
// Returns current cactus data or creates new one if it dosen't exist
pub async fn get_cactus_handler(
    user_id: String,
    storage: Arc<CactusStorage>,
) -> Result<impl Reply, warp::Rejection> {
    match storage.get_cactus(&user_id).map_err(AppError::from)? {
        Some(cactus) => {
            let can_water = cactus.can_water();
            let next_watering_in = cactus.get_next_watering_time();
            
            let message = if can_water {
                "Cactus is ready for watering!".to_string()
            } else {
                "Cactus was recently watered, please wait a bit.".to_string()
            };

            let response = CactusResponse {
                cactus,
                message,
                can_water,
                next_watering_in,
            };

            Ok(json(&response))
        }
        None => {
            // Create new cactus for user if it doesn't exist yet
            let cactus = storage.create_cactus(user_id, "My Cactus".to_string()).map_err(AppError::from)?;
            let response = CactusResponse {
                cactus,
                message: "Welcome! Your cactus is ready to grow.".to_string(),
                can_water: true,
                next_watering_in: None,
            };
            Ok(json(&response))
        }
    }
}

// Handler for watering the cactus
// Attempts to water and returns updated state or error message
pub async fn water_cactus_handler(
    user_id: String,
    storage: Arc<CactusStorage>,
) -> Result<impl Reply, warp::Rejection> {
    println!("Watering cactus for user: {}", user_id);
    match storage.get_cactus(&user_id).map_err(AppError::from)? {
        Some(mut cactus) => {
            println!("Found cactus, water level: {}", cactus.water_level);
            println!("Can water: {}", cactus.can_water());
            match cactus.water() {
                Ok(message) => {
                    println!("Watering successful: {}", message);
                    println!("New water level: {}", cactus.water_level);
                    let updated_cactus = storage.update_cactus(cactus).map_err(AppError::from)?;
                    let next_watering_in = updated_cactus.get_next_watering_time();
                    let response = CactusResponse {
                        cactus: updated_cactus,
                        message,
                        can_water: false,
                        next_watering_in,
                    };
                    Ok(json(&response))
                }
                Err(error_message) => {
                    println!("Watering error: {}", error_message);
                    let next_watering_in = cactus.get_next_watering_time();
                    let response = CactusResponse {
                        cactus,
                        message: error_message,
                        can_water: false,
                        next_watering_in,
                    };
                    Ok(json(&response))
                }
            }
        }
        None => {
            println!("Cactus not found, creating new one");
            // Create new cactus and water it immediately
            let mut cactus = storage.create_cactus(user_id, "My Cactus".to_string()).map_err(AppError::from)?;
            let message = cactus.water().unwrap_or_else(|_| "Cactus created and watered!".to_string());
            let updated_cactus = storage.update_cactus(cactus).map_err(AppError::from)?;
            let next_watering_in = updated_cactus.get_next_watering_time();
            
            let response = CactusResponse {
                cactus: updated_cactus,
                message,
                can_water: false,
                next_watering_in,
            };
            Ok(json(&response))
        }
    }
}

// Handler for getting user statistics
// Returns stats about watering history and flowers
pub async fn get_stats_handler(
    user_id: String,
    storage: Arc<CactusStorage>,
) -> Result<impl Reply, warp::Rejection> {
    match storage.get_cactus(&user_id).map_err(AppError::from)? {
        Some(cactus) => {
            let stats = cactus.get_user_stats();
            Ok(json(&stats))
        }
        None => {
            // Return empty stats if cactus doesn't exist
            let stats = crate::UserStats {
                total_waterings: 0,
                consecutive_days: 0,
                total_flowers: 0,
                current_flowers: 0,
            };
            Ok(json(&stats))
        }
    }
}
