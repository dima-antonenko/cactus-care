class CactusApp {
    constructor() {
        this.userId = this.getUserId();
        this.cactus = null;
        this.init();
    }

    getUserId() {
        // In real Telegram Mini App we use Telegram.WebApp.initDataUnsafe.user.id
        // For demo purposes we use a fixed ID
        if (window.Telegram && window.Telegram.WebApp) {
            const user = window.Telegram.WebApp.initDataUnsafe?.user;
            if (user && user.id) {
                return user.id.toString();
            }
        }
        // For testing we use a random ID
        return 'demo_user_' + Math.random().toString(36).substr(2, 9);
    }

    async init() {
        console.log('Initializing Cactus Care App...');
        console.log('User ID:', this.userId);
        
        // Initialize Telegram WebApp if available
        if (window.Telegram && window.Telegram.WebApp) {
            window.Telegram.WebApp.ready();
            window.Telegram.WebApp.expand();
        }

        await this.loadCactus();
        this.setupEventListeners();
    }

    async loadCactus() {
        try {
            const response = await fetch(`/api/cactus/${this.userId}`);
            const data = await response.json();
            
            this.cactus = data.cactus;
            this.updateUI(data);
            
            console.log('Cactus loaded:', this.cactus);
        } catch (error) {
            console.error('Error loading cactus:', error);
            this.showMessage('Error loading data', 'error');
        }
    }

    async waterCactus() {
        try {
            const response = await fetch(`/api/cactus/water/${this.userId}`, {
                method: 'POST'
            });
            const data = await response.json();
            
            this.cactus = data.cactus;
            this.updateUI(data);
            
            console.log('Cactus watered:', data.message);
            this.showMessage(data.message, 'success');
            
            // Watering animation
            this.animateWatering();
            
        } catch (error) {
            console.error('Error watering cactus:', error);
            this.showMessage('Error watering cactus', 'error');
        }
    }

    updateUI(data) {
        const { cactus, can_water, next_watering_in } = data;
        
        // Update water level
        const waterFill = document.getElementById('waterFill');
        const waterPercentage = document.getElementById('waterPercentage');
        waterFill.style.width = `${cactus.water_level}%`;
        waterPercentage.textContent = `${cactus.water_level}%`;
        
        // Update growth stage
        const growthStage = document.getElementById('growthStage');
        growthStage.textContent = this.getGrowthStageText(cactus.growth_stage);
        
        // Update statistics
        document.getElementById('totalWaterings').textContent = cactus.total_waterings;
        document.getElementById('consecutiveDays').textContent = cactus.consecutive_days;
        document.getElementById('totalFlowers').textContent = cactus.flowers.length;
        
        // Update flowers
        this.updateFlowers(cactus.flowers);
        
        // Update water button
        const waterBtn = document.getElementById('waterBtn');
        const nextWatering = document.getElementById('nextWatering');
        
        if (can_water) {
            waterBtn.disabled = false;
            waterBtn.textContent = 'Water Cactus';
            nextWatering.textContent = '';
        } else {
            waterBtn.disabled = true;
            waterBtn.textContent = 'Please wait...';
            
            if (next_watering_in && next_watering_in > 0) {
                const hours = Math.floor(next_watering_in / 3600);
                const minutes = Math.floor((next_watering_in % 3600) / 60);
                nextWatering.textContent = `Next watering in: ${hours}h ${minutes}m`;
            }
        }
        
        // Update cactus appearance
        this.updateCactusAppearance(cactus);
    }

    getGrowthStageText(stage) {
        const stages = {
            'Seed': 'Seed',
            'Sprout': 'Sprout',
            'Young': 'Young',
            'Mature': 'Mature',
            'Elder': 'Elder'
        };
        return stages[stage] || 'Unknown';
    }

    updateFlowers(flowers) {
        const flowersContainer = document.getElementById('flowers');
        flowersContainer.innerHTML = '';
        
        flowers.forEach(flower => {
            if (!flower.wilting_at) { // Show only non-wilted flowers
                const flowerElement = document.createElement('div');
                flowerElement.className = `flower ${flower.color.toLowerCase()}`;
                flowerElement.title = `${flower.color} flower (${new Date(flower.bloomed_at).toLocaleDateString()})`;
                flowersContainer.appendChild(flowerElement);
            }
        });
    }

    updateCactusAppearance(cactus) {
        const cactusBody = document.getElementById('cactusBody');
        
        // Change cactus size based on growth stage
        const sizeMultiplier = this.getSizeMultiplier(cactus.growth_stage);
        cactusBody.style.transform = `scale(${sizeMultiplier})`;
        
        // Change color based on water level
        const waterLevel = cactus.water_level;
        if (waterLevel < 30) {
            cactusBody.style.background = 'linear-gradient(45deg, #8BC34A, #CDDC39)'; // Yellowish
        } else if (waterLevel > 80) {
            cactusBody.style.background = 'linear-gradient(45deg, #2E7D32, #4CAF50)'; // Dark green
        } else {
            cactusBody.style.background = 'linear-gradient(45deg, #4CAF50, #8BC34A)'; // Normal green
        }
    }

    getSizeMultiplier(stage) {
        const multipliers = {
            'Seed': 0.5,
            'Sprout': 0.7,
            'Young': 1.0,
            'Mature': 1.2,
            'Elder': 1.4
        };
        return multipliers[stage] || 1.0;
    }

    animateWatering() {
        const cactus = document.getElementById('cactus');
        cactus.style.animation = 'none';
        setTimeout(() => {
            cactus.style.animation = 'waterDrop 0.6s ease';
        }, 10);
        
        // Add CSS animation for watering effect
        const style = document.createElement('style');
        style.textContent = `
            @keyframes waterDrop {
                0% { transform: scale(1); }
                25% { transform: scale(1.1); }
                50% { transform: scale(0.95); }
                75% { transform: scale(1.05); }
                100% { transform: scale(1); }
            }
        `;
        document.head.appendChild(style);
        
        setTimeout(() => {
            document.head.removeChild(style);
        }, 600);
    }

    showMessage(text, type = 'info') {
        const messageEl = document.getElementById('message');
        messageEl.textContent = text;
        messageEl.className = `message show ${type}`;
        
        setTimeout(() => {
            messageEl.classList.remove('show');
        }, 3000);
    }

    setupEventListeners() {
        const waterBtn = document.getElementById('waterBtn');
        waterBtn.addEventListener('click', () => {
            this.waterCactus();
        });
        
        // Auto-refresh every 30 seconds
        setInterval(() => {
            this.loadCactus();
        }, 30000);
    }
}

// Start the app when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new CactusApp();
});
