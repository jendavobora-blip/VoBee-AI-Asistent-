# FlutterFlow Integration Guide

This guide explains how to integrate the VoBee AI Assistant with your FlutterFlow applications.

## Overview

VoBee AI Assistant now provides a REST API server that can be integrated with FlutterFlow applications. This allows you to add the VoBee chatbot functionality to your Flutter apps without needing to rewrite the chatbot logic.

## Starting the API Server

### Using the Binary

Run the API server binary:

```bash
cargo run --bin vobee_api
```

By default, the server runs on port 3000. You can change the port using the `PORT` environment variable:

```bash
PORT=8080 cargo run --bin vobee_api
```

### Building and Running in Production

```bash
# Build the release binary
cargo build --release --bin vobee_api

# Run the binary
./target/release/vobee_api
```

## API Endpoints

### Health Check

Check if the server is running.

**Endpoint:** `GET /api/health`

**Response:**
```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

### Send Message

Send a message to VoBee and get a response.

**Endpoint:** `POST /api/chat`

**Request Body:**
```json
{
  "message": "Hello VoBee!"
}
```

**Response:**
```json
{
  "response": "Hi there! 👋 VoBee at your service! What's on your mind?",
  "timestamp": "2025-12-06T12:00:00Z"
}
```

**Error Response (400 Bad Request):**
```json
{
  "error": "Message cannot be empty"
}
```

### Get Conversation History

Retrieve the full conversation history.

**Endpoint:** `GET /api/chat/history`

**Response:**
```json
{
  "messages": [
    {
      "sender": "user",
      "text": "Hello VoBee!",
      "timestamp": "2025-12-06T12:00:00Z"
    },
    {
      "sender": "bot",
      "text": "Hi there! 👋 VoBee at your service!",
      "timestamp": "2025-12-06T12:00:01Z"
    }
  ]
}
```

### Clear Conversation History

Clear all conversation history.

**Endpoint:** `DELETE /api/chat/history`

**Response:** `204 No Content`

## FlutterFlow Integration Steps

### 1. Deploy the API Server

First, deploy the VoBee API server to a hosting platform of your choice:

- **Local Development:** Run on localhost:3000
- **Cloud Platforms:** Deploy to Heroku, Railway, Fly.io, AWS, GCP, etc.
- **VPS:** Deploy to any VPS with Docker or systemd

### 2. Configure API in FlutterFlow

In your FlutterFlow project:

1. Go to **API Calls** in the left sidebar
2. Click **+ Add API Call**
3. Name it "VoBeeChat"
4. Configure the endpoint:
   - Method: `POST`
   - URL: `https://your-server.com/api/chat`
   - Body Type: JSON
   - Body:
     ```json
     {
       "message": "[variable_name]"
     }
     ```

### 3. Create FlutterFlow Variables

Create these App State variables in FlutterFlow:

- `chatMessages` (List of DataType: Message)
  - Fields:
    - `sender` (String)
    - `text` (String)
    - `timestamp` (String)
- `currentMessage` (String)

### 4. Add UI Components

In your FlutterFlow page:

1. **Chat Display:**
   - Add a ListView
   - Bind to `chatMessages`
   - Display `sender` and `text` fields

2. **Input Field:**
   - Add a TextField
   - Bind to `currentMessage`

3. **Send Button:**
   - Add a Button
   - On tap action:
     - API Call: VoBeeChat
     - Pass `currentMessage` as parameter
     - On Success:
       - Parse response JSON
       - Append to `chatMessages`
       - Clear `currentMessage`

### 5. Example FlutterFlow Action Flow

**Send Message Button Action:**

```
1. Add to List: chatMessages
   - sender: "user"
   - text: currentMessage
   - timestamp: currentTime

2. API Call: VoBeeChat
   - message: currentMessage

3. On Success:
   - Add to List: chatMessages
     - sender: "bot"
     - text: response.body.response
     - timestamp: response.body.timestamp

4. Update State: currentMessage = ""
```

## CORS Configuration

The API server is configured with CORS enabled for all origins, making it easy to integrate with FlutterFlow web apps. The CORS settings allow:

- All origins
- All methods (GET, POST, DELETE, etc.)
- All headers

## Example cURL Requests

Test the API using curl:

```bash
# Health check
curl http://localhost:3000/api/health

# Send a message
curl -X POST http://localhost:3000/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello VoBee!"}'

# Get history
curl http://localhost:3000/api/chat/history

# Clear history
curl -X DELETE http://localhost:3000/api/chat/history
```

## Deployment Options

### Docker

Create a `Dockerfile`:

```dockerfile
FROM rust:1.91 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin vobee_api

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/vobee_api /usr/local/bin/
EXPOSE 3000
CMD ["vobee_api"]
```

Build and run:

```bash
docker build -t vobee-api .
docker run -p 3000:3000 vobee-api
```

### systemd Service

Create `/etc/systemd/system/vobee-api.service`:

```ini
[Unit]
Description=VoBee API Server
After=network.target

[Service]
Type=simple
User=vobee
WorkingDirectory=/opt/vobee
ExecStart=/opt/vobee/vobee_api
Restart=always
Environment=PORT=3000

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable vobee-api
sudo systemctl start vobee-api
```

## Security Considerations

For production deployments:

1. **Use HTTPS:** Always deploy with SSL/TLS encryption
2. **Authentication:** Consider adding API key authentication
3. **Rate Limiting:** Implement rate limiting to prevent abuse
4. **CORS:** Restrict CORS to specific origins in production
5. **Input Validation:** The API validates message inputs, but consider additional validation
6. **Monitoring:** Add logging and monitoring for production use

## Troubleshooting

### Server Won't Start

- Check if port 3000 is already in use
- Try a different port: `PORT=8080 cargo run --bin vobee_api`
- Check firewall settings

### CORS Errors

- Ensure the API URL is correct in FlutterFlow
- Check browser console for specific CORS errors
- Verify the server is running and accessible

### Connection Refused

- Verify the server is running
- Check the URL/port in your FlutterFlow configuration
- Ensure no firewall is blocking the connection

## Support

For issues or questions:
- Check the main README.md
- Open an issue on GitHub
- Review the API server logs for error messages

## Example Code

### Test Script

A bash script to test all API endpoints is available:

```bash
./test_api.sh
```

This script tests all endpoints and displays the responses in a formatted way.

### Flutter/Dart Integration

A complete Flutter/Dart example showing how to integrate VoBee in your Flutter or FlutterFlow app is available in `example_flutter_integration.dart`. This includes:

- Complete API client implementation
- Request/response models
- Example widget code
- Error handling

You can copy this code directly into your FlutterFlow Custom Code section or use it in any Flutter project.
