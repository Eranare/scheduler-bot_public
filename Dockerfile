# Stage 1: Build the application
FROM ubuntu:20.04 as builder

# Set non-interactive timezone configuration
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC

# Install necessary build tools and libraries
RUN apt-get update && \
    apt-get install -y build-essential cmake pkg-config libssl-dev curl && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set up Rust environment
ENV PATH="/root/.cargo/bin:${PATH}"

# Create a working directory
WORKDIR /usr/src/myapp

# Copy Cargo.toml and Cargo.lock and create a dummy main.rs to cache dependencies
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN mkdir src && echo "fn main() {}" > src/main.rs

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Now copy in your application source and build it
COPY ./src ./src
RUN cargo install --path .

# Stage 2: Create the runtime image
FROM ubuntu:20.04

# Install necessary libraries (including OpenSSL)
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/myapp/target/release/disc-bot /usr/local/bin/disc-bot

# Copy your resources/images directory from project root to the container
COPY resources/images /usr/local/bin/resources/images

# Override the default command to keep the container running
CMD ["disc-bot"]
