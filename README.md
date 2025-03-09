# Backend en Rust avec Actix

Ce projet est un backend simple en **Rust** utilisant le framework **Actix** pour la gestion des routes et des requêtes HTTP.

## 🛠️ Installation

### 1. Installer Rust
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

### 2. Créer un projet Rust
Créez un nouveau projet avec **Cargo** :

``` bash
cargo new mon-backend
cd mon-backend
```

---

### 3. Ajouter les dépendances Actix
Modifiez le fichier **Cargo.toml** :

``` toml
[dependencies]
actix-web = "4.0"
serde = { version = "1.0", features = ["derive"] }
serde```json = "1.0"
```

---

### 4. Écrire le code du backend
Remplacez le contenu de **src/main.rs** par ce code :

``` rust
use actix```web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Data {
    message: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust backend!")
}

#[post("/echo")]
async fn echo(data: web::Json<Data>) -> impl Responder {
    HttpResponse::Ok().json(Data {
        message: format!("You sent: {}", data.message),
    })
}

#[actix```web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(echo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

---

### 5. Lancer le serveur
Exécutez le serveur avec :

``` bash
cargo run
```

Le serveur sera accessible à l'adresse suivante :
http://127.0.0.1:8080


---

### 6. Tester l'API
**GET** sur `/` :

``` bash
curl http://127.0.0.1:8080/
```

**POST** sur `/echo` :

``` bash
curl -X POST http://127.0.0.1:8080/echo -H "Content-Type: application/json" -d '{"message": "Salut"}'
```

---

### 7. Créer une route dynamique
Exemple d'une route dynamique :

``` rust
#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}
```

---

### 8. Rendre le serveur accessible sur le réseau
Dans `main.rs` :

``` rust
HttpServer::new(|| {
    App::new()
        .service(index)
})
.bind("0.0.0.0:8080")?
.run()
.await
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
