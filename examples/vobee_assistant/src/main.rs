mod chatbot;
mod response_patterns;
mod settings;
mod storage;

use chatbot::{MessageSender, VoBeeChatbot};
use gpui::*;
use gpui_component::*;
use gpui_component::button::*;
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component_assets::Assets;
use settings::{AppSettings, get_default_settings_path};
use storage::{ConversationStorage, get_default_db_path};

struct VoBeeApp {
    chatbot: VoBeeChatbot,
    input_state: Entity<InputState>,
    messages: Vec<(MessageSender, String)>,
    _subscriptions: Vec<Subscription>,
    storage: Option<ConversationStorage>,
    current_conversation_id: Option<i64>,
    settings: AppSettings,
    #[allow(dead_code)]
    show_settings: bool,
    theme_mode: ThemeMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ThemeMode {
    Light,
    Dark,
}

impl VoBeeApp {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| InputState::new(window, cx).placeholder("Type your message..."));
        
        // Load settings
        let settings = get_default_settings_path()
            .and_then(|path| AppSettings::load(&path))
            .unwrap_or_default();
        
        // Initialize storage
        let storage = get_default_db_path()
            .and_then(|path| ConversationStorage::new(path))
            .ok();
        
        // Determine theme mode
        let theme_mode = match settings.theme.as_str() {
            "dark" => ThemeMode::Dark,
            "light" => ThemeMode::Light,
            _ => ThemeMode::Light, // Default to light for "system"
        };
        
        let mut messages = Vec::new();
        messages.push((MessageSender::Bot, VoBeeChatbot::welcome_message()));
        
        let _subscriptions = vec![cx.subscribe_in(&input_state, window, {
            let input_state = input_state.clone();
            move |this, _, ev: &InputEvent, window, cx| match ev {
                InputEvent::PressEnter { .. } => {
                    let value = input_state.read(cx).value().trim().to_string();
                    if !value.is_empty() {
                        this.send_message(value, window, cx);
                    }
                }
                _ => {}
            }
        })];
        
        // Create a new conversation
        let current_conversation_id = storage.as_ref()
            .and_then(|s| s.create_conversation("New Conversation").ok());
        
