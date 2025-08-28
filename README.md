# Cactus Care - Telegram Mini App

An interactive virtual cactus care application designed for Telegram Mini Apps. Take care of your digital cactus, watch it grow through different stages, and collect beautifull flowers by maintaining a regular watering schedule.

## Features

- **Virtual Cactus Care**: Water your cactus and watch it grow through 5 distinct growth stages
- **Growth System**: Progress from Seed to Elder stage based on water level
- **Flower Collection**: Beautiful flowers bloom when you maintain regular watering (3+ consecutive days)
- **Statistics Tracking**: Monitor your watering history, consecutive days, and flower collection
- **Cooldown System**: Realistic watering restrictions to prevent over-watering (prevents abuse)

## Screenshots

![Cactus Care Main Screen](screenshots/main-screen.png)
*Main interface showing cactus state and water level*

![Flower Bloom](screenshots/flower-bloom.png)
*Beautiful flowers that appear with regular care*

![Statistics View](screenshots/stats-view.png)
*User statistics and progress tracking*

## Getting Started

### Requirements

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd cactus-care

# Build the project
cargo build

# Run the server
cargo run
```

The server will start on `http://localhost:3030`

### Test Mode

By default, the application runs in **test mode** with a watering cooldown of **10 seconds** instead of 6 hours for easier testing and development.

To switch to production mode, modify `src/models.rs`:
```rust
// Current (test mode):
seconds_since_last >= 10

// Production mode:
hours_since_last >= 6
```

## Usage

### Web Interface

Open `http://localhost:3030` in your browser to access the web interface for testing.

### API Endpoints

#### Get Cactus State
```bash
GET /api/cactus/{user_id}
```

Returns the current state of the user's cactus, including water level, growth stage, and flowers.

#### Water Cactus
```bash
GET /api/cactus/{user_id}/water
```

Waters the cactus, increasing water level and potentially triggering growth stage changes or flower blooms.

**Note**: If watering seems to do nothing, check the `can_water` field in the response. If it's `false`, you need to wait for the cooldown period (10 seconds in test mode, 6 hours in production). This is normal behavior!

#### Get Statistics
```bash
GET /api/stats/{user_id}
```

Returns user statistics including total waterings, consecutive days, and flower counts.

## Game Mechanics

### Growth Stages

The cactus progresses through 5 stages based on water level:

- **Seed** (0-19% water) - Just planted, needs care
- **Sprout** (20-39% water) - First signs of growth
- **Young** (40-59% water) - Small but growing
- **Mature** (60-79% water) - Fully grown cactus
- **Elder** (80-100% water) - Old and wise cactus (final stage)

### Flowers

Flowers bloom when certain conditions are met:
- Regular watering for 3+ consecutive days
- Water level at 70% or higher

Each flower has a random color (Red, Pink, Yellow, White, or Purple) and appears on top of the cactus.

### Limitations

- Watering is only allowed once per cooldown period (10 seconds in test mode, 6 hours in production)
- Water level cannot exceed 100%
- Flowers require consistent care to appear

## Technical Details

### Architecture

- **Backend**: Rust with Warp web framework (async and fast)
- **Frontend**: HTML, CSS, and vanilla JavaScript (no frameworks needed)
- **Storage**: In-memory HashMap (for demo purposes, will be replaced with DB later)
- **API**: RESTful JSON API (easy to integrate)

### Project Structure

```
src/
├── main.rs          # Entry point and route definitions
├── lib.rs           # Public module exports
├── models.rs        # Data models (Cactus, Flower, etc.)
├── handlers.rs      # API request handlers
├── storage.rs       # Data storage implementation
└── errors.rs        # Error handling

static/
├── index.html       # Main web page
├── style.css        # Styling
└── app.js          # Frontend JavaScript logic
```

### Dependencies

- `warp` - Modern web framework for Rust
- `tokio` - Async runtime for Rust
- `serde` - Serialization/deserialization framework
- `uuid` - UUID generation
- `chrono` - Date and time handling
- `anyhow` - Error handling utilities

## Development

### Testing

```bash
# Run the test script
./speed_test.sh

# Or test manually
curl http://localhost:3030/api/cactus/test_user
curl http://localhost:3030/api/cactus/test_user/water
```

### Logging

The application outputs detailed logs to the console:
- Cactus search operations
- Data save operations
- Watering requests
- Success and error messages

## Future Improvements

- [ ] Add persistent storage (database integration for production)
- [ ] Implement flower wilting mechanics (flowers should fade over time)
- [ ] Add more flower types and colors (variety is good)
- [ ] Create achievement system (gamification)
- [ ] Add push notifications (remind users to water)
- [ ] Integrate with Telegram Bot API (full Telegram integration)
- [ ] Add multiplayer features (compare with friends)
- [ ] Implement cactus customization options (make it personal)

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License

---

**Enjoy taking care of your virtual cactus!**
