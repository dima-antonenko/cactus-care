class CactusApp {
    constructor() {
        this.userId = this.getUserId();
        this.cactus = null;
        this.init();
    }

    getUserId() {
        // В реальном Telegram Mini App используем Telegram.WebApp.initDataUnsafe.user.id
        // Для демонстрации используем фиксированный ID
        if (window.Telegram && window.Telegram.WebApp) {
            const user = window.Telegram.WebApp.initDataUnsafe?.user;
            if (user && user.id) {
                return user.id.toString();
            }
        }
        // Для тестирования используем случайный ID
        return 'demo_user_' + Math.random().toString(36).substr(2, 9);
    }

    async init() {
        console.log('🌵 Инициализация Cactus Care App...');
        console.log('👤 User ID:', this.userId);
        
        // Инициализируем Telegram WebApp если доступен
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
            
            console.log('🌵 Кактус загружен:', this.cactus);
        } catch (error) {
            console.error('❌ Ошибка загрузки кактуса:', error);
            this.showMessage('Ошибка загрузки данных', 'error');
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
            
            console.log('💧 Кактус полит:', data.message);
            this.showMessage(data.message, 'success');
            
            // Анимация полива
            this.animateWatering();
            
        } catch (error) {
            console.error('❌ Ошибка полива:', error);
            this.showMessage('Ошибка полива кактуса', 'error');
        }
    }

    updateUI(data) {
        const { cactus, can_water, next_watering_in } = data;
        
        // Обновляем уровень воды
        const waterFill = document.getElementById('waterFill');
        const waterPercentage = document.getElementById('waterPercentage');
        waterFill.style.width = `${cactus.water_level}%`;
        waterPercentage.textContent = `${cactus.water_level}%`;
        
        // Обновляем стадию роста
        const growthStage = document.getElementById('growthStage');
        growthStage.textContent = this.getGrowthStageText(cactus.growth_stage);
        
        // Обновляем статистику
        document.getElementById('totalWaterings').textContent = cactus.total_waterings;
        document.getElementById('consecutiveDays').textContent = cactus.consecutive_days;
        document.getElementById('totalFlowers').textContent = cactus.flowers.length;
        
        // Обновляем цветы
        this.updateFlowers(cactus.flowers);
        
        // Обновляем кнопку полива
        const waterBtn = document.getElementById('waterBtn');
        const nextWatering = document.getElementById('nextWatering');
        
        if (can_water) {
            waterBtn.disabled = false;
            waterBtn.textContent = '💧 Полить кактус';
            nextWatering.textContent = '';
        } else {
            waterBtn.disabled = true;
            waterBtn.textContent = '⏳ Подождите...';
            
            if (next_watering_in && next_watering_in > 0) {
                const hours = Math.floor(next_watering_in / 3600);
                const minutes = Math.floor((next_watering_in % 3600) / 60);
                nextWatering.textContent = `Следующий полив через: ${hours}ч ${minutes}м`;
            }
        }
        
        // Обновляем внешний вид кактуса
        this.updateCactusAppearance(cactus);
    }

    getGrowthStageText(stage) {
        const stages = {
            'Seed': 'Семечко',
            'Sprout': 'Росток',
            'Young': 'Молодой',
            'Mature': 'Зрелый',
            'Elder': 'Старый'
        };
        return stages[stage] || 'Неизвестно';
    }

    updateFlowers(flowers) {
        const flowersContainer = document.getElementById('flowers');
        flowersContainer.innerHTML = '';
        
        flowers.forEach(flower => {
            if (!flower.wilting_at) { // Показываем только не увядшие цветы
                const flowerElement = document.createElement('div');
                flowerElement.className = `flower ${flower.color.toLowerCase()}`;
                flowerElement.title = `Цветок ${flower.color} (${new Date(flower.bloomed_at).toLocaleDateString()})`;
                flowersContainer.appendChild(flowerElement);
            }
        });
    }

    updateCactusAppearance(cactus) {
        const cactusBody = document.getElementById('cactusBody');
        
        // Изменяем размер кактуса в зависимости от стадии роста
        const sizeMultiplier = this.getSizeMultiplier(cactus.growth_stage);
        cactusBody.style.transform = `scale(${sizeMultiplier})`;
        
        // Изменяем цвет в зависимости от уровня воды
        const waterLevel = cactus.water_level;
        if (waterLevel < 30) {
            cactusBody.style.background = 'linear-gradient(45deg, #8BC34A, #CDDC39)'; // Желтоватый
        } else if (waterLevel > 80) {
            cactusBody.style.background = 'linear-gradient(45deg, #2E7D32, #4CAF50)'; // Темно-зеленый
        } else {
            cactusBody.style.background = 'linear-gradient(45deg, #4CAF50, #8BC34A)'; // Обычный зеленый
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
        
        // Добавляем CSS анимацию для эффекта полива
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
        
        // Автообновление каждые 30 секунд
        setInterval(() => {
            this.loadCactus();
        }, 30000);
    }
}

// Запускаем приложение когда DOM загружен
document.addEventListener('DOMContentLoaded', () => {
    new CactusApp();
});
