# Sử dụng Rust official image để build
FROM rust:1.82 AS builder

WORKDIR /app

# Copy Cargo files để cache dependencies
COPY Cargo.toml Cargo.lock ./

# Tạo dummy main.rs để build dependencies trước
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src ./src

# Build application
RUN cargo build --release

# Runtime stage - sử dụng debian slim để giảm kích thước image
FROM debian:bookworm-slim

# Cài đặt các dependencies cần thiết cho runtime
RUN apt-get update && \
    apt-get install -y \
        ca-certificates \
        libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Tạo user non-root để chạy application
RUN useradd -r -s /bin/false appuser

WORKDIR /app

# Copy binary từ builder stage
COPY --from=builder /app/target/release/update-cloudflare-record .

# Chuyển ownership cho appuser
RUN chown appuser:appuser /app/update-cloudflare-record

# Switch to non-root user
USER appuser

# Expose port nếu cần (không cần thiết cho app này nhưng để làm ví dụ)
# EXPOSE 8080

# Health check (optional)
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD pgrep -f update-cloudflare-record > /dev/null || exit 1

# Command để chạy application
CMD ["./update-cloudflare-record"]
