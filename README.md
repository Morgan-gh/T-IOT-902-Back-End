# T-IOT-902 - SenSorSensei

## 📋 Vue d'ensemble

SenSorSensei est un système IoT complet qui collecte et analyse les données environnementales via des capteurs LoRa. Le système utilise LoRaWAN pour la transmission des données, un backend Rust pour le traitement, InfluxDB pour le stockage et Sensor Community pour partager les données publiquement.

### Caractéristiques principales

- 📡 Communication LoRaWAN pour une portée étendue
- 🔄 Backend haute performance en Rust
- 📊 Stockage optimisé des données temporelles avec InfluxDB
- 🌍 Partage des données avec Sensor Community
- 📈 Tableaux de bord personnalisables avec Grafana
- 🐳 Déploiement simplifié via Docker Compose

## 🏗️ Architecture du système

Le système se compose des éléments suivants :

1. **Capteurs et Module LoRa** :
   - Trois capteurs connectés à un module LoRa :
     - Capteur de température et d'humidité (DHT11/DHT22)
     - Capteur de qualité de l'air/poussière (SDS011/PMS5003)
     - Capteur de niveau sonore (INMP441)
   - Le module LoRa transmet les données via le protocole LoRaWAN

2. **Passerelle LoRa** :
   - Reçoit les données LoRaWAN des capteurs
   - Convertit les données en requêtes HTTP
   - Transmet les données au backend Rust

3. **Backend Rust** :
   - API REST haute performance
   - Endpoints dédiés pour chaque type de capteur
   - Validation et traitement des données
   - Double transmission vers InfluxDB et Sensor Community

4. **InfluxDB** :
   - Base de données optimisée pour les séries temporelles
   - Stockage efficace des données de capteurs
   - Rétention configurable des données
   - Source de données pour Grafana

5. **Sensor Community** :
   - Partage des données environnementales avec la communauté
   - Contribution aux cartes de pollution publiques
   - API ouverte pour accès aux données

6. **Grafana** :
   - Visualisation interactive des données
   - Tableaux de bord personnalisables
   - Système d'alertes configurable
   - Connexion directe à InfluxDB pour les données en temps réel

### Flux de données

```
[Capteurs] → [Module LoRa] → [LoRaWAN] → [Gateway] → [HTTP] → [Backend Rust] → [InfluxDB + Sensor Community] → [Grafana]
```

## 🔧 Prérequis

- Docker et Docker Compose
- Modules LoRa matériels configurés (passerelle et capteurs)
- Connexion réseau entre la passerelle LoRa et le serveur
- Au moins 2GB de RAM sur le serveur
- Espace disque recommandé : 10GB minimum
- Compte Sensor Community pour le partage de données

## 🚀 Installation et déploiement

1. Clonez ce dépôt :
   ```bash
   git clone https://github.com/Morgan-gh/T-IOT-902-Back-End.git
   ```

2. Configurez les variables d'environnement :
   ```bash
   cp .env.example .env
   # Éditez le fichier .env avec vos configurations
   ```

3. Démarrez les services avec Docker Compose :
   ```bash
   docker-compose up -d
   ```

4. Vérifiez que tous les services sont en cours d'exécution :
   ```bash
   docker-compose ps
   ```

## ⚙️ Configuration

### Variables d'environnement

Configurez le fichier `.env` avec les valeurs suivantes :

```env
# Configuration InfluxDB
INFLUXDB_URL=http://localhost:8086
INFLUXDB_TOKEN=
INFLUXDB_ORG=
INFLUXDB_BUCKET=

# Configuration Sensor Community
SENSOR_COMMUNITY_ID=
SENSOR_COMMUNITY_PIN=

# Configuration serveur
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
RUST_LOG=info
```

### Configuration du backend Rust

Le backend expose une API REST pour recevoir les données des capteurs LoRa et les transmet automatiquement vers InfluxDB et Sensor Community.

## 📡 API REST du backend

### Informations générales

- **URL de base** : `http://localhost:8080`
- **Format de données** : `multipart/form-data`
- **Méthode HTTP** : `POST`
- **Réponse** : JSON

### Endpoints disponibles

#### 🔊 Capteur de Son - `/sound`

**URL** : `POST /sound`

| Paramètre | Type | Plage | Description |
|-----------|------|-------|-------------|
| `sound_level` | float | -60.0 à 120.0 | Niveau sonore en décibels (dB) |

