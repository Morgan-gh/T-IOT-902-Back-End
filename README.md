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
