# Backend en Rust avec Actix

Ce projet est un backend simple en **Rust** utilisant le framework **Actix** pour la gestion des routes et des requêtes HTTP.

## 🛠️ Installation

### Installer Rust
Installez Rust avec la commande suivante :

``` bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Ajoutez Rust à votre PATH :

``` bash
source $HOME/.cargo/env
```

Vérifiez l'installation :

``` bash
rustc --version
cargo --version
```

---

### Lancer le serveur
Exécutez le serveur avec :

``` bash
cargo run
```

Le serveur sera accessible à l'adresse suivante :
http://127.0.0.1:8080


---

### Tester l'API
**GET** sur `/` :

``` bash
curl http://127.0.0.1:8080/
```

**POST** sur `/echo` :

``` bash
curl -X POST http://127.0.0.1:8080/echo -H "Content-Type: application/json" -d '{"message": "Salut"}'
```

---

### Ex. Créer une route dynamique
Exemple d'une route dynamique :

``` rust
#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}
```

---

## ✅ Commandes utiles
| Commande | Description |
|----------|-------------|
| ``` cargo new mon-backend ``` | Crée un nouveau projet |
| ``` cargo run ``` | Lance le serveur |
| ``` cargo check ``` | Vérifie le code sans le compiler |
| ``` cargo build --release ``` | Compile le code pour la production |
| ``` cargo clean ``` | Nettoie les fichiers compilés |

---

## 🎯 Ce que vous obtenez :
✅ Serveur REST fonctionnel en Rust  
✅ Gestion des routes (GET, POST)  
✅ Sérialisation/Désérialisation JSON  
✅ Serveur rapide et sécurisé  

---

---- PARTIE DE VIRGIL et THOMAS ----

# Système IoT avec LoRa, Rust, InfluxDB et Grafana

Ce projet implémente un système IoT utilisant des modules LoRa matériels, un backend Rust, le stockage des données dans InfluxDB et la visualisation via Grafana.

## Architecture du système

Le système se compose de:

1. **Module capteur LoRa (matériel)**: Collecte les données physiques et les transmet via radio LoRa
2. **Passerelle LoRa (matériel)**: Reçoit les signaux radio et les transmet au backend via HTTP
3. **Backend Rust**: API REST qui reçoit les données de la passerelle et les traite
4. **InfluxDB**: Stocke les données temporelles des capteurs
5. **Grafana**: Visualise les données des capteurs

## Prérequis

- Docker et Docker Compose
- Modules LoRa matériels configurés (passerelle et capteurs)
- Connexion réseau entre la passerelle LoRa et le serveur

## Installation

1. Clonez ce dépôt:
   ```bash
   git clone <url-du-repo>
   cd <nom-du-repo>
   ```

2. Créez la structure de dossiers:
   ```bash
   mkdir -p backend/src influxdb/config \
           grafana/provisioning/datasources grafana/provisioning/dashboards
   ```

3. Copiez les fichiers de code et de configuration dans les dossiers appropriés.

## Configuration

### Configuration du backend Rust

Le backend expose une API REST qui reçoit les données des capteurs. L'endpoint principal est:

- **POST /api/sensor**: Reçoit les données des capteurs au format JSON
- **GET /health**: Endpoint pour vérifier l'état du service

### Configuration de la passerelle LoRa matérielle

Vous devez configurer votre passerelle LoRa matérielle pour qu'elle envoie les données reçues des capteurs à l'API du backend:

- URL: `http://<adresse-ip-du-serveur>:8080/api/sensor`
- Méthode: POST
- Format: JSON avec au minimum le champ `device_id` et les données des capteurs

Exemple de payload JSON:
```json
{
  "device_id": "lora-sensor-01",
  "temperature": 25.5,
  "humidity": 65.2,
  "battery": 3.8
}
```

## Lancement

1. Démarrez les services avec Docker Compose:
   ```bash
   docker-compose up -d
   ```

2. Vérifiez que tous les services sont en cours d'exécution:
   ```bash
   docker-compose ps
   ```

## Accès aux interfaces

- **Backend Rust**: http://localhost:8080
  - Endpoint pour les données des capteurs: http://localhost:8080/api/sensor (POST)
  - Endpoint de santé: http://localhost:8080/health (GET)

- **InfluxDB**: http://localhost:8086
  - Identifiants par défaut: admin/adminpassword
  - Organisation: iot-org
  - Bucket: iot-data

- **Grafana**: http://localhost:3000
  - Identifiants par défaut: admin/admin

## Test du système

Pour tester le backend sans la passerelle matérielle, vous pouvez envoyer une requête HTTP simulant des données de capteur:

```bash
curl -X POST http://localhost:8080/api/sensor \
  -H "Content-Type: application/json" \
  -d '{"device_id":"test-sensor","temperature":25.5,"humidity":65.2,"battery":3.8}'
```

## Dépannage

### Vérification des logs

Pour voir les logs d'un service spécifique:
```bash
docker-compose logs -f <service>
```

Où `<service>` peut être `rust-backend`, `influxdb` ou `grafana`.

### Problèmes de connexion

Si votre passerelle LoRa ne peut pas se connecter au backend:

1. Vérifiez que le backend est accessible depuis le réseau de la passerelle
2. Vérifiez que le port 8080 est ouvert sur le serveur
3. Assurez-vous que le format des données envoyées correspond à celui attendu par le backend

## Développement futur

Dans les versions futures, nous prévoyons d'ajouter:

1. Intégration complète avec InfluxDB pour le stockage permanent des données
2. Tableaux de bord Grafana préconfigurés pour différents types de capteurs
3. Système d'alerte basé sur les seuils de données
4. Interface d'administration pour gérer les capteurs

## Sécurité

Pour un déploiement en production, n'oubliez pas de:

1. Changer tous les mots de passe par défaut
2. Activer HTTPS pour toutes les communications
3. Mettre en place une authentification pour l'API du backend
4. Configurer des règles de pare-feu appropriées