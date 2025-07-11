# Dockerfile và Docker Compose cho Update Cloudflare Record

## Cấu trúc File

### Dockerfile

- **Multi-stage build**: Sử dụng Rust official image để build, sau đó chuyển sang Debian slim để runtime
- **Security**: Chạy ứng dụng với non-root user
- **Optimization**: Cache dependencies để build nhanh hơn
- **Health check**: Kiểm tra trạng thái ứng dụng

### Docker Compose

- **Environment variables**: Cấu hình thông qua biến môi trường
- **Restart policy**: Tự động restart khi container bị crash
- **Resource limits**: Giới hạn CPU và memory
- **Network**: Tạo network riêng cho service
- **Volumes**: Mount .env file và logs directory

## Cách sử dụng

### 1. Chuẩn bị môi trường

```bash
# Copy file .env.example thành .env
cp .env.example .env

# Chỉnh sửa các giá trị trong .env
nano .env
```

### 2. Build và chạy với Docker Compose

```bash
# Build và chạy service
docker-compose up -d

# Xem logs
docker-compose logs -f

# Dừng service
docker-compose down
```

### 3. Chỉ sử dụng Docker

```bash
# Build image
docker build -t update-cloudflare-record .

# Chạy container
docker run -d \
  --name cloudflare-dns-updater \
  --env-file .env \
  --restart unless-stopped \
  update-cloudflare-record
```

## Tùy chọn nâng cao

### Development với volume mount

Để development, bạn có thể mount source code:

```yaml
volumes:
  - ./src:/app/src:ro
  - ./Cargo.toml:/app/Cargo.toml:ro
```

### Production với external network

```yaml
networks:
  default:
    external: true
    name: production-network
```

### Monitoring với labels

```yaml
labels:
  - "traefik.enable=false"
  - "prometheus.scrape=true"
  - "prometheus.port=8080"
```

## Lưu ý

- Đảm bảo các biến môi trường trong `.env` được cấu hình đúng
- Container sẽ tự động restart nếu ứng dụng bị crash
- Logs được lưu trong thư mục `./logs`
- Health check sẽ kiểm tra mỗi 30 giây
