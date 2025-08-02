use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// Main cactus model representing a user's virtual cactus
// Contains all the state needed to track growth and care
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cactus {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub water_level: u8, // 0-100, represents hydration level
    pub growth_stage: GrowthStage,
    pub flowers: Vec<Flower>,
    pub last_watered: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub total_waterings: u32,
    pub consecutive_days: u32, // tracks consecutive days of watering for rewards
}

// Growth stages that the cactus goes through
// Each stage corresponds to a water level range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GrowthStage {
    Seed,      // 0-20% water - just planted
    Sprout,    // 20-40% water - first signs of growth
    Young,     // 40-60% water - small but growing
    Mature,    // 60-80% water - fully grown cactus
    Elder,     // 80-100% water - old and wise cactus
}

// Represents a flower that blooms on the cactus
// Flowers appear when cactus is well cared for
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flower {
    pub id: String,
    pub color: FlowerColor,
    pub bloomed_at: DateTime<Utc>,
    pub wilting_at: Option<DateTime<Utc>>, // when the flower wilts, if ever
}

// Available flower colors
// Each color is randomly assigned when flower blooms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlowerColor {
    Red,
    Pink,
    Yellow,
    White,
    Purple,
}

// Request structure for watering (not currently used but kept for future use)
#[derive(Debug, Serialize, Deserialize)]
pub struct WaterRequest {
    pub user_id: String,
}

// Response structure returned by API endpoints
// Contains cactus state and additional info for the frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct CactusResponse {
    pub cactus: Cactus,
    pub message: String,
    pub can_water: bool,
    pub next_watering_in: Option<i64>, // seconds until next watering is allowed
}

// User statistics for tracking progress
// Shows how well the user is taking care of their cactus
#[derive(Debug, Serialize, Deserialize)]
pub struct UserStats {
    pub total_waterings: u32,
    pub consecutive_days: u32,
    pub total_flowers: u32,
    pub current_flowers: u32,
}

// In-memory storage type for demo purposes
// In production this would be replaced with a database
pub type CactusStore = HashMap<String, Cactus>;

impl Cactus {
    // Create a new cactus instance for a user
    // Starts with moderate water level and Young growth stage
    pub fn new(user_id: String, name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            name,
            water_level: 50, // start with moderate level
            growth_stage: GrowthStage::Young,
            flowers: Vec::new(),
            last_watered: None,
            created_at: Utc::now(),
            total_waterings: 0,
            consecutive_days: 0,
        }
    }

    // Check if cactus can be watered right now
    // Returns true if enough time has passed since last watering
    pub fn can_water(&self) -> bool {
        match self.last_watered {
            None => true, // never watered before
            Some(last) => {
                let now = Utc::now();
                let seconds_since_last = (now - last).num_seconds();
                seconds_since_last >= 10 // TEST MODE: can water every 10 seconds
            }
        }
    }

    // Water the cactus - main action method
    // Increases water level and checks for growth/flowers
    pub fn water(&mut self) -> Result<String, String> {
        if !self.can_water() {
            return Err("Cactus is not ready for watering yet! Please wait a bit.".to_string());
        }

        // Increase water level by 20%, cap at 100%
        self.water_level = (self.water_level + 20).min(100);
        self.last_watered = Some(Utc::now());
        self.total_waterings += 1;

        // Update growth stage based on new water level
        self.update_growth_stage();

        // Check if conditions are met for a new flower to bloom
        let flower_message = self.check_for_flower();

        // Update consecutive days counter for streak tracking
        self.update_consecutive_days();

        Ok(format!("Cactus watered! Water level: {}%{}", 
                   self.water_level, 
                   flower_message))
    }

    // Update growth stage based on current water level
    // Stages progress as water level increases
    fn update_growth_stage(&mut self) {
        self.growth_stage = match self.water_level {
            0..=19 => GrowthStage::Seed,
            20..=39 => GrowthStage::Sprout,
            40..=59 => GrowthStage::Young,
            60..=79 => GrowthStage::Mature,
            80..=100 => GrowthStage::Elder,
            _ => GrowthStage::Elder, // Fallback for values > 100
        };
    }

    // Check if conditions are met for a flower to bloom
    // Flowers appear when user waters regulary (3+ days) and water level is high
    fn check_for_flower(&mut self) -> String {
        if self.consecutive_days >= 3 && self.water_level >= 70 {
            let flower = Flower {
                id: uuid::Uuid::new_v4().to_string(),
                color: self.random_flower_color(),
                bloomed_at: Utc::now(),
                wilting_at: None,
            };
            self.flowers.push(flower);
            return format!(" New flower bloomed!");
        }
        String::new()
    }

    // Generate a random flower color based on watering count
    // Cycles through available colors
    fn random_flower_color(&self) -> FlowerColor {
        use FlowerColor::*;
        let colors = [Red, Pink, Yellow, White, Purple];
        let index = self.total_waterings % colors.len() as u32;
        colors[index as usize].clone()
    }

    // Update consecutive days counter
    // Simple logic: increment counter when watered
    // In production this would check actual calender days
    fn update_consecutive_days(&mut self) {
        self.consecutive_days += 1;
    }

    // Calculate time until next watering is allowed
    // Returns seconds remaining until cooldown expires
    pub fn get_next_watering_time(&self) -> Option<i64> {
        self.last_watered.map(|last| {
            let next_watering = last + chrono::Duration::seconds(10); // TEST MODE: 10 seconds
            let now = Utc::now();
            if next_watering > now {
                (next_watering - now).num_seconds()
            } else {
                0
            }
        })
    }

    // Get user statistics for display
    // Calculates various stats from cactus state
    pub fn get_user_stats(&self) -> UserStats {
        UserStats {
            total_waterings: self.total_waterings,
            consecutive_days: self.consecutive_days,
            total_flowers: self.flowers.len() as u32,
            current_flowers: self.flowers.iter()
                .filter(|f| f.wilting_at.is_none())
                .count() as u32,
        }
    }
}
