#!/bin/bash

# Script d'initialisation pour Grafana
# Remplace les variables d'environnement dans les fichiers de configuration

set -e

echo "🔧 Configuration de Grafana..."

# Vérifier que toutes les variables requises sont définies
if [ -z "$INFLUXDB_TOKEN" ]; then
    echo "❌ INFLUXDB_TOKEN doit être défini !" >&2
    exit 1
fi
if [ -z "$INFLUXDB_ORG" ]; then
    echo "❌ INFLUXDB_ORG doit être défini !" >&2
    exit 1
fi
if [ -z "$INFLUXDB_BUCKET" ]; then
    echo "❌ INFLUXDB_BUCKET doit être défini !" >&2
    exit 1
fi

echo "📊 Configuration InfluxDB:"
echo "   Token: ${INFLUXDB_TOKEN:0:8}..."
echo "   Org: ${INFLUXDB_ORG}"
echo "   Bucket: ${INFLUXDB_BUCKET}"

# Remplacer les variables dans le fichier de datasource
if [ -f "/etc/grafana/provisioning/datasources/influxdb.yml" ]; then
    echo "🔧 Mise à jour de la configuration InfluxDB..."
    sed -i "s/\${INFLUXDB_TOKEN}/${INFLUXDB_TOKEN}/g" /etc/grafana/provisioning/datasources/influxdb.yml
    sed -i "s/\${INFLUXDB_ORG}/${INFLUXDB_ORG}/g" /etc/grafana/provisioning/datasources/influxdb.yml
    sed -i "s/\${INFLUXDB_BUCKET}/${INFLUXDB_BUCKET}/g" /etc/grafana/provisioning/datasources/influxdb.yml
    echo "✅ Configuration InfluxDB mise à jour"
else
    echo "⚠️  Fichier de datasource InfluxDB non trouvé"
fi

echo "✅ Configuration Grafana terminée" 