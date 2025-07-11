# Docker trên Windows - Hướng dẫn cài đặt và sử dụng

## 🪟 **Cài đặt Docker trên Windows**

### **Yêu cầu hệ thống:**

- Windows 10 (build 19041+) hoặc Windows 11
- WSL 2 (khuyến nghị)
- Virtualization enabled trong BIOS

### **Các bước cài đặt:**

1. **Cài Docker Desktop:**

   ```powershell
   # Download từ: https://desktop.docker.com/win/main/amd64/Docker%20Desktop%20Installer.exe
   # Hoặc dùng winget
   winget install Docker.DockerDesktop
   ```

2. **Enable WSL 2:**

   ```powershell
   # Mở PowerShell as Administrator
   dism.exe /online /enable-feature /featurename:Microsoft-Windows-Subsystem-Linux /all /norestart
   dism.exe /online /enable-feature /featurename:VirtualMachinePlatform /all /norestart

   # Restart máy, sau đó:
   wsl --set-default-version 2
   ```

3. **Khởi động Docker Desktop và enable WSL 2 integration**

## 🚀 **Sử dụng Docker Compose trên Windows**

### **PowerShell/CMD:**

```cmd
# Clone hoặc copy project folder
cd C:\path\to\update-cloudflare-record

# Copy .env file
copy .env.example .env

# Edit .env với notepad hoặc VS Code
notepad .env

# Tạo thư mục logs
mkdir logs

# Build image (nếu cần)
docker build -t cloudflare-dns-updater:latest .

# Chạy với docker-compose
docker-compose -f docker-compose.windows.yml up -d

# Hoặc dùng file mặc định
docker-compose up -d
```

### **Git Bash/WSL:**

```bash
# Chạy các lệnh Linux thông thường
cp .env.example .env
nano .env
mkdir -p logs
docker-compose up -d
```

## 📂 **Đường dẫn Volume trên Windows**

### **Cách 1: Relative paths (khuyến nghị)**

```yaml
volumes:
  - ./.env:/app/.env:ro
  - ./logs:/app/logs
```

### **Cách 2: Absolute paths**

```yaml
volumes:
  - C:/Users/YourUser/project/.env:/app/.env:ro
  - C:/Users/YourUser/project/logs:/app/logs
```

### **Cách 3: WSL paths**

```yaml
volumes:
  - /mnt/c/Users/YourUser/project/.env:/app/.env:ro
  - /mnt/c/Users/YourUser/project/logs:/app/logs
```

## 🔧 **Script Commands trên Windows**

### **PowerShell Scripts:**

**start.ps1:**

```powershell
# Kiểm tra Docker đang chạy
$dockerRunning = docker info 2>$null
if (-not $dockerRunning) {
    Write-Host "Starting Docker Desktop..." -ForegroundColor Yellow
    Start-Process "Docker Desktop"
    Start-Sleep 30
}

# Copy .env nếu chưa có
if (-not (Test-Path ".env")) {
    Copy-Item ".env.example" ".env"
    Write-Host "Created .env file. Please edit it with your settings." -ForegroundColor Green
    notepad .env
    Read-Host "Press Enter after editing .env file..."
}

# Create logs directory
if (-not (Test-Path "logs")) {
    New-Item -ItemType Directory -Name "logs"
}

# Start services
docker-compose up -d
Write-Host "Service started! Check status with: docker-compose ps" -ForegroundColor Green
```

**stop.ps1:**

```powershell
docker-compose down
Write-Host "Service stopped!" -ForegroundColor Green
```

### **Batch Scripts:**

**start.bat:**

```batch
@echo off
echo Starting Cloudflare DNS Updater...

REM Check if .env exists
if not exist ".env" (
    copy ".env.example" ".env"
    echo .env file created. Please edit it with your settings.
    notepad .env
    pause
)

REM Create logs directory
if not exist "logs" mkdir logs

REM Start Docker Compose
docker-compose up -d

echo Service started!
echo Check status with: docker-compose ps
pause
```

## 🎯 **Recommended Setup cho Windows**

### **Option 1: WSL 2 + VS Code (Best)**

```bash
# Trong WSL 2 Ubuntu
cd /mnt/c/Users/YourUser/Projects/
git clone [your-repo]
cd update-cloudflare-record
code .  # Mở VS Code với WSL extension
```

### **Option 2: PowerShell + Docker Desktop**

```powershell
# Trong PowerShell
cd C:\Users\YourUser\Projects\update-cloudflare-record
# Sử dụng các script .ps1 ở trên
```

### **Option 3: Git Bash**

```bash
# Trong Git Bash - giống Linux commands
cd /c/Users/YourUser/Projects/update-cloudflare-record
cp .env.example .env
nano .env
docker-compose up -d
```

## 🔍 **Troubleshooting trên Windows**

### **Common Issues:**

1. **Volume mount không hoạt động:**

   ```yaml
   # Thay vì
   volumes:
     - ./logs:/app/logs

   # Thử
   volumes:
     - type: bind
       source: ./logs
       target: /app/logs
   ```

2. **Line ending issues (.env file):**

   ```powershell
   # Convert CRLF to LF
   (Get-Content .env -Raw) -replace "`r`n","`n" | Set-Content .env -NoNewline
   ```

3. **Health check fail:**
   ```yaml
   # Thay vì pgrep, dùng alternative
   healthcheck:
     test: ["CMD", "ps", "aux"]
   ```

## 📱 **VS Code Extensions cho Windows**

```json
{
  "recommendations": [
    "ms-vscode-remote.remote-wsl",
    "ms-azuretools.vscode-docker",
    "ms-vscode-remote.remote-containers"
  ]
}
```

Với setup này, bạn có thể chạy Docker hoàn toàn bình thường trên Windows! 🚀
