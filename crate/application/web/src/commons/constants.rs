// pub const HOME_PAGE: &str = "/";
pub const LOGIN_PAGE: &str = "/login";
pub const SIGNUP_PAGE: &str = "/signup";
pub const CALLBACK_ACTIVATE_PAGE: &str = "/callback/activate/*";
pub const HTMX_LOGIN_PAGE: &str = "/htmx/login";
pub const HTMX_REGISTER_PAGE: &str = "/htmx/register";

pub const PUBLIC_PAGES: [&str; 5] = [
    LOGIN_PAGE,
    SIGNUP_PAGE,
    CALLBACK_ACTIVATE_PAGE,
    HTMX_LOGIN_PAGE,
    HTMX_REGISTER_PAGE,
];

pub const DEBUG_PAGES: [&str; 1] = ["/debug/active-link"];
