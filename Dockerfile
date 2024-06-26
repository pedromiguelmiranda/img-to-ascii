# Stage 1: Build the application and run tests
FROM rust as builder

# Create a directory for the application
WORKDIR /usr/src/img_to_ascii

# Copy the Cargo.toml and Cargo.lock files to build dependencies first
COPY Cargo.toml Cargo.lock ./

# Copy the actual source code
COPY src ./src
COPY tests ./tests

# Build the application and run the tests
RUN cargo build   && cargo test 


# Install required dependencies for running the Rust application
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create a non-root user and switch to it
RUN useradd -m appuser
USER appuser

# Set the entrypoint to run the program
ENTRYPOINT ["./target/debug/img_to_ascii"]

# Default command to show usage info
CMD ["--help"]
