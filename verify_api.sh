#!/bin/bash
set -e

# Start server in background
echo "Starting server..."
cargo run > server.log 2>&1 &
SERVER_PID=$!
echo "Server PID: $SERVER_PID"

# Wait for server to start
sleep 5

# Keygen
echo "Testing Keygen..."
curl -s -X POST http://localhost:3000/keygen > keys.json
echo "Keys received."

# Extract keys (using python if jq not available, assuming python3)
PK=$(python3 -c "import sys, json; print(json.load(open('keys.json'))['public_key'])")
SK=$(python3 -c "import sys, json; print(json.load(open('keys.json'))['secret_key'])")

# Encrypt
echo "Testing Encrypt..."
# Escape quotes in PK if necessary, but base64 usually safe
curl -s -X POST http://localhost:3000/encrypt \
    -H "Content-Type: application/json" \
    -d "$(jq -n --arg pk "$PK" --arg msg "Hello API World" '{public_key: $pk, message: $msg}')" \
    > ciphertext.json
echo "Ciphertext received."

CT=$(python3 -c "import sys, json; print(json.load(open('ciphertext.json'))['ciphertext'])")

# Decrypt
echo "Testing Decrypt..."
curl -s -X POST http://localhost:3000/decrypt \
    -H "Content-Type: application/json" \
    -d "$(jq -n --arg sk "$SK" --arg ct "$CT" '{secret_key: $sk, ciphertext: $ct}')" \
    > decrypted.json

MSG=$(python3 -c "import sys, json; print(json.load(open('decrypted.json'))['message'])")

echo "Decrypted Message: $MSG"

if [ "$MSG" == "Hello API World" ]; then
    echo "VERIFICATION SUCCESSFUL"
else
    echo "VERIFICATION FAILED"
fi

kill $SERVER_PID
