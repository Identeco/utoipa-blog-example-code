# Build the binary in the foreground
cargo build --bin server
# Start the server (or a dummy) in the background
cargo run --bin server &
# Wait for the server to be ready
wait-for-it localhost:8080 --timeout=30
# Run the `schemathesis` tests
st run openapi.yml \
    --checks all \
    --data-generation-method all \
    --validate-schema true \
    --base-url http://localhost:8080 \
    -H "My-Api-Key: 123456"
# Kill the server to free the address
kill %%
