# Verwenden Sie das offizielle Rust-Image als Basis
FROM rust:latest

# Installieren Sie die notwendigen Pakete für Tesseract, OpenSSL und Cron
RUN apt-get update && apt-get install -y \
    tesseract-ocr \
    libssl-dev \
    pkg-config \
    cron \
    && rm -rf /var/lib/apt/lists/*

# Setzen Sie das Arbeitsverzeichnis im Container
WORKDIR /usr/src/hs_stats

# Kopieren Sie die Cargo.toml und Cargo.lock in das Arbeitsverzeichnis
COPY Cargo.toml Cargo.lock ./

# Kopieren Sie das Verzeichnis 'src' in das Arbeitsverzeichnis
COPY src ./src

# Bauen Sie Ihre Anwendung
RUN cargo build --release

# Erstellen Sie das Cronjob-File direkt im Image
RUN echo "*/15 * * * * root cd /usr/src/hs_stats && ./target/release/hs_stats" > /etc/cron.d/hs_stats_cron

# Geben Sie dem Cronjob-File die notwendigen Berechtigungen
RUN chmod 0644 /etc/cron.d/hs_stats_cron

# Wenden Sie das Cronjob-File an
RUN crontab /etc/cron.d/hs_stats_cron

# Kopieren Sie das Startskript in den Container
COPY start.sh /start.sh

# Geben Sie dem Startskript Ausführungsrechte
RUN chmod +x /start.sh

# Führen Sie das Startskript aus
CMD ["/start.sh"]