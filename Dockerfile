# Étape 1 : Build
FROM rust:1.85 as builder

# Définir le dossier de travail dans le conteneur
WORKDIR /app

# Installer les dépendances système requises (dont OpenSSL)
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev

# Copier le fichier de configuration Cargo en premier (pour gérer le cache efficacement)
COPY Cargo.toml Cargo.lock ./

# Télécharger les dépendances sans compiler
RUN cargo fetch

# Copier le reste du code source dans le conteneur
COPY src ./src

# Compiler en mode release
RUN cargo build --release

# Étape 2 : Image minimale pour l'exécution
FROM debian:buster-slim

# Installer les bibliothèques nécessaires à OpenSSL dans l'image de production
RUN apt-get update && \
    apt-get install -y openssl ca-certificates

# Définir le dossier de travail
WORKDIR /app

# Copier le binaire depuis le premier conteneur
COPY --from=builder /app/target/release/back-iot /app/back-iot

# Exposer le port de l'application
EXPOSE 8080

# Lancer le serveur Actix
CMD ["./back-iot"]