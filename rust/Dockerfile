# Use the official Rust image as a base
FROM rust:latest

# Install any additional system dependencies if needed

# Create a working directory
WORKDIR /app

# Copy the Rust source code into the container
COPY cmr/queries.rs .

# Build the Rust project
RUN cargo build --release

# Set the entry point for the container
CMD ["./target/release/your_executable_name"]