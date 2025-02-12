use minijinja::{context, Environment};
use shaku::{Component, Interface};

const CHAT: &str = include_str!("../../page/chat.html");
const SOMETHING_WENT_WRONG: &str = include_str!("../../page/500.html");
const PROFILE: &str = include_str!("../../page/profile.html");
const USER_INFO: &str = include_str!("../../page/htmx/user_info.html");
const CHAT_WINDOW: &str = include_str!("../../page/htmx/chat_window.html");

#[derive(Component)]
#[shaku(interface = JinjaTemplate)]
pub struct JinjaTemplateImpl {
    pub env: Environment<'static>,
}

impl Default for JinjaTemplateImpl {
    fn default() -> Self {
        let mut env = Environment::new();
        env.add_template("layout", include_str!("../../page/fragments/layout.html"))
            .unwrap();

        env.add_template(
            "modal_confirm",
            include_str!("../../page/fragments/modal_confirm.html"),
        )
        .unwrap();

        // content
        env.add_template("chat", CHAT).unwrap();
        env.add_template("something-went-wrong", SOMETHING_WENT_WRONG)
            .unwrap();
        env.add_template("profile", PROFILE).unwrap();
        env.add_template("htmx-user-info", USER_INFO).unwrap();
        env.add_template("htmx-chat-window", CHAT_WINDOW).unwrap();

        JinjaTemplateImpl { env }
    }
}

pub trait JinjaTemplate: Interface {
    fn env(&self) -> &Environment<'static>;
    fn something_went_wrong_page(&self) -> String;
    fn htmx_user_info(&self, user_info: &users::user::UserInfo) -> String;
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

    fn htmx_user_info(&self, user_info: &users::user::UserInfo) -> String {
        let profile_picture = format!("https://ui-avatars.com/api/?name={}&background=random&rounded=true", user_info.username);
        self.env
            .get_template("htmx-user-info")
            .unwrap()
            .render(context! {
                profile_picture => profile_picture,
                username => user_info.username,
                id => user_info.id.to_string(),
            })
            .unwrap()
    }
}
