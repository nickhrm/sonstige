# Verwenden Sie das offizielle Rust-Image als Basis
FROM rust:latest

# Installieren Sie die notwendigen Pakete für Tesseract und OpenSSL
# Für Debian-basierte Images
RUN apt-get update && apt-get install -y \
    tesseract-ocr \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Setzen Sie das Arbeitsverzeichnis im Container
WORKDIR /usr/src/hs_stats

# Kopieren Sie die Cargo.toml und Cargo.lock in das Arbeitsverzeichnis
COPY Cargo.toml Cargo.lock ./

# Kopieren Sie Ihren Quellcode in das Arbeitsverzeichnis
COPY src ./src

# Bauen Sie Ihre Anwendung
RUN cargo install --path .

# Stellen Sie den Port bereit, auf dem die Anwendung läuft
#EXPOSE 8080

# Führen Sie die Anwendung aus
CMD ["cargo", "run", "--release"]