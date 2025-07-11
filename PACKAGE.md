# Package Files for Deployment

Tập hợp các file cần thiết để deploy ứng dụng Cloudflare DNS Updater trên máy khác:

## 📦 Files cần copy sang máy production:

### 1. Docker Image (tạo sau khi build):

```
cloudflare-dns-updater.tar.gz
```

### 2. Configuration Files:

```
docker-compose.yml          # Docker compose config
.env.example                # Template environment variables
```

### 3. Scripts:

```
import-and-run.sh           # Script tự động import và chạy
```

### 4. Documentation:

```
DEPLOYMENT.md               # Hướng dẫn chi tiết
README.md                   # Hướng dẫn tổng quan
```

## 🚀 Quick Start cho Production:

1. **Tạo thư mục:**

   ```bash
   mkdir cloudflare-dns-updater
   cd cloudflare-dns-updater
   ```

2. **Copy các files trên vào thư mục**

3. **Chạy script:**

   ```bash
   chmod +x import-and-run.sh
   ./import-and-run.sh
   ```

4. **Cấu hình .env và restart:**
   ```bash
   nano .env
   docker-compose restart
   ```

## 📋 Minimum Required Files:

Nếu muốn tối giản, chỉ cần:

- `cloudflare-dns-updater.tar.gz` (Docker image)
- `docker-compose.yml` (Docker config)
- `.env` (với thông tin Cloudflare thật)

Sau đó chạy:

```bash
gunzip -c cloudflare-dns-updater.tar.gz | docker load
docker-compose up -d
```

## 🔑 Environment Variables cần thiết:

```env
CLOUDFLARE_ZONE_ID=your_zone_id_here
CLOUDFLARE_API_TOKEN=your_api_token_here
DNS_RECORD_ID=your_dns_record_id_here
DNS_NAME=subdomain_name
CLOUDFLARE_DOMAIN=yourdomain.com
```

## 📊 Kích thước dự kiến:

- Docker image: ~50-100MB (compressed)
- Config files: <1MB
- Total package: ~50-100MB
