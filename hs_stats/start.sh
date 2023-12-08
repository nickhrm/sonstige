#!/bin/bash
# Starten Sie den Cron-Dienst
service cron start

# Führen Sie hier weitere Befehle aus, falls erforderlich
# Zum Beispiel das Starten Ihres Rust-Programms, falls es sofort laufen soll
# ./target/release/hs_stats

cargo run --release

# Verhindern Sie, dass der Container beendet wird
# Dies kann erforderlich sein, wenn das Startskript keine lang laufenden Prozesse hat
while true; do sleep 1000; done
