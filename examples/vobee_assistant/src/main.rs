mod chatbot;
mod response_patterns;

use chatbot::{MessageSender, VoBeeChatbot};
use gpui::*;
use gpui_component::*;
use gpui_component::button::*;
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component_assets::Assets;

struct VoBeeApp {
    chatbot: VoBeeChatbot,
    input_state: Entity<InputState>,
    messages: Vec<(MessageSender, String)>,
    _subscriptions: Vec<Subscription>,
}

impl VoBeeApp {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| InputState::new(window, cx).placeholder("Type your message..."));
        
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
        
        Self {
            chatbot: VoBeeChatbot::new(),
            input_state,
            messages,
            _subscriptions,
        }
    }

    fn send_message(&mut self, input: String, _window: &mut Window, cx: &mut Context<Self>) {
        // Add user message
        self.messages.push((MessageSender::User, input.clone()));
        
        // Get bot response
        let response = self.chatbot.process_message(input);
        
        // Add bot response
        self.messages.push((MessageSender::Bot, response));
        
        // Clear input
        self.input_state.update(cx, |state, cx| {
            state.set_value("", _window, cx);
        });
        
        cx.notify();
    }

    fn clear_history(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.chatbot.clear_conversation_history();
        self.messages.clear();
        self.messages.push((MessageSender::Bot, VoBeeChatbot::welcome_message()));
        cx.notify();
    }
}

impl Render for VoBeeApp {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(gpui::white())
            .child(
                // Header
                h_flex()
                    .p_4()
                    .gap_3()
                    .items_center()
                    .bg(rgb(0xffc107))
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
                // Messages area
                div()
                    .flex_1()
                    .overflow_hidden()
                    .bg(rgb(0xf5f5f5))
                    .child(
                        v_flex()
                            .gap_3()
                            .p_4()
                            .w_full()
                            .children(
                                self.messages.iter().map(|(sender, text)| {
                                    let (bg_color, text_color, align_end) = match sender {
                                        MessageSender::User => (
                                            rgb(0x2196f3),
                                            rgb(0xffffff),
                                            true,
                                        ),
                                        MessageSender::Bot => (
                                            rgb(0xe0e0e0),
                                            rgb(0x212121),
                                            false,
                                        ),
                                    };

                                    let msg = div()
                                        .max_w(px(600.0))
                                        .p_3()
                                        .rounded_lg()
                                        .bg(bg_color)
                                        .text_color(text_color)
                                        .child(text.clone());

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
                    .bg(gpui::white())
                    .border_t_1()
                    .border_color(rgb(0xbdbdbd))
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
