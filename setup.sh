#!/bin/bash

# Script de configuration et démarrage de SenSorSensei
# Ce script aide à configurer et démarrer le projet

set -e

echo "🚀 Configuration de SenSorSensei"
echo "=================================="

# Vérifier si Docker et Docker Compose sont installés
if ! command -v docker &> /dev/null; then
    echo "❌ Docker n'est pas installé. Veuillez installer Docker d'abord."
    exit 1
fi

if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose n'est pas installé. Veuillez installer Docker Compose d'abord."
    exit 1
fi

echo "✅ Docker et Docker Compose sont installés"

# Vérifier si le fichier .env existe
if [ ! -f ".env" ]; then
    echo "📝 Création du fichier .env..."
    echo ""
    echo "⚠️  ATTENTION : Vous devez créer un fichier .env avec vos configurations."
    echo "   Consultez le README.md pour la liste complète des variables."
    echo ""
    echo "📋 Variables obligatoires à configurer :"
    echo "   - SENSOR_COMMUNITY_ID"
    echo "   - SENSOR_COMMUNITY_PIN"
    echo ""
    echo "🔒 Variables de sécurité à changer en production :"
    echo "   - INFLUXDB_TOKEN"
    echo "   - INFLUXDB_ADMIN_TOKEN"
    echo "   - INFLUXDB_ADMIN_PASSWORD"
    echo "   - GRAFANA_ADMIN_PASSWORD"
    echo ""
    
    # Demander si l'utilisateur veut créer un fichier .env de base
    read -p "Voulez-vous créer un fichier .env avec les valeurs par défaut ? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        cat > .env << 'EOF'
# =============================================================================
# CONFIGURATION DU BACKEND RUST
# =============================================================================

# Configuration du serveur
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
RUST_LOG=info

# =============================================================================
# CONFIGURATION INFLUXDB
# =============================================================================

# URL de connexion InfluxDB (interne Docker)
INFLUXDB_URL=http://influxdb:8086

# Token d'authentification InfluxDB
INFLUXDB_TOKEN=my-super-secret-token

# Organisation InfluxDB
INFLUXDB_ORG=iot-org

# Bucket de données InfluxDB
INFLUXDB_BUCKET=iot-data

# =============================================================================
# CONFIGURATION SENSOR COMMUNITY
# =============================================================================

# ID du capteur Sensor Community (OBLIGATOIRE - à remplacer)
SENSOR_COMMUNITY_ID=your_sensor_community_id_here

# PIN du capteur Sensor Community (OBLIGATOIRE - à remplacer)
SENSOR_COMMUNITY_PIN=your_sensor_community_pin_here

# =============================================================================
# CONFIGURATION INFLUXDB (INITIALISATION)
# =============================================================================

# Nom d'utilisateur administrateur InfluxDB
INFLUXDB_ADMIN_USERNAME=admin

# Mot de passe administrateur InfluxDB
INFLUXDB_ADMIN_PASSWORD=adminpassword

# Token administrateur InfluxDB
INFLUXDB_ADMIN_TOKEN=my-super-secret-token

# Période de rétention des données
INFLUXDB_RETENTION=30d

# =============================================================================
# CONFIGURATION GRAFANA
# =============================================================================

# Nom d'utilisateur administrateur Grafana
GRAFANA_ADMIN_USER=admin

# Mot de passe administrateur Grafana
GRAFANA_ADMIN_PASSWORD=admin

# =============================================================================
# CONFIGURATION DES PORTS
# =============================================================================

# Port du backend Rust
BACKEND_PORT=8080

# Port d'InfluxDB
INFLUXDB_PORT=8086

# Port de Grafana
GRAFANA_PORT=3000

# =============================================================================
# CONFIGURATION DES CAPTEURS
# =============================================================================

# Localisation des capteurs
SENSOR_LOCATION=marseille

# ID du capteur de température/humidité
DHT_SENSOR_ID=DHT11

# ID du capteur de son
SOUND_SENSOR_ID=INMP441

# ID du capteur de poussière
DUST_SENSOR_ID=dust_sensor

# =============================================================================
# VARIABLES DE FALLBACK (pour les valeurs par défaut)
# =============================================================================

# Valeurs par défaut pour le backend Rust
SERVER_HOST_DEFAULT=0.0.0.0
SERVER_PORT_DEFAULT=8080
RUST_LOG_DEFAULT=info

# Valeurs par défaut pour InfluxDB
INFLUXDB_URL_DEFAULT=http://localhost:8086
INFLUXDB_ORG_DEFAULT=iot-org
INFLUXDB_BUCKET_DEFAULT=iot-data

# Valeur par défaut pour le test Sensor Community
TEST_SENSOR_COMMUNITY_DEFAULT=false

# Test de connexion à Sensor Community au démarrage
TEST_SENSOR_COMMUNITY=false

# Valeurs par défaut pour les capteurs
SENSOR_LOCATION_DEFAULT=marseille
DHT_SENSOR_ID_DEFAULT=DHT11
SOUND_SENSOR_ID_DEFAULT=INMP441
DUST_SENSOR_ID_DEFAULT=dust_sensor
EOF
        echo "✅ Fichier .env créé avec les valeurs par défaut"
        echo "⚠️  N'oubliez pas de configurer SENSOR_COMMUNITY_ID et SENSOR_COMMUNITY_PIN !"
    else
        echo "❌ Fichier .env requis. Veuillez le créer manuellement."
        exit 1
    fi
else
    echo "✅ Fichier .env trouvé"
fi

# Vérifier les variables obligatoires
echo "🔍 Vérification des variables obligatoires..."

if ! grep -q "SENSOR_COMMUNITY_ID=your_sensor_community_id_here" .env && ! grep -q "SENSOR_COMMUNITY_ID=" .env; then
    echo "⚠️  SENSOR_COMMUNITY_ID n'est pas configuré"
fi

if ! grep -q "SENSOR_COMMUNITY_PIN=your_sensor_community_pin_here" .env && ! grep -q "SENSOR_COMMUNITY_PIN=" .env; then
    echo "⚠️  SENSOR_COMMUNITY_PIN n'est pas configuré"
fi

echo "✅ Configuration vérifiée"

# Construire et démarrer les services
echo "🐳 Démarrage des services Docker..."

# Construire les images
echo "📦 Construction des images..."
docker-compose build

# Démarrer les services
echo "🚀 Démarrage des services..."
docker-compose up -d

# Attendre que les services soient prêts
echo "⏳ Attente du démarrage des services..."
sleep 10

# Vérifier le statut des services
echo "📊 Statut des services :"
docker-compose ps

echo ""
echo "🎉 SenSorSensei est maintenant en cours d'exécution !"
echo ""
echo "📱 Accès aux interfaces :"
echo "   - Backend API : http://localhost:8080"
echo "   - InfluxDB    : http://localhost:8086"
echo "   - Grafana     : http://localhost:3000"
echo ""
echo "📚 Documentation :"
echo "   - Variables d'environnement : consultez le README.md"
echo "   - API REST : consultez le README.md"
echo ""
echo "🔍 Logs :"
echo "   docker-compose logs -f"
echo ""
echo "🛑 Arrêt :"
echo "   docker-compose down" 