        Self {
            chatbot: VoBeeChatbot::new(),
            input_state,
            messages,
            _subscriptions,
            storage,
            current_conversation_id,
            settings,
            show_settings: false,
            theme_mode,
        }
    }

    fn send_message(&mut self, input: String, window: &mut Window, cx: &mut Context<Self>) {
        // Add user message
        self.messages.push((MessageSender::User, input.clone()));
        
        // Get bot response
        let response = self.chatbot.process_message(input.clone());
        
        // Add bot response
        self.messages.push((MessageSender::Bot, response.clone()));
        
        // Save to storage if enabled
        if self.settings.save_history {
            if let (Some(storage), Some(conv_id)) = (&self.storage, self.current_conversation_id) {
                // Save both user and bot messages
                let user_msg = crate::chatbot::Message {
                    sender: MessageSender::User,
                    text: input,
                    timestamp: chrono::Utc::now(),
                };
                let bot_msg = crate::chatbot::Message {
                    sender: MessageSender::Bot,
                    text: response,
                    timestamp: chrono::Utc::now(),
                };
                let _ = storage.save_message(conv_id, &user_msg);
                let _ = storage.save_message(conv_id, &bot_msg);
            }
        }
        
        // Clear input
        self.input_state.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        
        cx.notify();
    }

    fn clear_history(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.chatbot.clear_conversation_history();
        self.messages.clear();
        self.messages.push((MessageSender::Bot, VoBeeChatbot::welcome_message()));
        
        // Start a new conversation in storage
        if let Some(storage) = &self.storage {
            self.current_conversation_id = storage.create_conversation("New Conversation").ok();
        }
        
        cx.notify();
    }

    fn toggle_theme(&mut self, cx: &mut Context<Self>) {
        self.theme_mode = match self.theme_mode {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };
        cx.notify();
    }

    fn export_conversation(&self, format: ExportFormat) {
        if let (Some(storage), Some(conv_id)) = (&self.storage, self.current_conversation_id) {
            let result = match format {
                ExportFormat::Json => storage.export_conversation_json(conv_id),
                ExportFormat::Markdown => storage.export_conversation_markdown(conv_id),
            };
            
            if let Ok(content) = result {
                let filename = format!(
                    "vobee_conversation_{}.{}",
                    chrono::Utc::now().format("%Y%m%d_%H%M%S"),
                    match format {
                        ExportFormat::Json => "json",
                        ExportFormat::Markdown => "md",
                    }
                );
                
                if let Some(downloads_dir) = dirs::download_dir() {
                    let path = downloads_dir.join(filename);
                    let _ = std::fs::write(&path, content);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ExportFormat {
    Json,
    Markdown,
}

impl Render for VoBeeApp {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (bg_color, _text_color, header_bg, msg_bg) = match self.theme_mode {
            ThemeMode::Light => (
                rgb(0xffffff),
                rgb(0x212121),
                rgb(0xffc107),
                rgb(0xf5f5f5),
            ),
            ThemeMode::Dark => (
                rgb(0x1e1e1e),
                rgb(0xe0e0e0),
                rgb(0xff9800),
                rgb(0x2d2d2d),
            ),
        };

        v_flex()
            .size_full()
            .bg(bg_color)
            .child(
                // Header with controls
                h_flex()
                    .p_4()
                    .gap_3()
                    .items_center()
                    .justify_between()
                    .bg(header_bg)
                    .child(
                        h_flex()
                            .gap_3()
                            .items_center()
                            .child(
                                div()
                                    .text_3xl()
                                    .child("🐝")
                            )
                            .child(
                                v_flex()
                                    .child(
                                        div()
                                            .text_xl()
                                            .font_weight(FontWeight::BOLD)
                                            .child("VoBee AI")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x424242))
                                            .child("Your Friendly Assistant")
                                    )
                            )
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new("theme-toggle")
                                    .label(match self.theme_mode {
                                        ThemeMode::Light => "🌙",
                                        ThemeMode::Dark => "☀️",
                                    })
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.toggle_theme(cx);
                                    }))
                            )
                            .child(
                                Button::new("export-json")
                                    .label("📄 JSON")
                                    .on_click(cx.listener(|this, _, _, _| {
                                        this.export_conversation(ExportFormat::Json);
                                    }))
                            )
                            .child(
                                Button::new("export-md")
                                    .label("📝 MD")
                                    .on_click(cx.listener(|this, _, _, _| {
                                        this.export_conversation(ExportFormat::Markdown);
                                    }))
                            )
                    )
            )
            .child(
                // Messages area
                div()
                    .flex_1()
                    .overflow_hidden()
                    .bg(msg_bg)
                    .child(
                        v_flex()
                            .gap_3()
                            .p_4()
                            .w_full()
                            .children(
                                self.messages.iter().enumerate().map(|(idx, (sender, text))| {
                                    let (msg_bg_color, msg_text_color, align_end) = match sender {
                                        MessageSender::User => (
                                            rgb(0x2196f3),
                                            rgb(0xffffff),
                                            true,
                                        ),
                                        MessageSender::Bot => match self.theme_mode {
                                            ThemeMode::Light => (
                                                rgb(0xe0e0e0),
                                                rgb(0x212121),
                                                false,
                                            ),
                                            ThemeMode::Dark => (
                                                rgb(0x424242),
                                                rgb(0xffffff),
                                                false,
                                            ),
                                        },
                                    };

                                    let msg_content = v_flex()
                                        .gap_1()
                                        .child(text.clone());
                                    
                                    let msg_content = if self.settings.show_timestamps {
                                        msg_content.child(
                                            div()
                                                .text_xs()
                                                .opacity(0.7)
                                                .child(format!("Message #{}", idx + 1))
                                        )
                                    } else {
                                        msg_content
                                    };

                                    let msg = div()
                                        .max_w(px(600.0))
                                        .p_3()
                                        .rounded_lg()
                                        .bg(msg_bg_color)
                                        .text_color(msg_text_color)
                                        .child(msg_content);

                                    if align_end {
                                        h_flex()
                                            .w_full()
                                            .justify_end()
                                            .child(msg)
                                    } else {
                                        h_flex()
                                            .w_full()
                                            .justify_start()
                                            .child(msg)
                                    }
                                })
                            )
                    )
            )
            .child(
                // Input area
                h_flex()
                    .p_4()
                    .gap_2()
                    .bg(bg_color)
                    .border_t_1()
                    .border_color(match self.theme_mode {
                        ThemeMode::Light => rgb(0xbdbdbd),
                        ThemeMode::Dark => rgb(0x424242),
                    })
                    .child(
                        div()
                            .flex_1()
                            .child(Input::new(&self.input_state))
                    )
                    .child(
                        Button::new("send")
                            .primary()
                            .label("Send")
                            .on_click(cx.listener(|this, _, window, cx| {
                                let value = this.input_state.read(cx).value().trim().to_string();
                                if !value.is_empty() {
                                    this.send_message(value, window, cx);
                                }
                            }))
                    )
                    .child(
                        Button::new("clear")
                            .label("Clear")
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.clear_history(window, cx);
                            }))
                    )
            )
    }
}

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        // Initialize GPUI Component
        gpui_component::init(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(800.), px(600.)), cx)),
            titlebar: Some(TitlebarOptions {
                title: Some("VoBee AI Assistant".into()),
                appears_transparent: false,
                traffic_light_position: None,
            }),
            ..Default::default()
        };

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|cx| VoBeeApp::new(window, cx));
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
