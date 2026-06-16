use crate::utils::{Context, hypertext_elements};
use hypertext::prelude::*;

pub fn blank_layout<P, D>(title: &str, content: impl Renderable, ctx: &Context<P, D>, scripts: Option<Vec<&str>>) -> impl Renderable {
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
            <link href="/assets/favicon.webp" rel="icon" type="image/webp">
            // CSS
            <link href="/assets/css/admin.css" rel="stylesheet" type="text/css">
            // AlpineJS
            <script defer src="/assets/js/app.js"></script>

            // Render optional scripts if they exist
            @if let Some(s) = &scripts {
                @for script in s {
                    <script defer src=(format!("/assets/js/{script}.js"))></script>
                }
            }
        </head>

        <body>
            <main>
                (content)
            </main>
        </body>

        </html>
    }
}
