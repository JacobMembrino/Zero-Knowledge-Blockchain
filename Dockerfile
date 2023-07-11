# Use the official Rust base image
FROM rust:1.69

# Install the necessary build dependencies
RUN apt-get update \
    && apt-get install -y libssl-dev pkg-config \
    && apt-get install -y libclang-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/src/app

# Copy the project's manifest files (Cargo.toml and Cargo.lock) into the container
COPY Cargo.toml Cargo.lock ./

# Build the dependencies to cache them
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build \
    && rm -rf src

# Copy the rest of the application code
COPY . .

# Build the application
RUN cargo build --release

# Set the start command
CMD ["./target/release/zero_knowledge_blockchain"]
