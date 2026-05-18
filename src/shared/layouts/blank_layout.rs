use crate::server::backend_handlers::Context;
use crate::utils::hypertext_elements;
use hypertext::validation::attributes::*;
use hypertext::{Renderable, rsx};

pub fn blank_layout<P, D>(title: &str, content: impl Renderable, ctx: &Context<P, D>) -> impl Renderable {
    let full_title = format!("{title} | Rust Ecommerce App");

    rsx! {
        <!DOCTYPE html>
        <html lang="en-US">

        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            // CSRF Token
            <meta name="csrf_token" content=(ctx.csrf_token.0)>
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
