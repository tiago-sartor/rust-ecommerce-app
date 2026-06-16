use crate::server::handlers::backend::payloads::ResetPasswordPayload;
use crate::utils::{Context, hypertext_elements};
use hypertext::prelude::*;

pub fn admin_reset_password_template(ctx: &Context<ResetPasswordPayload, ()>) -> impl Renderable {
    rsx! {
        // ===== Page Wrapper Start =====
        <div
            x-data="{ page: 'reset-password', 'loaded': true, 'stickyMenu': false, 'sidebarToggle': false, 'scrollTop': false }"
            class="relative p-6 bg-white z-1 sm:p-0">

            <div class="relative flex flex-col justify-center w-full h-screen sm:p-0">
                // Form
                <div class="flex flex-col flex-1 w-full">
                    <div class="w-full max-w-md pt-10 mx-auto">
                        <a href="/admin/login" class="flex items-center gap-2 text-sm text-neutral-500 transition-colors hover:text-neutral-800">
                            <svg class="size-4.5" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="1.5" "stroke"="currentColor">
                                <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M6.75 15.75 3 12m0 0 3.75-3.75M3 12h18"></path>
                            </svg>
                            "Back to login"
                        </a>
                    </div>
                    <div class="flex flex-col justify-center flex-1 w-full max-w-md mx-auto">
                        <div class="mb-5 sm:mb-8">
                            <h1 class="mb-2 font-semibold text-neutral-800 text-3xl sm:text-4xl">
                                "Reset Password"
                            </h1>
                            <p class="text-sm text-neutral-500">
                                "Please enter your new password below."
                            </p>
                        </div>
                        <form action="" method="POST">
                            <input type="hidden" name="csrf_token" value=(ctx.csrf_token.0) />
                            <div class="space-y-5">
                                // New Password
                                <div>
                                    <label for="password" class="mb-1.5 block text-sm font-medium text-neutral-700">
                                        "New Password"
                                        <span class="text-red-600" aria-hidden="true">"*"</span>
                                    </label>
                                    <div x-data="{ showPassword: false }" class="relative">
                                        <input
                                            x-bind:type="showPassword ? 'text' : 'password'"
                                            class="h-11 w-full rounded-lg border border-neutral-300 bg-transparent px-4 py-2.5 text-sm text-neutral-800 shadow-xs placeholder:text-neutral-400 focus:border-neutral-300 focus:outline-hidden focus:ring-3 focus:ring-neutral-200/70"
                                            id="password"
                                            name="password"
                                            value=(ctx.payload.password)
                                            placeholder="Enter new password"
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
                                    @if let Some(err) = ctx.errors.get("password") {
                                        <p class="mt-1 text-xs text-red-700">(err)</p>
                                    }
                                </div>
                                // Confirm Password
                                <div>
                                    <label for="confirm_password" class="mb-1.5 block text-sm font-medium text-neutral-700">
                                        "Confirm New Password"
                                        <span class="text-red-600" aria-hidden="true">"*"</span>
                                    </label>
                                    <div x-data="{ showPassword: false }" class="relative">
                                        <input
                                            x-bind:type="showPassword ? 'text' : 'password'"
                                            class="h-11 w-full rounded-lg border border-neutral-300 bg-transparent px-4 py-2.5 text-sm text-neutral-800 shadow-xs placeholder:text-neutral-400 focus:border-neutral-300 focus:outline-hidden focus:ring-3 focus:ring-neutral-200/70"
                                            id="confirm_password"
                                            name="confirm_password"
                                            value=(ctx.payload.confirm_password)
                                            placeholder="Confirm new password"
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
                                    @if let Some(err) = ctx.errors.get("confirm_password") {
                                        <p class="mt-1 text-xs text-red-700">(err)</p>
                                    }
                                </div>
                                // Button
                                <div>
                                    <button
                                        type="submit"
                                        class="flex w-full items-center justify-center rounded-lg bg-neutral-900 px-4 py-3 text-sm font-medium text-white transition hover:bg-neutral-900/90 hover:shadow-sm">
                                        "Reset Password"
                                    </button>
                                </div>
                            </div>
                        </form>
                        @if let Some(err) = ctx.errors.get("internal_error") {
                            <div class="mt-4 rounded-lg border border-red-600 bg-red-50 p-4 text-sm font-medium text-red-600">
                                (err)
                            </div>
                        }
                    </div>
                </div>
            </div>
        </div>
        // ===== Page Wrapper End =====
    }
}
