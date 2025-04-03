# T-IOT-902 - SenSorSensei

## 📋 Vue d'ensemble

Ce projet implémente un système IoT complet utilisant des modules LoRa pour la collecte de données à distance, un backend Rust pour le traitement, InfluxDB pour le stockage des séries temporelles et Grafana pour la visualisation et le monitoring.

### Caractéristiques principales

- 📡 Communication sans fil longue portée via protocole LoRa
- 🔄 Backend haute performance en Rust
- 📊 Stockage optimisé des données temporelles avec InfluxDB
- 📈 Tableaux de bord personnalisables avec Grafana
- 🐳 Déploiement simplifié via Docker Compose

## 🏗️ Architecture du système

Le système se compose des éléments suivants :

1. **Modules capteurs LoRa (matériel)** :
   - Collectent les données environnementales (température, humidité, etc.)
   - Transmettent les données via radio LoRa
   - Fonctionnent sur batterie avec optimisation de la consommation énergétique

2. **Passerelle LoRa (matériel)** :
   - Reçoit les signaux radio des capteurs
   - Transmet les données au backend via HTTP/REST
   - Assure la conversion entre protocole LoRa et réseau IP

3. **Backend Rust** :
   - API REST haute performance pour l'ingestion des données
   - Validation et traitement des données
   - Transmission vers InfluxDB pour stockage

4. **InfluxDB** :
   - Base de données optimisée pour les séries temporelles
   - Stockage efficace des données de capteurs
   - Rétention configurable des données

5. **Grafana** :
   - Visualisation interactive des données
   - Tableaux de bord personnalisables
   - Système d'alertes configurable

## 🔧 Prérequis

- Docker et Docker Compose
- Modules LoRa matériels configurés (passerelle et capteurs)
- Connexion réseau entre la passerelle LoRa et le serveur
- Au moins 2GB de RAM sur le serveur
- Espace disque recommandé : 10GB minimum

## ⚙️ Installation

1. Clonez ce dépôt :
   ```bash
   git clone https://github.com/votre-utilisateur/lora-iot-system.git
   cd lora-iot-system
   ```

## 🚀 Lancement

1. Démarrez les services avec Docker Compose :
   ```bash
   docker-compose up -d
   ```

2. Vérifiez que tous les services sont en cours d'exécution :
   ```bash
   docker-compose ps
   ```

3. Initialisez la base de données InfluxDB (première exécution uniquement) :
   ```bash
   docker-compose exec influxdb influx setup \
     --username admin \
     --password adminpassword \
     --org iot-org \
     --bucket iot-data \
     --retention 30d \
     --force
   ```

## ⚙️ Configuration

### Configuration du backend Rust

Le fichier de configuration principal du backend se trouve dans `backend/config.toml`.

Principales options de configuration :
- `server.port` : Port d'écoute de l'API (par défaut : 8080)
- `influxdb.url` : URL de connexion à InfluxDB
- `influxdb.token` : Token d'authentification pour InfluxDB
- `influxdb.org` : Organisation InfluxDB
- `influxdb.bucket` : Bucket de stockage des données

### API REST du backend

Le backend expose les endpoints suivants :

- **POST /api/sensor** : Endpoint principal pour recevoir les données des capteurs
- **GET /health** : Vérification de l'état du service
- **GET /metrics** : Endpoint Prometheus pour la surveillance du backend (optionnel)

### Format des données des capteurs

Les données envoyées à l'API doivent être au format JSON avec la structure minimale suivante :

```json
{
  "device_id": "lora-sensor-01",
  "timestamp": "2025-04-03T14:30:00Z",  // Optionnel, UTC ISO8601
  "measurements": {
    "temperature": 25.5,
    "humidity": 65.2,
    "pressure": 1013.2,  // Optionnel
    "battery": 3.8       // Optionnel
  }
}
```

### Configuration de la passerelle LoRa

Configurez votre passerelle LoRa matérielle pour qu'elle envoie les données à l'API du backend :

- URL : `http://<adresse-ip-du-serveur>:8080/api/sensor`
- Méthode : POST
- Content-Type : application/json
- Authentification : Basic ou Bearer Token (si configuré)


## 🖥️ Accès aux interfaces

- **Backend Rust** : http://localhost:8080
  - Documentation API : http://localhost:8080/docs (si activée)

- **InfluxDB** : http://localhost:8086
  - Identifiants par défaut : admin/adminpassword
  - Organisation : iot-org
  - Bucket : iot-data

- **Grafana** : http://localhost:3000
  - Identifiants par défaut : admin/admin
  - Tableau de bord préconfiguré : "IoT Sensors Overview"

## 🧪 Test du système

### Test sans matériel LoRa

Pour tester le backend sans utiliser de passerelle LoRa matérielle :

```bash
curl -X POST http://localhost:8080/api/sensor \
  -H "Content-Type: application/json" \
  -d '{
    "device_id": "test-sensor",
    "measurements": {
      "temperature": 25.5,
      "humidity": 65.2,
      "battery": 3.8
    }
  }'
```

## 🔍 Surveillance et maintenance

### Vérification des logs

Pour voir les logs d'un service spécifique :
```bash
docker-compose logs -f <service>
```

Où `<service>` peut être `rust-backend`, `influxdb` ou `grafana`.
```