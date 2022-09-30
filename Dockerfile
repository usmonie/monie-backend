# Tells docker to use the latest Rust official image
FROM rust:latest

# Copy our current working directory into the container
COPY ./ ./

# Create the release build
RUN cargo build --release
ENV DATABASE_URL=postgres://postgres:password@localhost/monie_test
ENV HOST=127.0.0.1
ENV PORT=9231
# Expose the port our app is running on
EXPOSE 9231
# Run the application!
CMD ["./target/release/monie"]