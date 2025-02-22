use chats::entity::ChatMessages;
use minijinja::{context, Environment};
use shaku::{Component, Interface};
use users::user::UserInfoDisplay;

#[derive(Component)]
#[shaku(interface = JinjaTemplate)]
pub struct JinjaTemplateImpl {
    pub env: Environment<'static>,
}

impl Default for JinjaTemplateImpl {
    fn default() -> Self {
        let mut env = Environment::new();
        // Layout
        const LAYOUT: &str = include_str!("../../page/fragments/layout.html");
        const MODAL_CONFIRM: &str = include_str!("../../page/fragments/modal_confirm.html");
        env.add_template("layout", LAYOUT).unwrap();
        env.add_template("modal_confirm", MODAL_CONFIRM).unwrap();

        // content
        const CHAT: &str = include_str!("../../page/chat.html");
        const SOMETHING_WENT_WRONG: &str = include_str!("../../page/500.html");
        const PROFILE: &str = include_str!("../../page/profile.html");
        env.add_template("chat", CHAT).unwrap();
        env.add_template("something-went-wrong", SOMETHING_WENT_WRONG)
            .unwrap();
        env.add_template("profile", PROFILE).unwrap();

        // htmx
        const USER_INFO: &str = include_str!("../../page/htmx/user_info.html");
        const CHAT_WINDOW: &str = include_str!("../../page/htmx/chat_window.html");
        const CHAT_WINDOW_EMPTY: &str = include_str!("../../page/htmx/chat_window_empty.html");
        const CHAT_HEADER: &str = include_str!("../../page/htmx/chat_header.html");
        env.add_template("htmx-user-info", USER_INFO).unwrap();
        env.add_template("htmx-chat-window", CHAT_WINDOW).unwrap();
        env.add_template("htmx-chat-window-empty", CHAT_WINDOW_EMPTY).unwrap();
        env.add_template("htmx-chat-header", CHAT_HEADER).unwrap();

        JinjaTemplateImpl { env }
    }
}

pub trait JinjaTemplate: Interface {
    fn env(&self) -> &Environment<'static>;
    fn something_went_wrong_page(&self) -> String;
    fn htmx_user_info(&self, user_id: &str, user_info: Box<dyn UserInfoDisplay>) -> String;
    fn htmx_chat_header(&self, user_id: &str, user_info: Box<dyn UserInfoDisplay>) -> String;
    fn htmx_chat_box(&self, chat_messages: &Option<ChatMessages>) -> String;
}

impl JinjaTemplate for JinjaTemplateImpl {
    fn env(&self) -> &Environment<'static> {
        &self.env
    }

    fn something_went_wrong_page(&self) -> String {
        self.env
            .get_template("something-went-wrong")
            .unwrap()
            .render(context! {
                title => "500 - Internal Server Error"
            })
            .unwrap()
    }

    fn htmx_user_info(&self, user_id: &str, user_info: Box<dyn UserInfoDisplay>) -> String {
        self.env
            .get_template("htmx-user-info")
            .unwrap()
            .render(context! {
                profile_picture => user_info.get_profile_picture(),
                username => user_info.get_user_name(),
                fullname => user_info.get_full_name(),
                id => user_id,
            })
            .unwrap()
    }

    fn htmx_chat_header(&self, user_id: &str, user_info: Box<dyn UserInfoDisplay>) -> String {
        return self
            .env
            .get_template("htmx-chat-header")
            .unwrap()
            .render(context! {
                user_id => user_id,
                name =>  user_info.get_full_name(),
                profile_picture => user_info.get_profile_picture(),
                status_online => "online"
            })
            .unwrap();
    }

    fn htmx_chat_box(&self, chat_messages: &Option<ChatMessages>) -> String {
        if chat_messages.is_none() {
            return self
                .env
                .get_template("htmx-chat-window-empty")
                .unwrap()
                .render(context! {})
                .unwrap();
        }
        self.env
            .get_template("htmx-chat-window")
            .unwrap()
            .render(context! {})
            .unwrap()
    }
}
