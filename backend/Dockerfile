FROM rust:latest

WORKDIR /app

# Installer les dépendances
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Copier les fichiers du projet
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Compiler le projet
RUN cargo build --release

# Exposer le port
EXPOSE 8080

# Démarrer l'application
CMD ["./target/release/backend"]