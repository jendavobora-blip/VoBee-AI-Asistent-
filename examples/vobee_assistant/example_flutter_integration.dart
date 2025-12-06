// Example Flutter/Dart integration with VoBee API
// This file demonstrates how to integrate VoBee chatbot in a Flutter app
// Copy this code into your FlutterFlow Custom Code or Flutter project

import 'dart:convert';
import 'package:http/http.dart' as http;

/// VoBee API Client
/// 
/// Usage in FlutterFlow:
/// 1. Add this as Custom Code (Backend Code)
/// 2. Set your API URL in the constructor
/// 3. Call the methods from your UI actions
class VoBeeClient {
  final String baseUrl;

  VoBeeClient({this.baseUrl = 'http://localhost:3000'});

  /// Send a message to VoBee and get a response
  Future<ChatResponse> sendMessage(String message) async {
    final url = Uri.parse('$baseUrl/api/chat');
    
    try {
      final response = await http.post(
        url,
        headers: {'Content-Type': 'application/json'},
        body: json.encode({'message': message}),
      );

      if (response.statusCode == 200) {
        final data = json.decode(response.body);
        return ChatResponse(
          response: data['response'],
          timestamp: DateTime.parse(data['timestamp']),
        );
      } else {
        throw Exception('Failed to send message: ${response.statusCode}');
      }
    } catch (e) {
      throw Exception('Error sending message: $e');
    }
  }

  /// Get conversation history
  Future<List<Message>> getHistory() async {
    final url = Uri.parse('$baseUrl/api/chat/history');
    
    try {
      final response = await http.get(url);

      if (response.statusCode == 200) {
        final data = json.decode(response.body);
        final messages = (data['messages'] as List)
            .map((msg) => Message.fromJson(msg))
            .toList();
        return messages;
      } else {
        throw Exception('Failed to get history: ${response.statusCode}');
      }
    } catch (e) {
      throw Exception('Error getting history: $e');
    }
  }

  /// Clear conversation history
  Future<void> clearHistory() async {
    final url = Uri.parse('$baseUrl/api/chat/history');
    
    try {
      final response = await http.delete(url);

      if (response.statusCode != 204) {
        throw Exception('Failed to clear history: ${response.statusCode}');
      }
    } catch (e) {
      throw Exception('Error clearing history: $e');
    }
  }

  /// Check server health
  Future<bool> checkHealth() async {
    final url = Uri.parse('$baseUrl/api/health');
    
    try {
      final response = await http.get(url);
      return response.statusCode == 200;
    } catch (e) {
      return false;
    }
  }
}

/// Chat Response Model
class ChatResponse {
  final String response;
  final DateTime timestamp;

  ChatResponse({
    required this.response,
    required this.timestamp,
  });
}

/// Message Model
class Message {
  final String sender; // "user" or "bot"
  final String text;
  final DateTime timestamp;

  Message({
    required this.sender,
    required this.text,
    required this.timestamp,
  });

  factory Message.fromJson(Map<String, dynamic> json) {
    return Message(
      sender: json['sender'],
      text: json['text'],
      timestamp: DateTime.parse(json['timestamp']),
    );
  }

  bool get isUser => sender == 'user';
  bool get isBot => sender == 'bot';
}

// Example usage in a Flutter Widget:
/*
class ChatScreen extends StatefulWidget {
  @override
  _ChatScreenState createState() => _ChatScreenState();
}

class _ChatScreenState extends State<ChatScreen> {
  final VoBeeClient client = VoBeeClient(baseUrl: 'https://your-vobee-api.com');
  final TextEditingController messageController = TextEditingController();
  List<Message> messages = [];
  bool isLoading = false;

  @override
  void initState() {
    super.initState();
    loadHistory();
  }

  Future<void> loadHistory() async {
    try {
      final history = await client.getHistory();
      setState(() {
        messages = history;
      });
    } catch (e) {
      print('Error loading history: $e');
    }
  }

  Future<void> sendMessage() async {
    final text = messageController.text.trim();
    if (text.isEmpty) return;

    setState(() {
      messages.add(Message(
        sender: 'user',
        text: text,
        timestamp: DateTime.now(),
      ));
      isLoading = true;
    });

    messageController.clear();

    try {
      final response = await client.sendMessage(text);
      setState(() {
        messages.add(Message(
          sender: 'bot',
          text: response.response,
          timestamp: response.timestamp,
        ));
        isLoading = false;
      });
    } catch (e) {
      setState(() {
        isLoading = false;
      });
      print('Error sending message: $e');
    }
  }

  Future<void> clearChat() async {
    try {
      await client.clearHistory();
      setState(() {
        messages = [];
      });
    } catch (e) {
      print('Error clearing chat: $e');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('VoBee Chat'),
        actions: [
          IconButton(
            icon: Icon(Icons.delete),
            onPressed: clearChat,
          ),
        ],
      ),
      body: Column(
        children: [
          Expanded(
            child: ListView.builder(
              itemCount: messages.length,
              itemBuilder: (context, index) {
                final message = messages[index];
                return ListTile(
                  title: Text(
                    message.text,
                    style: TextStyle(
                      color: message.isUser ? Colors.blue : Colors.black,
                    ),
                  ),
                  subtitle: Text(message.sender),
                );
              },
            ),
          ),
          if (isLoading)
            Padding(
              padding: const EdgeInsets.all(8.0),
              child: CircularProgressIndicator(),
            ),
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: messageController,
                    decoration: InputDecoration(
                      hintText: 'Type a message...',
                      border: OutlineInputBorder(),
                    ),
                    onSubmitted: (_) => sendMessage(),
                  ),
                ),
                SizedBox(width: 8),
                IconButton(
                  icon: Icon(Icons.send),
                  onPressed: sendMessage,
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
*/
