# T-IOT-902 - SenSorSensei

## 📋 Vue d'ensemble

SenSorSensei est un système IoT complet qui collecte et analyse les données environnementales via des capteurs LoRa. Le système utilise LoRaWAN pour la transmission des données, un backend Rust pour le traitement, et Grafana pour la visualisation.

### Caractéristiques principales

- 📡 Communication LoRaWAN pour une portée étendue
- 🔄 Backend haute performance en Rust
- 📊 Stockage optimisé des données temporelles avec InfluxDB
- 📈 Tableaux de bord personnalisables avec Grafana
- 🐳 Déploiement simplifié via Docker Compose

## 🏗️ Architecture du système

Le système se compose des éléments suivants :

1. **Capteurs et Module LoRa** :
   - Trois capteurs connectés à un module LoRa :
     - Capteur de température et d'humidité
     - Capteur de qualité de l'air (poussière)
     - Capteur de niveau sonore
   - Le module LoRa transmet les données via le protocole LoRaWAN

2. **Passerelle ESP32** :
   - Reçoit les données LoRaWAN des capteurs
   - Convertit les données en requêtes HTTP
   - Transmet les données au backend Rust

3. **Backend Rust** :
   - API REST haute performance
   - Endpoints dédiés pour chaque type de capteur :
     - `/api/sound` pour les données sonores
     - `/api/humidity` pour l'humidité et la température
     - `/api/dust` pour la qualité de l'air
   - Validation et traitement des données
   - Transmission vers InfluxDB

4. **InfluxDB** :
   - Base de données optimisée pour les séries temporelles
   - Stockage efficace des données de capteurs
   - Rétention configurable des données
   - Source de données pour Grafana

5. **Grafana** :
   - Visualisation interactive des données
   - Tableaux de bord personnalisables
   - Système d'alertes configurable
   - Connexion directe à InfluxDB pour les données en temps réel

### Flux de données

```
[Capteurs] → [Module LoRa] → [LoRaWAN] → [ESP32 Gateway] → [HTTP] → [Backend Rust] → [InfluxDB] → [Grafana]
```

## 🔧 Prérequis

- Docker et Docker Compose
- Modules LoRa matériels configurés (passerelle et capteurs)
- Connexion réseau entre la passerelle LoRa et le serveur
- Au moins 2GB de RAM sur le serveur
- Espace disque recommandé : 10GB minimum

## 🚀 Installation et déploiement

1. Clonez ce dépôt :
   ```bash
   git clone https://github.com/Morgan-gh/T-IOT-902-Back-End.git
   ```

2. Démarrez les services avec Docker Compose :
   ```bash
   docker-compose up -d
   ```

3. Vérifiez que tous les services sont en cours d'exécution :
   ```bash
   docker-compose ps
   ```

4. Initialisez la base de données InfluxDB (première exécution uniquement) :
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

Le backend est configuré via des variables d'environnement dans le `docker-compose.yml` :

- `INFLUXDB_URL` : URL de connexion à InfluxDB
- `INFLUXDB_TOKEN` : Token d'authentification
- `INFLUXDB_ORG` : Organisation InfluxDB
- `INFLUXDB_BUCKET` : Bucket de stockage

### API REST du backend

Le backend expose les endpoints suivants :

- **POST /api/sound** : Données des capteurs sonores
- **POST /api/humidity** : Données des capteurs d'humidité
- **POST /api/dust** : Données des capteurs de poussière

### Format des données (A revoir)

Les données doivent être envoyées au format JSON. Exemple pour un capteur d'humidité :

```json
{
  "device_id": "lora-humidity-01",
  "timestamp": "2024-03-20T14:30:00Z",
  "humidity": 65.2,
  "temperature": 25.5,
  "battery": 3.8
}
```

## 🖥️ Accès aux interfaces

- **Backend Rust** : http://localhost:8080
- **InfluxDB** : http://localhost:8086
  - Identifiants par défaut : admin/adminpassword
- **Grafana** : http://localhost:3000
  - Identifiants par défaut : admin/admin

## 🔍 Surveillance et maintenance

### Vérification des logs

Pour voir les logs d'un service spécifique :
```bash
docker-compose logs -f <service>
```

Où `<service>` peut être `rust-backend`, `influxdb` ou `grafana`.

### Sauvegarde des données

Les données sont stockées dans des volumes Docker :
- `influxdb-data` : Données InfluxDB
- `grafana-data` : Configuration et tableaux de bord Grafana

## 🤝 Contribution

Les contributions sont les bienvenues ! N'hésitez pas à :
1. Fork le projet
2. Créer une branche pour votre fonctionnalité
3. Commiter vos changements
4. Pousser vers la branche
5. Ouvrir une Pull Request

## 📚 Documentation

### Documentation IOT
Pour plus de détails sur la configuration et l'implémentation des capteurs et de la passerelle LoRa, consultez notre documentation complète sur Notion :
[Documentation IOT SenSorSensei](https://www.notion.so/1aeceb35280c806db5b0cefe2d1deb24?v=1aeceb35280c81d0a6e6000cff38b90a)
