# Quick Start Guide - VoBee API Server

Get started with the VoBee API server in minutes!

## Option 1: Run Locally with Cargo (Fastest)

### Prerequisites
- Rust 1.91 or later
- Cargo

### Steps

1. **Navigate to the vobee_assistant directory:**
   ```bash
   cd examples/vobee_assistant
   ```

2. **Start the API server:**
   ```bash
   cargo run --bin vobee_api
   ```

3. **Test the API:**
   ```bash
   # In another terminal
   curl http://localhost:3000/api/health
   ```

That's it! The server is running on `http://localhost:3000`.

## Option 2: Run with Docker Compose (Production-Ready)

### Prerequisites
- Docker
- Docker Compose

### Steps

1. **Start the server:**
   ```bash
   docker-compose up
   ```

2. **Test the API:**
   ```bash
   curl http://localhost:3000/api/health
   ```

The server will automatically restart if it crashes.

## Testing the API

### Using the provided test script:
```bash
./test_api.sh
```

### Using curl manually:

**Health Check:**
```bash
curl http://localhost:3000/api/health
```

**Send a message:**
```bash
curl -X POST http://localhost:3000/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message":"Hello VoBee!"}'
```

**Get conversation history:**
```bash
curl http://localhost:3000/api/chat/history
```

**Clear conversation history:**
```bash
curl -X DELETE http://localhost:3000/api/chat/history
```

## Integrating with FlutterFlow

1. **Start the VoBee API server** (using either option above)

2. **In FlutterFlow:**
   - Go to **API Calls** → **+ Add API Call**
   - Name: `VoBeeChat`
   - Method: `POST`
   - URL: `http://localhost:3000/api/chat` (or your deployed URL)
   - Body: `{"message": "[your_variable]"}`

3. **Use in your app:**
   - Create a text field for user input
   - Add a send button that calls the `VoBeeChat` API
   - Display the response in your UI

See [FLUTTERFLOW_INTEGRATION.md](FLUTTERFLOW_INTEGRATION.md) for detailed integration instructions.

## Integrating with Flutter/Dart

Copy the example client from [example_flutter_integration.dart](example_flutter_integration.dart) into your Flutter project:

```dart
final client = VoBeeClient(baseUrl: 'http://localhost:3000');
final response = await client.sendMessage('Hello!');
print(response.response);
```

## Common Issues

### Port 3000 already in use
Change the port:
```bash
PORT=8080 cargo run --bin vobee_api
```

### CORS errors in the browser
The API has CORS enabled for all origins. If you still get CORS errors:
1. Make sure the API server is running
2. Check that you're using the correct URL
3. Verify your browser's console for the exact error

### Connection refused
1. Verify the server is running: `curl http://localhost:3000/api/health`
2. Check if a firewall is blocking the connection
3. Ensure you're using the correct port

## Next Steps

- Read the [full API documentation](FLUTTERFLOW_INTEGRATION.md)
- Check out the [security considerations](SECURITY_SUMMARY.md)
- Explore the [Flutter integration example](example_flutter_integration.dart)
- Deploy to production using Docker

## Support

For issues or questions, please check the documentation or open an issue on GitHub.

---

Happy coding! 🐝
