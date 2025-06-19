# T-IOT-902 - SenSorSensei 🌟

## 📋 Vue d'ensemble

SenSorSensei est un système IoT complet qui collecte et analyse les données environnementales via des capteurs. Le système utilise un backend Rust pour le traitement, InfluxDB pour le stockage et Sensor Community pour partager les données publiquement.

### ✨ Caractéristiques principales

- 🔄 Backend haute performance en Rust
- 📊 Stockage optimisé des données temporelles avec InfluxDB
- 🌍 Partage des données avec Sensor Community
- 📈 Tableaux de bord personnalisables avec Grafana
- 🐳 Déploiement simplifié via Docker Compose

## 🏗️ Architecture du système

Le système se compose des éléments suivants :

1. **📡 Capteurs** :
   - Capteur de température et d'humidité (DHT11/DHT22)
   - Capteur de qualité de l'air/poussière (SDS011/PMS5003)
   - Capteur de niveau sonore (INMP441)

2. **⚡ Backend Rust** :
   - API REST haute performance
   - Endpoints dédiés pour chaque type de capteur
   - Validation et traitement des données
   - Double transmission vers InfluxDB et Sensor Community

3. **🗄️ InfluxDB** :
   - Base de données optimisée pour les séries temporelles
   - Stockage efficace des données de capteurs
   - Source de données pour Grafana

4. **🌐 Sensor Community** :
   - Partage des données environnementales avec la communauté
   - Contribution aux cartes de pollution publiques

5. **📊 Grafana** :
   - Visualisation interactive des données
   - Tableaux de bord personnalisables
   - Connexion directe à InfluxDB

### 🔄 Flux de données

```
[Capteurs] → [LoRa] → [LoRaWan] → [Gateway] → [HTTPS] → [Backend Rust] → [InfluxDB + Sensor Community] → [Grafana + Sensor Map]
```

## 🔧 Prérequis

- 🐳 Docker et Docker Compose
- 💾 Au moins 2GB de RAM
- 💿 Espace disque : 10GB minimum
- 🌍 Compte Sensor Community

## 🚀 Installation et déploiement

1. **📥 Clonez ce dépôt** :
   ```bash
   git clone https://github.com/Morgan-gh/T-IOT-902-Back-End.git
   cd T-IOT-902-Back-End
   ```

2. **⚙️ Configurez automatiquement le projet** :
   ```bash
   ./setup.sh
   ```

## ⚙️ Configuration

### 🔑 Variables d'environnement

**📋 Variables obligatoires :**

```env
# Configuration Sensor Community (OBLIGATOIRE)
SENSOR_COMMUNITY_URL=https://api.sensor.community/v1/push-sensor-data/
SENSOR_COMMUNITY_ID=MON_DEVICE_SENSOR
SENSOR_COMMUNITY_PIN=MON_PIN_SENSOR

# Configuration InfluxDB (recommandé de changer en production)
INFLUXDB_TOKEN=my-super-secret-token
INFLUXDB_ADMIN_TOKEN=my-super-secret-token
INFLUXDB_ADMIN_PASSWORD=adminpassword
GRAFANA_ADMIN_PASSWORD=admin
```

**⚙️ Variables avec valeurs par défaut :**

```env
# Configuration du serveur
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
RUST_LOG=info

# Configuration InfluxDB
INFLUXDB_URL=http://influxdb:8086
INFLUXDB_ORG=iot-org
INFLUXDB_BUCKET=iot-data

# Configuration des ports
BACKEND_PORT=8080
INFLUXDB_PORT=8086
GRAFANA_PORT=3000

# Configuration des capteurs
SENSOR_LOCATION=marseille
DHT_SENSOR_ID=DHT11
SOUND_SENSOR_ID=INMP441
DUST_SENSOR_ID=dust_sensor
```

### 🌍 Configuration Sensor Community

