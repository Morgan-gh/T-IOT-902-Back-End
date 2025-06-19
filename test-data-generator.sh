#!/bin/bash

# Script de génération de données de test pour SenSorSensei
# Envoie des données toutes les secondes pour simuler des capteurs IoT

BASE_URL="http://localhost:8080"
INTERVAL=1  # Intervalle en secondes

echo "🚀 Générateur de données de test SenSorSensei"
echo "=============================================="
echo "URL de base: $BASE_URL"
echo "Intervalle: ${INTERVAL} seconde(s)"
echo "Appuyez sur Ctrl+C pour arrêter"
echo ""

# Fonction pour générer une valeur aléatoire dans une plage
generate_random_value() {
    local min=$1
    local max=$2
    local precision=$3
    awk -v min=$min -v max=$max -v precision=$precision 'BEGIN {
        srand()
        value = min + (rand() * (max - min))
        printf "%.*f", precision, value
    }'
}

# Fonction pour envoyer des données de son
send_sound_data() {
    local sound_level=$(generate_random_value 40 80 1)
    echo "🔊 Envoi données son: ${sound_level} dB"
    
    response=$(curl -s -X POST $BASE_URL/sound \
        -F "sound_level=$sound_level" \
        -H "Content-Type: multipart/form-data" 2>/dev/null)
    
    if [ $? -eq 0 ]; then
        status=$(echo $response | grep -o '"status":"[^"]*"' | cut -d'"' -f4 2>/dev/null || echo "unknown")
        echo "   Status: $status"
    else
        echo "   Erreur: Impossible de contacter l'API"
    fi
}

# Fonction pour envoyer des données de température/humidité
send_humidity_data() {
    local temperature=$(generate_random_value 15 30 1)
    local humidity=$(generate_random_value 40 80 1)
    echo "🌡️  Envoi données DHT11: ${temperature}°C, ${humidity}%"
    
    response=$(curl -s -X POST $BASE_URL/humidity \
        -F "temperature=$temperature" \
        -F "humidity=$humidity" \
        -H "Content-Type: multipart/form-data" 2>/dev/null)
    
    if [ $? -eq 0 ]; then
        status=$(echo $response | grep -o '"status":"[^"]*"' | cut -d'"' -f4 2>/dev/null || echo "unknown")
        echo "   Status: $status"
    else
        echo "   Erreur: Impossible de contacter l'API"
    fi
}

# Fonction pour envoyer des données de poussière
send_dust_data() {
    local dust_concentration=$(generate_random_value 10 60 1)
    local pm25=$(generate_random_value 5 40 1)
    local pm10=$(generate_random_value 10 50 1)
    echo "💨 Envoi données poussière: ${dust_concentration} µg/m³ (PM2.5: ${pm25}, PM10: ${pm10})"
    
    response=$(curl -s -X POST $BASE_URL/dust \
        -F "dust_concentration=$dust_concentration" \
        -F "pm25=$pm25" \
        -F "pm10=$pm10" \
        -H "Content-Type: multipart/form-data" 2>/dev/null)
    
    if [ $? -eq 0 ]; then
        status=$(echo $response | grep -o '"status":"[^"]*"' | cut -d'"' -f4 2>/dev/null || echo "unknown")
        echo "   Status: $status"
    else
        echo "   Erreur: Impossible de contacter l'API"
    fi
}

# Fonction pour envoyer des données de poussière avec estimation
send_dust_data_estimated() {
    local dust_concentration=$(generate_random_value 10 60 1)
    echo "💨 Envoi données poussière (estimées): ${dust_concentration} µg/m³"
    
    response=$(curl -s -X POST $BASE_URL/dust \
        -F "dust_concentration=$dust_concentration" \
        -H "Content-Type: multipart/form-data" 2>/dev/null)
    
    if [ $? -eq 0 ]; then
        status=$(echo $response | grep -o '"status":"[^"]*"' | cut -d'"' -f4 2>/dev/null || echo "unknown")
        echo "   Status: $status"
    else
        echo "   Erreur: Impossible de contacter l'API"
    fi
}

# Compteur pour alterner entre les types de capteurs
counter=0

echo "📊 Démarrage de la génération de données..."
echo ""

while true; do
    counter=$((counter + 1))
    timestamp=$(date '+%H:%M:%S')
    
    echo "[$timestamp] Cycle #$counter"
    
    # Alterner entre les différents types de capteurs
    case $((counter % 4)) in
        0)
            send_sound_data
            ;;
        1)
            send_humidity_data
            ;;
        2)
            send_dust_data
            ;;
        3)
            send_dust_data_estimated
            ;;
    esac
    
    echo ""
    
    # Attendre l'intervalle spécifié
    sleep $INTERVAL
done 