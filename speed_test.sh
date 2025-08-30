#!/bin/bash

echo "🧪 Тест быстрого полива (10 секунд между поливами)"
echo ""

echo "1️⃣ Первый полив:"
curl -s http://localhost:3030/api/cactus/speedtest/water | grep -o '"water_level":[0-9]*\|"total_waterings":[0-9]*\|"message":"[^"]*"'

echo ""
echo ""
echo "2️⃣ Второй полив сразу (должен заблокировать):"
curl -s http://localhost:3030/api/cactus/speedtest/water | grep -o '"can_water":[a-z]*\|"message":"[^"]*"'

echo ""
echo ""
echo "⏳ Ждем 10 секунд..."
sleep 10

echo ""
echo "3️⃣ Третий полив через 10 секунд (должен сработать):"
curl -s http://localhost:3030/api/cactus/speedtest/water | grep -o '"water_level":[0-9]*\|"total_waterings":[0-9]*\|"message":"[^"]*"'

echo ""
