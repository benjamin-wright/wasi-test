# Use the official Rust image as the base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the project files into the container
COPY Cargo* ./
COPY src ./src

# Statically link the C runtime library
ENV RUSTFLAGS="-C target-feature=+crt-static"

# Build the project
RUN cargo build --target=aarch64-unknown-linux-gnu --release

RUN ls -la /app/target/aarch64-unknown-linux-gnu/release

# Create a new stage for the final image
FROM scratch

# Set the working directory inside the container
WORKDIR /app

# Copy the built binary from the previous stage
COPY --from=builder /app/target/aarch64-unknown-linux-gnu/release/wasm .

# Set the entrypoint for the container
ENTRYPOINT ["./wasm"]