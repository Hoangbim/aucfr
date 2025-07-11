#!/bin/bash

# Script để build và export Docker image

set -e

IMAGE_NAME="cloudflare-dns-updater"
VERSION="latest"
EXPORT_FILE="cloudflare-dns-updater.tar"

echo "🔨 Building Docker image..."
docker build -t ${IMAGE_NAME}:${VERSION} .

echo "📦 Exporting Docker image to ${EXPORT_FILE}..."
docker save -o ${EXPORT_FILE} ${IMAGE_NAME}:${VERSION}

echo "🗜️ Compressing image..."
gzip -f ${EXPORT_FILE}

echo "✅ Docker image exported successfully!"
echo "📁 File: ${EXPORT_FILE}.gz"
echo "📊 Size: $(du -h ${EXPORT_FILE}.gz | cut -f1)"

echo ""
echo "🚀 To use on another machine:"
echo "1. Transfer ${EXPORT_FILE}.gz to target machine"
echo "2. Load image: gunzip -c ${EXPORT_FILE}.gz | docker load"
echo "3. Create .env file with your Cloudflare credentials"
echo "4. Run: docker-compose up -d"
