# Use the official Rust runtime as a parent image
FROM rust:1.86.0-slim-bookworm

# # 1. Install system dependencies
# RUN apt-get update && \
#     apt-get install -y \
#     pkg-config \
#     libssl-dev \
#     openssl \
#     ca-certificates \
#     && rm -rf /var/lib/apt/lists/*

# 2. Create app directory
# WORKDIR /app

# 3. Copy your pre-built binary (from your local target/release)
# COPY target/release/onchain_agent /app/onchain_agent

# 4. Copy any necessary files (like .env, config files)
# COPY .env /app/

# # 5. Set environment variables (adjust as needed)
# ENV RUST_LOG=info
# ENV PORT=8081
# EXPOSE 8081

# 6. Run the application
# CMD ["cargo run"]