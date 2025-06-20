# Neo GUI Docker Build Environment
FROM rust:1.75-bookworm

# Install system dependencies
RUN apt-get update && apt-get install -y \
    # Tauri dependencies
    libwebkit2gtk-4.0-dev \
    libjavascriptcoregtk-4.1-dev \
    libsoup-3.0-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    # Build tools
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    # Additional tools
    pkg-config \
    # Clean up
    && rm -rf /var/lib/apt/lists/*

# Install Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs

# Create app directory
WORKDIR /app

# Copy the entire project
COPY .. .

# Build the GUI
WORKDIR /app/neo-gui

# Install npm dependencies
RUN npm install

# Build the Tauri application
RUN npm run tauri build

# The built application will be in src-tauri/target/release/
# You can extract it with:
# docker cp <container_id>:/app/neo-gui/src-tauri/target/release/neo-gui ./neo-gui-binary