1. **📝 Obtenir un ID Sensor Community** :
   - Créez un compte sur https://sensor.community/
   - Enregistrez votre station
   - Notez l'ID et le PIN fournis

2. **🔧 Configuration dans le projet** :
   - `SENSOR_COMMUNITY_URL` : URL de l'API (toujours la même)
   - `SENSOR_COMMUNITY_ID` : ID de votre station
   - `SENSOR_COMMUNITY_PIN` : PIN de votre station

## 📡 API REST du backend

### ℹ️ Informations générales

- **🌐 URL de base** : `http://localhost:8080`
- **📦 Format de données** : `multipart/form-data`
- **📤 Méthode HTTP** : `POST`

### 🔌 Endpoints disponibles

#### 🔊 Capteur de Son - `/sound`

```bash
curl -X POST http://localhost:8080/sound -F "sound_level=65.2"
```

| Paramètre | Type | Plage | Description |
|-----------|------|-------|-------------|
| `sound_level` | float | -60.0 à 120.0 | Niveau sonore en décibels (dB) |

#### 🌡️ Capteur Température/Humidité - `/humidity`

```bash
curl -X POST http://localhost:8080/humidity -F "temperature=23.5" -F "humidity=68.2"
```

| Paramètre | Type | Plage | Description |
|-----------|------|-------|-------------|
| `temperature` | float | -40.0 à 80.0 | Température en °C |
| `humidity` | float | 0.0 à 100.0 | Humidité relative en % |

#### 💨 Capteur de Poussière - `/dust`

```bash
curl -X POST http://localhost:8080/dust -F "dust_concentration=45.2" -F "pm25=32.1" -F "pm10=45.2"
```

| Paramètre | Type | Plage | Requis | Description |
|-----------|------|-------|--------|-------------|
| `dust_concentration` | float | 0.0 à 1000.0 | ✅ Oui | Concentration générale en µg/m³ |
| `pm25` | float | 0.0 à 500.0 | ❌ Non | Particules PM2.5 en µg/m³ |
| `pm10` | float | 0.0 à 500.0 | ❌ Non | Particules PM10 en µg/m³ |

## 🧪 Test du système

### 🔄 Script de test automatique

Utilisez le script fourni pour tester le système :

```bash
./test-data-generator.sh
```

Ce script envoie des données de test toutes les secondes vers tous les endpoints.

### 🖱️ Test manuel

```bash
# Test du capteur de son
curl -X POST http://localhost:8080/sound -F "sound_level=65.2"

# Test du capteur température/humidité
curl -X POST http://localhost:8080/humidity -F "temperature=23.5" -F "humidity=68.2"

# Test du capteur de poussière
curl -X POST http://localhost:8080/dust -F "dust_concentration=45.2"
```

## 🖥️ Accès aux interfaces

- **⚡ Backend Rust** : http://localhost:8080
- **🗄️ InfluxDB** : http://localhost:8086
  - Identifiants : admin/adminpassword
- **📊 Grafana** : http://localhost:3000
  - Identifiants : admin/admin
- **🌍 Sensor Community** : https://sensor.community/

## 🔍 Surveillance

### 📋 Vérification des logs

```bash
# Logs du backend
docker-compose logs -f rust-backend

# Logs d'un service spécifique
docker-compose logs -f influxdb
docker-compose logs -f grafana
```

### 📊 Statut des services

```bash
docker-compose ps
```

## 🔒 Sécurité

⚠️ **Important pour la production :**

1. **🔐 Changez tous les mots de passe par défaut**
2. **🔑 Utilisez des tokens sécurisés**
3. **🌐 Limitez l'accès réseau**
4. **🚫 Ne committez jamais le fichier `.env`**

## 📚 Documentation

Pour plus de détails sur la configuration des capteurs :
[Documentation IOT SenSorSensei](https://www.notion.so/1aeceb35280c806db5b0cefe2d1deb24?v=1aeceb35280c81d0a6e6000cff38b90a)