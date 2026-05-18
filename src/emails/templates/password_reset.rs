use crate::utils::hypertext_elements;
use hypertext::validation::attributes::*;
use hypertext::{Renderable, rsx};

pub fn password_reset_email(reset_link: &str) -> impl Renderable + '_ {
    rsx! {
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8" />
            <meta name="format-detection" content="telephone=no, date=no, address=no, email=no" />
            <title>"Password Reset"</title>
            <style>
                ".body { width: 100%; height: 100%; padding: 40px 0; font-family: sans-serif; line-height: 1.5; color: #333; background-color: #f6f6f6; }"
                ".container { max-width: 600px; margin: 0 auto; background-color: #fff; border-radius: 10px; }"
                ".header { padding: 30px 40px 20px 40px; text-align: center; }"
                ".content { padding: 20px 40px; font-size: 14px; }"
                ".content h1 { margin: 0 0 14px 0; font-size: 26px; font-weight: 700; }"
                ".content p { margin: 0 0 14px 0; }"
                ".btn { display: block; width: fit-content; margin: 30px 0; padding: 12px 28px; font-size: 12px; font-weight: 700; letter-spacing: 0.15em; text-transform: uppercase; text-decoration: none; color: #fff !important; background-color: #1a1a1a; border-radius: 50px; }"
                ".footer { margin-top: 20px; padding: 20px 40px; font-size: 12px; text-align: center; line-height: 2; color: #777; border-top: 1px solid #ddd; }"
                ".footer a, .footer a:visited, .footer a:active { color: #777; text-decoration: underline; }"
                ".footer a:hover { color: hsl(40, 35%, 60%) !important; }"
            </style>
        </head>
        <body class="body">
            <div class="container">
                <div class="header">
                    <span style="font-family: serif; font-size: 36px; font-weight: 400; letter-spacing: 0.075em;">"sartorello"<span style="vertical-align: sub; font-size: 30%;">"®"</span></span>
                </div>
                <div class="content">
                    <h1>"Password Reset"</h1>
                    <p>"We received a request to reset your password. The link to reset your password will expire in 60 minutes. Click the button below to set a new password:"</p>
                    <a href=(format!("https://localhost:3000{reset_link}")) class="btn">"Reset Password"</a>
                    <p>"If you didn't request this, you can safely ignore this email."</p>
                </div>
                <div class="footer">
                    <div>"sartorello.\u{200b}com.\u{200b}br"</div>
                    <div>
                        <a href="https://www.instagram.com/sartorello.moveis" target="_blank">"Instagram"</a>"\u{a0}\u{a0}\u{a0}\u{a0}"
                        <a href="https://pinterest.com/sartorellomoveis" target="_blank">"Pinterest"</a>"\u{a0}\u{a0}\u{a0}\u{a0}"
                        <a href="https://www.facebook.com/sartorello.moveis" target="_blank">"Facebook"</a>"\u{a0}\u{a0}\u{a0}\u{a0}"
                        <a href="https://sartorello.com.br" target="_blank">"Loja Online"</a>
                    </div>
                    <div style="margin-top: 12px;">"SARTORELLO MOVEIS LTDA"</div>
                    <div>"CNPJ 32.929.577/0001-79"</div>
                </div>
            </div>
        </body>
        </html>
    }
}
