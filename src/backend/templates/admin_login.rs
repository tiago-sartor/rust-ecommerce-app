use crate::server::backend_handlers::Type;
use hypertext::validation::attributes::*;
use hypertext::{Renderable, rsx};
use std::collections::HashMap;

use crate::shared::hypertext_elements;

pub fn admin_login(context: &HashMap<String, Type>) -> impl Renderable {
    let payload = if let Some(Type::Map(map)) = context.get("payload") { Some(map) } else { None };
    let errors = if let Some(Type::Map(map)) = context.get("errors") { Some(map) } else { None };
    let pwd_reset_success = matches!(context.get("password_reset_success"), Some(Type::Bool(v)) if *v == true);

    rsx! {
        // ===== Page Wrapper Start =====
        <div
            x-data="{ page: 'login', 'loaded': true, 'stickyMenu': false, 'sidebarToggle': false, 'scrollTop': false }"
            class="relative p-6 bg-white z-1 sm:p-0">

            <div class="relative flex flex-col justify-center w-full h-screen sm:p-0">
                // Form
                <div class="flex flex-col flex-1 w-full">
                    <div class="w-full max-w-md pt-10 mx-auto">
                        <a href="/" class="flex items-center gap-2 text-sm text-neutral-500 transition-colors hover:text-neutral-800">
                            <svg class="size-4.5" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="1.5" "stroke"="currentColor">
                                <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M6.75 15.75 3 12m0 0 3.75-3.75M3 12h18"></path>
                            </svg>
                            "Back to store"
                        </a>
                    </div>
                    <div class="flex flex-col justify-center flex-1 w-full max-w-md mx-auto">
                        <div>
                            <div class="mb-5 sm:mb-8">
                                <h1
                                    class="mb-2 font-semibold text-neutral-800 text-3xl sm:text-4xl">
                                    "Login"
                                </h1>
                                <p class="text-sm text-neutral-500">
                                    "Enter your email and password to sign in!"
                                </p>
                            </div>
                            <div>
                                <button
                                    class="flex items-center justify-center gap-3 w-full py-3 text-sm font-medium text-neutral-600 transition bg-neutral-100 rounded-lg px-7 hover:bg-neutral-200 hover:text-neutral-800 hover:shadow-xs">
                                    <svg "width"="20" "height"="20" "viewBox"="0 0 20 20" "fill"="none">
                                        <path "d"="M18.7511 10.1944C18.7511 9.47495 18.6915 8.94995 18.5626 8.40552H10.1797V11.6527H15.1003C15.0011 12.4597 14.4654 13.675 13.2749 14.4916L13.2582 14.6003L15.9087 16.6126L16.0924 16.6305C17.7788 15.1041 18.7511 12.8583 18.7511 10.1944Z" "fill"="#4285F4"></path>
                                        <path "d"="M10.1788 18.75C12.5895 18.75 14.6133 17.9722 16.0915 16.6305L13.274 14.4916C12.5201 15.0068 11.5081 15.3666 10.1788 15.3666C7.81773 15.3666 5.81379 13.8402 5.09944 11.7305L4.99473 11.7392L2.23868 13.8295L2.20264 13.9277C3.67087 16.786 6.68674 18.75 10.1788 18.75Z" "fill"="#34A853"></path>
                                        <path "d"="M5.10014 11.7305C4.91165 11.186 4.80257 10.6027 4.80257 9.99992C4.80257 9.3971 4.91165 8.81379 5.09022 8.26935L5.08523 8.1534L2.29464 6.02954L2.20333 6.0721C1.5982 7.25823 1.25098 8.5902 1.25098 9.99992C1.25098 11.4096 1.5982 12.7415 2.20333 13.9277L5.10014 11.7305Z" "fill"="#FBBC05"></path>
                                        <path "d"="M10.1789 4.63331C11.8554 4.63331 12.9864 5.34303 13.6312 5.93612L16.1511 3.525C14.6035 2.11528 12.5895 1.25 10.1789 1.25C6.68676 1.25 3.67088 3.21387 2.20264 6.07218L5.08953 8.26943C5.81381 6.15972 7.81776 4.63331 10.1789 4.63331Z" "fill"="#EB4335"></path>
                                    </svg>
                                    "Sign in with Google"
                                </button>

                                <div class="relative py-3 sm:py-5">
                                    <div class="absolute inset-0 flex items-center">
                                        <div
                                            class="w-full border-t border-neutral-200 "></div>
                                    </div>
                                    <div class="relative flex justify-center text-sm">
                                        <span
                                            class="p-2 text-neutral-400 bg-white sm:px-5 sm:py-2">"Or"</span>
                                    </div>
                                </div>
                                <form action="" method="POST">
                                    <input type="hidden" name="csrf_token" value=(if let Some(Type::Text(v)) = context.get("csrf_token") { v.as_str() } else { "" }) />
                                    <div class="space-y-5">
                                        // Email
                                        <div>
                                            <label
                                                class="mb-1.5 block text-sm font-medium text-neutral-700">
                                                "E-mail"
                                                <span class="text-red-600" aria-hidden="true">"*"</span>
                                            </label>
                                            <input
                                                class="h-11 w-full rounded-lg border border-neutral-300 bg-transparent px-4 py-2.5 text-sm text-neutral-800 shadow-xs placeholder:text-neutral-400 focus:border-neutral-300 focus:outline-hidden focus:ring-3 focus:ring-neutral-200/70"
                                                type="email"
                                                id="email"
                                                name="email"
                                                value=(payload.and_then(|p| p.get("email")).unwrap_or(&"".to_string()))
                                                placeholder="johndoe@example.com"
                                                autocomplete="email"
                                                required />
                                            @if let Some(err) = errors.and_then(|m| m.get("login")) {
                                                <p class="mt-1 text-xs text-red-700">(err)</p>
                                            }
                                        </div>
                                        // Password
                                        <div>
                                            <label
                                                class="mb-1.5 block text-sm font-medium text-neutral-700">
                                                "Password"
                                                <span class="text-red-600" aria-hidden="true">"*"</span>
                                            </label>
                                            <div x-data="{ showPassword: false }" class="relative">
                                                <input
                                                    x-bind:type="showPassword ? 'text' : 'password'"
                                                    class="h-11 w-full rounded-lg border border-neutral-300 bg-transparent py-2.5 pl-4 pr-11 text-sm text-neutral-800 shadow-xs placeholder:text-neutral-400 focus:border-neutral-300 focus:outline-hidden focus:ring-3 focus:ring-neutral-200/70"
                                                    id="password"
                                                    name="password"
                                                    placeholder="Enter your password"
                                                    autocomplete="current-password"
                                                    required />
                                                <span
                                                    x-on:click="showPassword = !showPassword"
                                                    class="absolute z-30 text-neutral-500 -translate-y-1/2 cursor-pointer right-4 top-1/2">
                                                    <svg
                                                        x-show="!showPassword"
                                                        class="fill-current"
                                                        "width"="20"
                                                        "height"="20"
                                                        "viewBox"="0 0 20 20"
                                                        "fill"="none">
                                                        <path
                                                            "fill-rule"="evenodd"
                                                            "clip-rule"="evenodd"
                                                            "d"="M10.0002 13.8619C7.23361 13.8619 4.86803 12.1372 3.92328 9.70241C4.86804 7.26761 7.23361 5.54297 10.0002 5.54297C12.7667 5.54297 15.1323 7.26762 16.0771 9.70243C15.1323 12.1372 12.7667 13.8619 10.0002 13.8619ZM10.0002 4.04297C6.48191 4.04297 3.49489 6.30917 2.4155 9.4593C2.3615 9.61687 2.3615 9.78794 2.41549 9.94552C3.49488 13.0957 6.48191 15.3619 10.0002 15.3619C13.5184 15.3619 16.5055 13.0957 17.5849 9.94555C17.6389 9.78797 17.6389 9.6169 17.5849 9.45932C16.5055 6.30919 13.5184 4.04297 10.0002 4.04297ZM9.99151 7.84413C8.96527 7.84413 8.13333 8.67606 8.13333 9.70231C8.13333 10.7286 8.96527 11.5605 9.99151 11.5605H10.0064C11.0326 11.5605 11.8646 10.7286 11.8646 9.70231C11.8646 8.67606 11.0326 7.84413 10.0064 7.84413H9.99151Z"
                                                            "fill"="#98A2B3"></path>
                                                    </svg>
                                                    <svg
                                                        x-show="showPassword"
                                                        class="fill-current"
                                                        "width"="20"
                                                        "height"="20"
                                                        "viewBox"="0 0 20 20"
                                                        "fill"="none">
                                                        <path
                                                            "fill-rule"="evenodd"
                                                            "clip-rule"="evenodd"
                                                            "d"="M4.63803 3.57709C4.34513 3.2842 3.87026 3.2842 3.57737 3.57709C3.28447 3.86999 3.28447 4.34486 3.57737 4.63775L4.85323 5.91362C3.74609 6.84199 2.89363 8.06395 2.4155 9.45936C2.3615 9.61694 2.3615 9.78801 2.41549 9.94558C3.49488 13.0957 6.48191 15.3619 10.0002 15.3619C11.255 15.3619 12.4422 15.0737 13.4994 14.5598L15.3625 16.4229C15.6554 16.7158 16.1302 16.7158 16.4231 16.4229C16.716 16.13 16.716 15.6551 16.4231 15.3622L4.63803 3.57709ZM12.3608 13.4212L10.4475 11.5079C10.3061 11.5423 10.1584 11.5606 10.0064 11.5606H9.99151C8.96527 11.5606 8.13333 10.7286 8.13333 9.70237C8.13333 9.5461 8.15262 9.39434 8.18895 9.24933L5.91885 6.97923C5.03505 7.69015 4.34057 8.62704 3.92328 9.70247C4.86803 12.1373 7.23361 13.8619 10.0002 13.8619C10.8326 13.8619 11.6287 13.7058 12.3608 13.4212ZM16.0771 9.70249C15.7843 10.4569 15.3552 11.1432 14.8199 11.7311L15.8813 12.7925C16.6329 11.9813 17.2187 11.0143 17.5849 9.94561C17.6389 9.78803 17.6389 9.61696 17.5849 9.45938C16.5055 6.30925 13.5184 4.04303 10.0002 4.04303C9.13525 4.04303 8.30244 4.17999 7.52218 4.43338L8.75139 5.66259C9.1556 5.58413 9.57311 5.54303 10.0002 5.54303C12.7667 5.54303 15.1323 7.26768 16.0771 9.70249Z"
                                                            "fill"="#98A2B3"></path>
                                                    </svg>
                                                </span>
                                            </div>
                                        </div>
                                        // Checkbox
                                        <div class="flex items-center justify-between">
                                            <div x-data="{ checkboxToggle: false }">
                                                <label for="checkboxLabelOne" class="flex items-center gap-2 text-sm font-normal text-neutral-800 cursor-pointer select-none">
                                                    <div class="relative">
                                                        <input x-on:change="checkboxToggle = !checkboxToggle" type="checkbox" id="checkboxLabelOne" class="sr-only" />
                                                        <div
                                                            x-bind:class="checkboxToggle ? 'border-neutral-800 bg-neutral-800' : 'bg-transparent border-neutral-300'"
                                                            class="flex items-center justify-center size-4 rounded-sm border-[1.25px]">
                                                            <span x-bind:class="checkboxToggle ? '' : 'opacity-0'">
                                                                <svg class="size-3" "viewBox"="0 0 14 14" "fill"="none">
                                                                    <path "d"="M11.6666 3.5L5.24992 9.91667L2.33325 7" "stroke"="white" "stroke-width"="1.94437" "stroke-linecap"="round" "stroke-linejoin"="round"></path>
                                                                </svg>
                                                            </span>
                                                        </div>
                                                    </div>
                                                    "Keep me logged in"
                                                </label>
                                            </div>
                                            <a href="/admin/forgot-password" class="text-sm underline decoration-transparent underline-offset-4 transition-colors text-neutral-800 hover:decoration-neutral-800">
                                                "Forgot password?"
                                            </a>
                                        </div>
                                        // Button
                                        <div>
                                            <button
                                                type="submit"
                                                class="flex w-full items-center justify-center rounded-lg bg-neutral-900 px-4 py-3 text-sm font-medium text-white transition hover:bg-neutral-900/90 hover:shadow-sm">
                                                "Sign In"
                                            </button>
                                        </div>
                                    </div>
                                </form>
                                @if pwd_reset_success == true {
                                    <div class="mt-4 rounded-lg border border-green-600 bg-green-100 p-4 text-sm font-medium text-green-600">
                                        "Your password has been reset. You can now login."
                                    </div>
                                }
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        // ===== Page Wrapper End =====
    }
}
