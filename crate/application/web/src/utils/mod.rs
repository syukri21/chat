pub fn render_error_alert(message: String) -> String {
    format!(
        r#"<div class="mb-4 p-4 text-red-700 text-sm bg-red-100 rounded-lg" role="alert">
            <p class="text-center">{}</p>
        </div>"#,
        message,
    )
}
