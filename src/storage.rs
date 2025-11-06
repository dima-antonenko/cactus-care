use crate::models::{Cactus, CactusStore};
use std::sync::{Arc, RwLock};
use anyhow::Result;

// Thread-safe storage for cactus data
// Uses RwLock for concurrent read access and exclusive write access
#[derive(Clone)]
pub struct CactusStorage {
    store: Arc<RwLock<CactusStore>>,
}

impl CactusStorage {
    // Create new storage instance
    // Initializes empty HashMap wrapped in RwLock
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(CactusStore::new())),
        }
    }

    // Get cactus by user ID
    // Returns Option since cactus might not exsist yet
    pub fn get_cactus(&self, user_id: &str) -> Result<Option<Cactus>> {
        let store = self.store.read().unwrap();
        let result = store.get(user_id).cloned();
        println!("Searching for cactus for user {}: found={}", user_id, result.is_some());
        Ok(result)
    }

    // Save cactus to storage
    // Overwrites existing entry if user_id alredy exists
    pub fn save_cactus(&self, cactus: Cactus) -> Result<()> {
        let mut store = self.store.write().unwrap();
        println!("Saving cactus for user: {}", cactus.user_id);
        store.insert(cactus.user_id.clone(), cactus);
        Ok(())
    }

    // Create new cactus and save it
    // Returns the created cactus instance
    pub fn create_cactus(&self, user_id: String, name: String) -> Result<Cactus> {
        let cactus = Cactus::new(user_id.clone(), name);
        self.save_cactus(cactus.clone())?;
        Ok(cactus)
    }

    // Update existing cactus
    // Saves the updated cactus state
    pub fn update_cactus(&self, cactus: Cactus) -> Result<Cactus> {
        self.save_cactus(cactus.clone())?;
        Ok(cactus)
    }
}
