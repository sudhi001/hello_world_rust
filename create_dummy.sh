#!/bin/bash
# Benchmark script to test API performance

# Configuration
ITERATIONS=100000
HOST="http://localhost:8080"

# Function to calculate average
calculate_average() {
    local sum=0
    local count=0
    
    for time in "$@"; do
        sum=$(echo "$sum + $time" | bc -l)
        count=$((count + 1))
    done
    
    if [ $count -eq 0 ]; then
        echo "0"
    else
        echo "scale=6; $sum / $count" | bc -l
    fi
}

# Test create user endpoint
echo "Testing create user endpoint"
create_times=()
for i in $(seq 1 $ITERATIONS); do
    echo -n "."
    time=$(curl -s -X POST -w "%{time_total}" -o /dev/null \
      -H "Content-Type: application/json" \
      -d "{\"name\":\"User $i\",\"email\":\"user$i@example.com\",\"age\":30}" \
      $HOST/users)
    create_times+=($time)
done
echo ""
avg_create=$(calculate_average "${create_times[@]}")
echo "Average response time for create user endpoint: ${avg_create}s"

# # Clean up - delete the test user
# echo "Cleaning up - deleting test user"
# curl -s -X DELETE $HOST/users/$USER_ID > /dev/null

# Print summary
echo ""
echo "BENCHMARK SUMMARY"
echo "================="
echo "Iterations per endpoint: $ITERATIONS"
echo "Health endpoint: ${avg_health}s"
echo "Get all users endpoint: ${avg_users}s"
echo "Get user by ID endpoint: ${avg_user_id}s"
echo "Create user endpoint: ${avg_create}s"