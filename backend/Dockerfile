FROM rust:latest

WORKDIR /app

# Copier les fichiers source
COPY . .

# Compiler l'application
RUN cargo build --release

# Exposer le port
EXPOSE 8080

# Lancer l'application
CMD ["./target/release/backend"]