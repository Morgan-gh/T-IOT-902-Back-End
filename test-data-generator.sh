#!/bin/bash

# Script de génération de données de test pour SenSorSensei
# Envoie des données toutes les secondes pour simuler des capteurs IoT
# NOUVELLE ARCHITECTURE: Collecte groupée pour Sensor Community

BASE_URL="http://localhost:8080"
INTERVAL=1  # Intervalle en secondes
SENSOR_COMMUNITY_INTERVAL=10  # Envoi groupé vers Sensor Community toutes les 10 secondes

echo "🚀 Générateur de données de test SenSorSensei"
echo "=============================================="
echo "URL de base: $BASE_URL"
echo "Intervalle: ${INTERVAL} seconde(s)"
echo "Envoi Sensor Community: toutes les ${SENSOR_COMMUNITY_INTERVAL} secondes"
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
        influx_status=$(echo $response | grep -o '"influxdb_storage":[^,]*' | grep -o 'true\|false' 2>/dev/null || echo "unknown")
        sensor_collection=$(echo $response | grep -o '"sensor_community_collection":[^,]*' | grep -o 'true\|false' 2>/dev/null || echo "unknown")
        echo "   Status: $status | InfluxDB: $influx_status | Collecte SC: $sensor_collection"
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
        influx_status=$(echo $response | grep -o '"influxdb_storage":[^,]*' | grep -o 'true\|false' 2>/dev/null || echo "unknown")
        sensor_collection=$(echo $response | grep -o '"sensor_community_collection":[^,]*' | grep -o 'true\|false' 2>/dev/null || echo "unknown")
        echo "   Status: $status | InfluxDB: $influx_status | Collecte SC: $sensor_collection"
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
        influx_status=$(echo $response | grep -o '"influxdb_storage":[^,]*' | grep -o 'true\|false' 2>/dev/null || echo "unknown")
        sensor_collection=$(echo $response | grep -o '"sensor_community_collection":[^,]*' | grep -o 'true\|false' 2>/dev/null || echo "unknown")
        echo "   Status: $status | InfluxDB: $influx_status | Collecte SC: $sensor_collection"
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
        influx_status=$(echo $response | grep -o '"influxdb_storage":[^,]*' | grep -o 'true\|false' 2>/dev/null || echo "unknown")
        sensor_collection=$(echo $response | grep -o '"sensor_community_collection":[^,]*' | grep -o 'true\|false' 2>/dev/null || echo "unknown")
        echo "   Status: $status | InfluxDB: $influx_status | Collecte SC: $sensor_collection"
    else
        echo "   Erreur: Impossible de contacter l'API"
    fi
}

# Fonction pour vérifier le statut des données collectées
check_sensor_community_status() {
    echo "📊 Vérification du statut des dernières valeurs..."
    
    response=$(curl -s -X POST $BASE_URL/sensor-community/status 2>/dev/null)
    
    if [ $? -eq 0 ]; then
        has_data=$(echo $response | grep -o '"has_data":[^,]*' | grep -o 'true\|false' 2>/dev/null || echo "false")
        echo "   Données disponibles: $has_data"
    else
        echo "   Erreur: Impossible de vérifier le statut"
    fi
}

# Fonction pour envoyer toutes les données collectées vers Sensor Community
send_to_sensor_community() {
    echo "🚀 Envoi des dernières valeurs vers Sensor Community..."
    
    response=$(curl -s -X POST $BASE_URL/sensor-community/send 2>/dev/null)
    
    if [ $? -eq 0 ]; then
        status=$(echo $response | grep -o '"status":"[^"]*"' | cut -d'"' -f4 2>/dev/null || echo "unknown")
        data_sent=$(echo $response | grep -o '"data_sent":[^,]*' | grep -o 'true\|false' 2>/dev/null || echo "unknown")
        message=$(echo $response | grep -o '"message":"[^"]*"' | cut -d'"' -f4 2>/dev/null || echo "unknown")
        echo "   Status: $status | Données envoyées: $data_sent"
        echo "   Message: $message"
    else
        echo "   Erreur: Impossible d'envoyer vers Sensor Community"
    fi
}

# Fonction pour vider les données collectées
clear_collected_data() {
    echo "🗑️  Vidage des données collectées..."
    
    response=$(curl -s -X POST $BASE_URL/sensor-community/clear 2>/dev/null)
    
    if [ $? -eq 0 ]; then
        status=$(echo $response | grep -o '"status":"[^"]*"' | cut -d'"' -f4 2>/dev/null || echo "unknown")
        sessions_cleared=$(echo $response | grep -o '"sessions_cleared":[0-9]*' | cut -d':' -f2 2>/dev/null || echo "0")
        echo "   Status: $status | Sessions vidées: $sessions_cleared"
    else
        echo "   Erreur: Impossible de vider les données"
    fi
}

# Compteur pour alterner entre les types de capteurs
counter=0
sensor_community_counter=0

echo "📊 Démarrage de la génération de données..."
echo ""

while true; do
    counter=$((counter + 1))
    sensor_community_counter=$((sensor_community_counter + 1))
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
    
    # Vérifier le statut des données collectées
    check_sensor_community_status
    
    # Envoyer vers Sensor Community selon l'intervalle configuré
    if [ $sensor_community_counter -ge $SENSOR_COMMUNITY_INTERVAL ]; then
        echo ""
        echo "🔄 === ENVOI AUTOMATIQUE DES DERNIÈRES VALEURS ==="
        send_to_sensor_community
        echo "=== FIN ENVOI AUTOMATIQUE ==="
        echo ""
        sensor_community_counter=0
    fi
    
    echo ""
    
    # Attendre l'intervalle spécifié
    sleep $INTERVAL
done 