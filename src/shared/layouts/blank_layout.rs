use hypertext::validation::attributes::*;
use hypertext::{Renderable, rsx};

use crate::server::backend_handlers::Type;
use crate::shared::hypertext_elements;
use std::collections::HashMap;

pub fn blank_layout(title: &str, content: impl Renderable, context: &HashMap<String, Type>) -> impl Renderable {
    let full_title = format!("{title} | Sartorello Móveis");
    rsx! {
        <!DOCTYPE html>
        <html lang="en-US">

        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            // CSRF Token
            <meta name="csrf_token" content=(if let Some(Type::Text(v)) = context.get("csrf_token") { v.as_str() } else { "" })>
            // Title
            <title>(full_title)</title>
            // Favicon
            <link href="/public/favicon.webp" rel="icon" type="image/webp">
            // CSS
            <link href="/public/css/admin.css" rel="stylesheet" type="text/css">
            // AlpineJS
            <script defer src="/public/js/app.js"></script>
        </head>

        <body>
            <main>
                (content)
            </main>
        </body>

        </html>
    }
}
