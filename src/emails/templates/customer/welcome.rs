use hypertext::{Renderable, rsx};
use hypertext::validation::attributes::*;
use crate::utils::hypertext_elements;

pub fn welcome_email(user_name: &str) -> impl Renderable + '_ {
    rsx! {
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="utf-8" />
            <title>"Welcome"</title>
            <style>
                "body { font-family: sans-serif; line-height: 1.5; color: #333; }"
                ".container { max-width: 600px; margin: 0 auto; padding: 20px; }"
                ".header { background-color: #f8f9fa; padding: 20px; text-align: center; }"
                ".content { padding: 20px 0; }"
                ".footer { text-align: center; font-size: 12px; color: #777; margin-top: 20px; }"
            </style>
        </head>
        <body>
            <div class="container">
                <div class="header">
                    <h1>"Welcome to our Store!"</h1>
                </div>
                <div class="content">
                    <p>"Hi " (user_name) ","</p>
                    <p>"Thanks for joining us. We're excited to have you on board."</p>
                    <p>"You can now explore our store and enjoy the best deals."</p>
                </div>
                <div class="footer">
                    <p>"© 2026 Rust Ecommerce App. All rights reserved."</p>
                </div>
            </div>
        </body>
        </html>
    }
}
