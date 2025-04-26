#!/bin/bash
# Health check endpoint timing
curl -w "\n\nTime statistics for health endpoint:\n\
  DNS lookup time: %{time_namelookup}s\n\
  TCP connect time: %{time_connect}s\n\
  TLS handshake time: %{time_appconnect}s\n\
  Pre-transfer time: %{time_pretransfer}s\n\
  Redirect time: %{time_redirect}s\n\
  Time to first byte: %{time_starttransfer}s\n\
  Total time: %{time_total}s\n" \
  -o /dev/null -s http://localhost:8080/health

# Get all users timing
curl -w "\n\nTime statistics for get all users endpoint:\n\
  DNS lookup time: %{time_namelookup}s\n\
  TCP connect time: %{time_connect}s\n\
  TLS handshake time: %{time_appconnect}s\n\
  Pre-transfer time: %{time_pretransfer}s\n\
  Redirect time: %{time_redirect}s\n\
  Time to first byte: %{time_starttransfer}s\n\
  Total time: %{time_total}s\n" \
  -o /dev/null -s http://localhost:8080/users

# Create user timing
curl -w "\n\nTime statistics for create user endpoint:\n\
  DNS lookup time: %{time_namelookup}s\n\
  TCP connect time: %{time_connect}s\n\
  TLS handshake time: %{time_appconnect}s\n\
  Pre-transfer time: %{time_pretransfer}s\n\
  Redirect time: %{time_redirect}s\n\
  Time to first byte: %{time_starttransfer}s\n\
  Total time: %{time_total}s\n" \
  -o /dev/null -s -X POST \
  -H "Content-Type: application/json" \
  -d '{"name":"John Smith","email":"john@example.com","age":35}' \
  http://localhost:8080/users

# Get user by ID timing (first create a user and capture the ID)
USER_ID=$(curl -s -X POST \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Doe","email":"jane@example.com","age":28}' \
  http://localhost:8080/users | jq -r '.id')

curl -w "\n\nTime statistics for get user by ID endpoint:\n\
  DNS lookup time: %{time_namelookup}s\n\
  TCP connect time: %{time_connect}s\n\
  TLS handshake time: %{time_appconnect}s\n\
  Pre-transfer time: %{time_pretransfer}s\n\
  Redirect time: %{time_redirect}s\n\
  Time to first byte: %{time_starttransfer}s\n\
  Total time: %{time_total}s\n" \
  -o /dev/null -s http://localhost:8080/users/$USER_ID

# Update user timing
curl -w "\n\nTime statistics for update user endpoint:\n\
  DNS lookup time: %{time_namelookup}s\n\
  TCP connect time: %{time_connect}s\n\
  TLS handshake time: %{time_appconnect}s\n\
  Pre-transfer time: %{time_pretransfer}s\n\
  Redirect time: %{time_redirect}s\n\
  Time to first byte: %{time_starttransfer}s\n\
  Total time: %{time_total}s\n" \
  -o /dev/null -s -X PUT \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Updated","email":"jane.updated@example.com"}' \
  http://localhost:8080/users/$USER_ID

# Delete user timing
curl -w "\n\nTime statistics for delete user endpoint:\n\
  DNS lookup time: %{time_namelookup}s\n\
  TCP connect time: %{time_connect}s\n\
  TLS handshake time: %{time_appconnect}s\n\
  Pre-transfer time: %{time_pretransfer}s\n\
  Redirect time: %{time_redirect}s\n\
  Time to first byte: %{time_starttransfer}s\n\
  Total time: %{time_total}s\n" \
  -o /dev/null -s -X DELETE \
  http://localhost:8080/users/$USER_ID