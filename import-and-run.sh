#!/bin/bash

# Script để import và chạy Docker image

set -e

IMAGE_FILE="cloudflare-dns-updater.tar.gz"

if [ ! -f "$IMAGE_FILE" ]; then
    echo "❌ File $IMAGE_FILE không tìm thấy!"
    echo "Vui lòng đảm bảo file image đã được copy vào thư mục này."
    exit 1
fi

echo "📦 Loading Docker image từ $IMAGE_FILE..."
gunzip -c $IMAGE_FILE | docker load

echo "📋 Checking .env file..."
if [ ! -f ".env" ]; then
    echo "⚠️  File .env không tìm thấy. Tạo từ template..."
    cp .env.example .env
    echo "✏️  Vui lòng chỉnh sửa file .env với thông tin Cloudflare của bạn trước khi chạy!"
    echo "📝 nano .env"
    exit 0
fi

echo "🚀 Starting Docker container..."
docker-compose up -d

echo "✅ Container đã được khởi động!"
echo "📋 Kiểm tra logs: docker-compose logs -f"
echo "🛑 Dừng service: docker-compose down"
