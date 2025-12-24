use std::collections::HashMap;

pub struct ResponsePatterns {
    patterns: HashMap<&'static str, Vec<&'static str>>,
    keywords: HashMap<&'static str, Vec<&'static str>>,
}

impl ResponsePatterns {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        let mut keywords = HashMap::new();

        // Greetings
        patterns.insert("greetings", vec![
            "Hello! 🐝 I'm VoBee, your friendly AI assistant! How can I help you today?",
            "Hi there! 👋 VoBee at your service! What's on your mind?",
            "Hey! 🌟 Great to see you! I'm here to chat and help!",
            "Greetings! 🎉 VoBee here, ready to assist you!",
        ]);
        keywords.insert("greetings", vec!["hello", "hi", "hey", "greetings", "good morning", "good afternoon", "good evening"]);

        // Farewell
        patterns.insert("farewell", vec![
            "Goodbye! 👋 Come back soon!",
            "See you later! 🐝 Have a wonderful day!",
            "Bye! ✨ It was great chatting with you!",
            "Take care! 🌟 Don't be a stranger!",
        ]);
        keywords.insert("farewell", vec!["bye", "goodbye", "see you", "later", "farewell"]);

        // Identity
        patterns.insert("identity", vec![
            "I'm VoBee! 🐝 Your friendly AI assistant created to chat and help you!",
            "VoBee's the name! 🌟 I'm an AI chatbot here to make your day brighter!",
            "I'm VoBee, a creative AI assistant! 🎨 Nice to meet you!",
        ]);
        keywords.insert("identity", vec!["who are you", "what are you", "your name", "who is vobee"]);

        // Capabilities
        patterns.insert("capabilities", vec![
            "I can chat with you, share jokes, provide fun facts, and much more! 🐝",
            "I'm here to have friendly conversations, answer questions, and keep you company! ✨",
            "I can help with various topics, tell jokes, share facts, and just be a friend! 🌟",
        ]);
        keywords.insert("capabilities", vec!["what can you do", "your capabilities", "help me", "help"]);

        // Happy
        patterns.insert("happy", vec![
            "That's wonderful! 🎉 I'm so happy for you!",
            "Yay! 🌟 Your happiness makes me happy too!",
            "That's amazing! ✨ Keep spreading those good vibes!",
        ]);
        keywords.insert("happy", vec!["happy", "excited", "great", "awesome", "wonderful"]);

        // Sad
        patterns.insert("sad", vec![
            "I'm sorry to hear that 😔 Remember, tough times don't last!",
            "Sending virtual hugs! 🤗 Things will get better!",
            "I'm here for you! 💙 Want to talk about it?",
        ]);
        keywords.insert("sad", vec!["sad", "unhappy", "down", "depressed", "upset"]);

        // Jokes
        patterns.insert("jokes", vec![
            "Why don't scientists trust atoms? Because they make up everything! 😄",
            "What do you call a bee that can't make up its mind? A maybe! 🐝",
            "Why did the scarecrow win an award? He was outstanding in his field! 🌾",
            "What do you call a fake noodle? An impasta! 🍝",
        ]);
        keywords.insert("jokes", vec!["joke", "funny", "make me laugh", "tell me a joke"]);

        // Fun Facts
        patterns.insert("funfacts", vec![
            "Did you know? Honey never spoils! Archaeologists have found 3000-year-old honey that's still edible! 🍯",
            "Fun fact: A group of flamingos is called a 'flamboyance'! 🦩",
            "Here's something cool: Octopuses have three hearts! 🐙",
            "Interesting fact: Bananas are berries, but strawberries aren't! 🍌",
        ]);
        keywords.insert("funfacts", vec!["fun fact", "tell me something", "interesting", "fact"]);

        // Thanks
        patterns.insert("thanks", vec![
            "You're welcome! 🐝 Always happy to help!",
            "No problem at all! ✨ That's what I'm here for!",
            "My pleasure! 🌟 Anytime you need me!",
        ]);
        keywords.insert("thanks", vec!["thank", "thanks", "thank you", "thx"]);

        // Time
        patterns.insert("time", vec![
            "Time flies when you're having fun! ⏰",
            "Every moment is precious! Make the most of it! ⏳",
        ]);
        keywords.insert("time", vec!["time", "what time", "clock"]);

        // Bored
        patterns.insert("bored", vec![
            "Boredom is just creativity waiting to happen! 🎨 Try something new!",
            "Let's make things interesting! Want to hear a joke or a fun fact? 🌟",
            "When bored, remember: the best adventures start with 'I wonder what would happen if...' 🚀",
        ]);
        keywords.insert("bored", vec!["bored", "boring", "nothing to do"]);

        // Fallback responses
        patterns.insert("fallback", vec![
            "That's interesting! Tell me more! 🤔",
            "I'm still learning about that! Can you rephrase? 🐝",
            "Hmm, that's a new one for me! I'm logging it to learn! 📝",
            "I'm not quite sure about that, but I'm always learning! 🌱",
            "That's beyond my knowledge right now, but I've noted it for learning! 🧠",
            "I appreciate your patience! I'm still growing my knowledge base! 🌟",
            "That's a curious question! I'm recording it to improve! 📚",
        ]);

        Self { patterns, keywords }
    }

    pub fn get_response(&self, input: &str) -> String {
        let input_lower = input.to_lowercase();
        
        // Check each category for keyword matches
        for (category, kwords) in &self.keywords {
            for keyword in kwords {
                if input_lower.contains(keyword)
                    && let Some(responses) = self.patterns.get(category) {
                        return self.random_response(responses);
                    }
            }
        }

        // Fallback response
        self.random_response(self.patterns.get("fallback").unwrap())
    }

    fn random_response(&self, responses: &[&'static str]) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..responses.len());
        responses[index].to_string()
    }
}
