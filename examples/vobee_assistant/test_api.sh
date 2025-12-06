#!/bin/bash
# Test script for VoBee API Server
# This script demonstrates how to interact with the VoBee API

API_URL="http://localhost:3000"

echo "🧪 Testing VoBee API Server"
echo "=============================="
echo ""

# Color codes
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test 1: Health Check
echo -e "${BLUE}1. Health Check${NC}"
echo "GET $API_URL/api/health"
curl -s "$API_URL/api/health" | jq .
echo ""
echo ""

# Test 2: Send a greeting
echo -e "${BLUE}2. Send Greeting${NC}"
echo "POST $API_URL/api/chat"
curl -s -X POST "$API_URL/api/chat" \
  -H "Content-Type: application/json" \
  -d '{"message":"Hello VoBee!"}' | jq .
echo ""
echo ""

# Test 3: Ask for a joke
echo -e "${BLUE}3. Ask for a Joke${NC}"
echo "POST $API_URL/api/chat"
curl -s -X POST "$API_URL/api/chat" \
  -H "Content-Type: application/json" \
  -d '{"message":"Tell me a joke"}' | jq .
echo ""
echo ""

# Test 4: Get conversation history
echo -e "${BLUE}4. Get Conversation History${NC}"
echo "GET $API_URL/api/chat/history"
curl -s "$API_URL/api/chat/history" | jq .
echo ""
echo ""

# Test 5: Ask for a fun fact
echo -e "${BLUE}5. Ask for Fun Fact${NC}"
echo "POST $API_URL/api/chat"
curl -s -X POST "$API_URL/api/chat" \
  -H "Content-Type: application/json" \
  -d '{"message":"Tell me a fun fact"}' | jq .
echo ""
echo ""

# Test 6: Clear history
echo -e "${BLUE}6. Clear History${NC}"
echo "DELETE $API_URL/api/chat/history"
curl -s -X DELETE "$API_URL/api/chat/history" -w "\nHTTP Status: %{http_code}\n"
echo ""
echo ""

# Test 7: Verify history is empty
echo -e "${BLUE}7. Verify History is Empty${NC}"
echo "GET $API_URL/api/chat/history"
curl -s "$API_URL/api/chat/history" | jq .
echo ""
echo ""

echo -e "${GREEN}✅ All tests completed!${NC}"
