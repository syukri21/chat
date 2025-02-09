use minijinja::{context, Environment};
use shaku::{Component, Interface};

const CHAT: &str = include_str!("../../page/chat.html");
const SOMETHING_WENT_WRONG: &str = include_str!("../../page/500.html");
const PROFILE: &str = include_str!("../../page/profile.html");

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

        JinjaTemplateImpl { env }
    }
}

pub trait JinjaTemplate: Interface {
    fn env(&self) -> &Environment<'static>;
    fn something_went_wrong_page(&self) -> String;
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
}
