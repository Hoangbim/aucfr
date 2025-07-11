# Hướng Dẫn Sử Dụng Docker Image

## 🔨 Build và Export Image

### Trên máy development:

1. **Build và export image:**

   ```bash
   ./build-and-export.sh
   ```

   Script này sẽ:

   - Build Docker image từ source code
   - Export image thành file `cloudflare-dns-updater.tar.gz`
   - Hiển thị kích thước file và hướng dẫn

2. **Transfer file:**
   - Copy file `cloudflare-dns-updater.tar.gz` lên server/máy đích
   - Cùng với file `docker-compose.yml`, `.env.example`, và `import-and-run.sh`

## 📦 Import và Chạy Image

### Trên máy production:

1. **Chuẩn bị files:**

   ```
   /your-app-directory/
   ├── cloudflare-dns-updater.tar.gz
   ├── docker-compose.yml
   ├── .env.example
   └── import-and-run.sh
   ```

2. **Import và chạy:**

   ```bash
   ./import-and-run.sh
   ```

3. **Cấu hình environment:**

   ```bash
   nano .env
   ```

   Điền thông tin Cloudflare:

   ```env
   CLOUDFLARE_ZONE_ID=your_zone_id
   CLOUDFLARE_API_TOKEN=your_api_token
   DNS_RECORD_ID=your_record_id
   DNS_NAME=your_subdomain
   CLOUDFLARE_DOMAIN=yourdomain.com
   ```

4. **Khởi động service:**
   ```bash
   docker-compose up -d
   ```

## 🛠️ Quản Lý Service

```bash
# Xem logs
docker-compose logs -f

# Restart service
docker-compose restart

# Dừng service
docker-compose down

# Xem status
docker-compose ps

# Update config và restart
nano .env
docker-compose restart
```

## 📋 Lệnh Thủ Công

### Import image thủ công:

```bash
gunzip -c cloudflare-dns-updater.tar.gz | docker load
```

### Chạy container thủ công:

```bash
docker run -d \
  --name cloudflare-dns-updater \
  --env-file .env \
  --restart unless-stopped \
  cloudflare-dns-updater:latest
```

### Xem logs:

```bash
docker logs -f cloudflare-dns-updater
```

## 🔧 Troubleshooting

### Kiểm tra image đã load chưa:

```bash
docker images | grep cloudflare-dns-updater
```

### Kiểm tra container đang chạy:

```bash
docker ps | grep cloudflare-dns-updater
```

### Debug container:

```bash
docker exec -it cloudflare-dns-updater sh
```

### Xóa và tạo lại:

```bash
docker-compose down
docker rmi cloudflare-dns-updater:latest
./import-and-run.sh
```

## 📊 Monitoring

Service sẽ log thông tin về:

- IP public hiện tại
- Thay đổi IP
- Kết quả update DNS
- Lỗi nếu có

Logs sẽ hiển thị emoji để dễ đọc:

- 🌐 IP hiện tại
- 🔄 IP thay đổi
- ✅ Update thành công
- ❌ Lỗi
- ⚠️ Cảnh báo
