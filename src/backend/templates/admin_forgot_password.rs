use crate::server::backend_handlers::Type;
use hypertext::validation::attributes::*;
use hypertext::{Renderable, rsx};
use std::collections::HashMap;

use crate::shared::hypertext_elements;

pub fn admin_forgot_password(context: &HashMap<String, Type>) -> impl Renderable {
    let payload = if let Some(Type::Map(map)) = context.get("payload") { Some(map) } else { None };
    let errors = context.get("errors").and_then(|t| if let Type::Map(m) = t { Some(m) } else { None });
    let success = matches!(context.get("forgot_password_success"), Some(Type::Bool(v)) if *v == true);

    rsx! {
        // ===== Page Wrapper Start =====
        <div x-data="{ page: 'forgot-password', 'loaded': true, 'stickyMenu': false, 'sidebarToggle': false, 'scrollTop': false }"
            class="relative p-6 bg-white z-1 sm:p-0">

            <div class="relative flex flex-col justify-center w-full h-screen sm:p-0">
                // Form
                <div class="flex flex-col flex-1 w-full">
                    <div class="w-full max-w-md pt-10 mx-auto">
                        <a href="/admin/login"
                            class="flex items-center gap-2 text-sm text-neutral-500 transition-colors hover:text-neutral-800">
                            <svg class="size-4.5" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="1.5" "stroke"="currentColor">
                                <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M6.75 15.75 3 12m0 0 3.75-3.75M3 12h18"></path>
                            </svg>
                            "Back to login"
                        </a>
                    </div>
                    <div class="flex flex-col justify-center flex-1 w-full max-w-md mx-auto">
                        <div class="mb-5 sm:mb-8">
                            <h1 class="mb-2 font-semibold text-neutral-800 text-3xl sm:text-4xl">
                                "Forgot Password?"
                            </h1>
                            <p class="text-sm text-neutral-500">
                                "Enter your email address and we'll send you a link to reset your password."
                            </p>
                        </div>
                        <form action="/admin/forgot-password" method="POST">
                            <input type="hidden" name="csrf_token" value=(if let Some(Type::Text(v)) = context.get("csrf_token") { v.as_str() } else { "" }) />
                            <div class="space-y-5">
                                // Email
                                <div>
                                    <label class="mb-1.5 block text-sm font-medium text-neutral-700">
                                        "E-mail"
                                        <span class="text-red-600" aria-hidden="true">"*"</span>
                                    </label>
                                    <input
                                        class="h-11 w-full rounded-lg border border-neutral-300 bg-transparent px-4 py-2.5 text-sm text-neutral-800 shadow-xs placeholder:text-neutral-400 focus:border-neutral-300 focus:outline-hidden focus:ring-3 focus:ring-neutral-200/70"
                                        value=(payload.and_then(|p| p.get("email")))
                                        type="email" id="email" name="email"
                                        placeholder="johndoe@example.com" autocomplete="email" />
                                    @if let Some(err) = errors.and_then(|m| m.get("email")) {
                                        <p class="mt-1 text-xs text-red-700">(err)</p>
                                    }
                                </div>
                                // Button
                                <div>
                                    <button type="submit"
                                        class="flex w-full items-center justify-center rounded-lg bg-neutral-900 px-4 py-3 text-sm font-medium text-white transition hover:bg-neutral-900/90 hover:shadow-sm">
                                        "Send Reset Link"
                                    </button>
                                </div>
                            </div>
                        </form>
                        @if success == true {
                            <div
                                class="mt-4 rounded-lg border border-green-600 bg-green-100 p-4 text-sm font-medium text-green-600">
                                "If an account with this email exists, a password reset link will be sent."
                            </div>
                        }
                    </div>
                </div>
            </div>
        </div>
        // ===== Page Wrapper End =====
    }
}