**Réponse de succès** :
```json
{
  "status": "success",
  "message": "Données stockées dans InfluxDB et envoyées à Sensor Community",
  "data": {
    "sound_level": 65.2,
    "unit": "dB",
    "sensor_type": "INMP441",
    "location": "marseille"
  },
  "delivery_status": {
    "influxdb": true,
    "sensor_community": true
  },
  "timestamp": "2025-06-12T14:30:45Z"
}
```

#### 🌡️ Capteur Température/Humidité - `/humidity`

**URL** : `POST /humidity`

| Paramètre | Type | Plage | Description |
|-----------|------|-------|-------------|
| `temperature` | float | -40.0 à 80.0 | Température en degrés Celsius (°C) |
| `humidity` | float | 0.0 à 100.0 | Humidité relative en pourcentage (%) |

**Réponse de succès** :
```json
{
  "status": "success",
  "message": "Données stockées dans InfluxDB et envoyées à Sensor Community",
  "data": {
    "temperature": {
      "value": 23.5,
      "unit": "°C"
    },
    "humidity": {
      "value": 68.2,
      "unit": "%"
    },
    "sensor_type": "DHT11",
    "location": "marseille"
  },
  "delivery_status": {
    "influxdb": true,
    "sensor_community": true
  },
  "timestamp": "2025-06-12T14:30:45Z"
}
```

#### 💨 Capteur de Poussière - `/dust`

**URL** : `POST /dust`

| Paramètre | Type | Plage | Requis | Description |
|-----------|------|-------|--------|-------------|
| `dust_concentration` | float | 0.0 à 1000.0 | ✅ Oui | Concentration générale de poussière en µg/m³ |
| `pm25` | float | 0.0 à 500.0 | ❌ Non | Particules PM2.5 en µg/m³ (optionnel) |
| `pm10` | float | 0.0 à 500.0 | ❌ Non | Particules PM10 en µg/m³ (optionnel) |

> **Note** : Si `pm25` et `pm10` ne sont pas fournis, ils seront estimés automatiquement à partir de `dust_concentration`.

**Réponse de succès** :
```json
{
  "status": "success",
  "message": "Données stockées dans InfluxDB et envoyées à Sensor Community",
  "data": {
    "dust_concentration": {
      "value": 45.2,
      "unit": "µg/m³"
    },
    "pm25": {
      "value": 32.1,
      "unit": "µg/m³",
      "estimated": false
    },
    "pm10": {
      "value": 45.2,
      "unit": "µg/m³",
      "estimated": false
    },
    "sensor_type": "particulate_matter",
    "location": "marseille"
  },
  "delivery_status": {
    "influxdb": true,
    "sensor_community": true
  },
  "air_quality_index": {
    "pm25_category": "Moderate",
    "pm10_category": "Good"
  },
  "timestamp": "2025-06-12T14:30:45Z"
}
```

### Gestion des erreurs

#### Codes de statut :
- **200 OK** : Données traitées avec succès
- **400 Bad Request** : Données invalides ou manquantes
- **500 Internal Server Error** : Erreur serveur

#### Types de statut dans la réponse :
- **`"success"`** : Données envoyées avec succès vers InfluxDB ET Sensor Community
- **`"partial_success"`** : Données envoyées vers une seule destination
- **`"error"`** : Échec complet

## 🖥️ Accès aux interfaces

- **Backend Rust** : http://localhost:8080
- **InfluxDB** : http://localhost:8086
  - Identifiants par défaut : admin/adminpassword
- **Grafana** : http://localhost:3000
  - Identifiants par défaut : admin/admin
- **Sensor Community** : https://sensor.community/
  - Visualisation publique des données

## 🔍 Surveillance et maintenance

### Vérification des logs

Pour voir les logs du backend :
```bash
docker-compose logs -f rust-backend
```

Pour voir les logs d'un service spécifique :
```bash
docker-compose logs -f <service>
```

Où `<service>` peut être `rust-backend`, `influxdb` ou `grafana`.

### Sauvegarde des données

Les données sont stockées dans des volumes Docker :
- `influxdb-data` : Données InfluxDB
- `grafana-data` : Configuration et tableaux de bord Grafana

## 📚 Documentation

### Documentation IOT
Pour plus de détails sur la configuration et l'implémentation des capteurs et de la passerelle LoRa, consultez notre documentation complète sur Notion :
[Documentation IOT SenSorSensei](https://www.notion.so/1aeceb35280c806db5b0cefe2d1deb24?v=1aeceb35280c81d0a6e6000cff38b90a)