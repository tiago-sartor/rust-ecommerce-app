use crate::server::backend_handlers::Type;
use crate::shared::hypertext_elements;
use hypertext::validation::attributes::*;
use hypertext::{Renderable, rsx};
use std::collections::HashMap;

pub fn admin_account(context: &HashMap<String, Type>) -> impl Renderable {
    rsx! {
    <div x-data=(format!(r#"{{
        isProfileInfoModal: false,
        profileImage: '{}',
        async uploadProfileImage(event) {{
            const file = event.target.files[0]; 
            if (!file) return;

            const formData = new FormData(this.$refs.profileImageForm);
            
            try {{
                const response = await fetch('/admin/update-profile-image', {{
                    method: 'POST',
                    body: formData,
                }});
                const data = await response.json();
                if (data.success) {{
                    this.profileImage = data.imageUrl;
                    window.dispatchEvent(new CustomEvent('profile-updated', {{ detail: data.imageUrl }}));
                }}
            }} catch (error) {{ console.error('Upload failed:', error); }}
        }},
    }}"#, if let Some(Type::Text(v)) = context.get("profile_image_url") { v.as_str() } else { "" })) >
        <div class="p-4 mx-auto max-w-(--breakpoint-2xl) md:p-6">
            <div class="rounded-2xl border border-gray-200 bg-white p-5 lg:p-6">
                <h3 class="mb-5 text-lg font-semibold text-gray-800 lg:mb-7">
                    "Profile"
                </h3>

                <div class="p-5 mb-6 border border-gray-200 rounded-2xl lg:p-6">
                    <div class="flex flex-col gap-5 xl:flex-row xl:items-center xl:justify-between">
                        <div class="flex flex-col items-center w-full gap-6 xl:flex-row">
                            <form class="relative w-20 h-20 border border-gray-200 rounded-full cursor-pointer"
                                x-on:click="$refs.profileImageInput.click()" x-ref="profileImageForm">
                                <input type="hidden" name="csrf_token" value=(if let Some(Type::Text(v)) = context.get("csrf_token") { v.as_str() } else { "" }) />
                                <input type="file" accept="image/png,image/jpg,image/jpeg,image/webp" class="sr-only"
                                    name="admin_profile_image" id="admin_profile_image" x-on:change="uploadProfileImage($event)"
                                    x-ref="profileImageInput">
                                <template x-if="profileImage">
                                    <img x-bind:src="profileImage" alt="admin profile image"
                                        class="object-cover size-full rounded-full">
                                </template>
                                <template x-if="!profileImage">
                                    <svg class="object-cover size-full rounded-full" "viewBox"="312.81 0 401 401">
                                        <path "fill"="#e4e6e7" "d"="M268.073-44.735h490.423v490.423H268.073z"></path>
                                        <path "fill"="#aeb4b7"
                                            "d"="M513.81 267.142c-103.361 0-187.754 58.93-192.475 132.842h384.988c-4.733-73.918-89.157-132.842-192.512-132.842m96.605-109.116c0 57.17-42.935 103.516-95.896 103.516s-95.895-46.346-95.895-103.516S461.559 54.51 514.52 54.51c52.968 0 95.896 46.352 95.896 103.515z">
                                        </path>
                                    </svg>
                                </template>
                                <span
                                    class="absolute p-0.75 size-5 right-0 bottom-0 z-10 inline-flex items-center justify-center rounded-full bg-neutral-800 text-white ring-2 ring-white select-none">
                                    <svg "fill"="currentColor" "width"="18" "height"="18" "viewBox"="0 0 18 18">
                                        <path "fill-rule"="evenodd" "clip-rule"="evenodd"
                                            "d"="M15.0911 2.78206C14.2125 1.90338 12.7878 1.90338 11.9092 2.78206L4.57524 10.116C4.26682 10.4244 4.0547 10.8158 3.96468 11.2426L3.31231 14.3352C3.25997 14.5833 3.33653 14.841 3.51583 15.0203C3.69512 15.1996 3.95286 15.2761 4.20096 15.2238L7.29355 14.5714C7.72031 14.4814 8.11172 14.2693 8.42013 13.9609L15.7541 6.62695C16.6327 5.74827 16.6327 4.32365 15.7541 3.44497L15.0911 2.78206ZM12.9698 3.84272C13.2627 3.54982 13.7376 3.54982 14.0305 3.84272L14.6934 4.50563C14.9863 4.79852 14.9863 5.2734 14.6934 5.56629L14.044 6.21573L12.3204 4.49215L12.9698 3.84272ZM11.2597 5.55281L5.6359 11.1766C5.53309 11.2794 5.46238 11.4099 5.43238 11.5522L5.01758 13.5185L6.98394 13.1037C7.1262 13.0737 7.25666 13.003 7.35947 12.9002L12.9833 7.27639L11.2597 5.55281Z">
                                        </path>
                                    </svg>
                                </span>
                            </form>
                            <div class="order-3 xl:order-2">
                                <h4 class="mb-2 text-lg font-semibold text-center text-gray-800 xl:text-left">
                                    (if let Some(Type::Text(v)) = context.get("first_name") { v.as_str() } else { "" })
                                    " "
                                    (if let Some(Type::Text(v)) = context.get("last_name") { v.as_str() } else { "" })
                                </h4>
                                <div class="flex flex-col items-center gap-1 text-center xl:flex-row xl:gap-3 xl:text-left">
                                    <p class="text-sm text-gray-500 capitalize">
                                        (format!("{:?}", (if let Some(Type::Text(v)) = context.get("first_name") { v.as_str() } else { "" })))
                                    </p>
                                </div>
                            </div>
                        </div>

                        <button x-on:click="isProfileInfoModal = true" class="flex w-full items-center justify-center gap-2 rounded-full border border-gray-300 bg-white px-4 py-3 text-sm font-medium text-gray-700 shadow-theme-xs hover:bg-gray-50 hover:text-gray-800 lg:inline-flex lg:w-auto">
                            <svg "fill"="currentColor" "width"="18" "height"="18" "viewBox"="0 0 18 18">
                                <path "fill-rule"="evenodd" "clip-rule"="evenodd"
                                    "d"="M15.0911 2.78206C14.2125 1.90338 12.7878 1.90338 11.9092 2.78206L4.57524 10.116C4.26682 10.4244 4.0547 10.8158 3.96468 11.2426L3.31231 14.3352C3.25997 14.5833 3.33653 14.841 3.51583 15.0203C3.69512 15.1996 3.95286 15.2761 4.20096 15.2238L7.29355 14.5714C7.72031 14.4814 8.11172 14.2693 8.42013 13.9609L15.7541 6.62695C16.6327 5.74827 16.6327 4.32365 15.7541 3.44497L15.0911 2.78206ZM12.9698 3.84272C13.2627 3.54982 13.7376 3.54982 14.0305 3.84272L14.6934 4.50563C14.9863 4.79852 14.9863 5.2734 14.6934 5.56629L14.044 6.21573L12.3204 4.49215L12.9698 3.84272ZM11.2597 5.55281L5.6359 11.1766C5.53309 11.2794 5.46238 11.4099 5.43238 11.5522L5.01758 13.5185L6.98394 13.1037C7.1262 13.0737 7.25666 13.003 7.35947 12.9002L12.9833 7.27639L11.2597 5.55281Z">
                                </path>
                            </svg>
                            "Edit"
                        </button>
                    </div>
                </div>

                <div class="p-5 mb-6 border border-gray-200 rounded-2xl lg:p-6">
                    <div class="flex flex-col gap-6 lg:flex-row lg:items-start lg:justify-between">
                        <div>
                            <h4 class="text-lg font-semibold text-gray-800 mb-4 lg:mb-6">
                                "Account Information"
                            </h4>

                            <div class="grid grid-cols-1 gap-4 lg:grid-cols-2 lg:gap-7 2xl:gap-x-32">
                                <div>
                                    <p class="mb-2 text-xs leading-normal text-gray-500">
                                        "First Name"
                                    </p>
                                    <p class="text-sm font-medium text-gray-800">
                                        (if let Some(Type::Text(v)) = context.get("first_name") { v.as_str() } else { "" })
                                    </p>
                                </div>

                                <div>
                                    <p class="mb-2 text-xs leading-normal text-gray-500">
                                        "Last Name"
                                    </p>
                                    <p class="text-sm font-medium text-gray-800">
                                        (if let Some(Type::Text(v)) = context.get("last_name") { v.as_str() } else { "" })
                                    </p>
                                </div>

                                <div>
                                    <p class="mb-2 text-xs leading-normal text-gray-500">
                                        "E-mail"
                                    </p>
                                    <p class="text-sm font-medium text-gray-800">
                                        (if let Some(Type::Text(v)) = context.get("email") { v.as_str() } else { "" })
                                    </p>
                                </div>

                                <div>
                                    <p class="mb-2 text-xs leading-normal text-gray-500">
                                        "Phone"
                                    </p>
                                    <p class="text-sm font-medium text-gray-800">
                                        (if let Some(Type::Text(v)) = context.get("phone") { v.as_str() } else { "" })
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="p-5 border border-gray-200 rounded-2xl lg:p-6">
                    <div class="flex flex-col gap-6 lg:flex-row lg:items-start lg:justify-between">
                        <div>
                            <h4 class="text-lg font-semibold text-gray-800 mb-4 lg:mb-6">
                                "Security & Metadata"
                            </h4>

                            <div class="grid grid-cols-1 gap-4 lg:grid-cols-2 lg:gap-7 2xl:gap-x-32">
                                <div>
                                    <p class="mb-2 text-xs leading-normal text-gray-500">
                                        "Last Login"
                                    </p>
                                    <p class="text-sm font-medium text-gray-800">
                                        (if let Some(Type::Text(v)) = context.get("last_login") { v.as_str() } else { "Never" })
                                    </p>
                                </div>
                                <div>
                                    <p class="mb-2 text-xs leading-normal text-gray-500">
                                        "Registered Since"
                                    </p>
                                    <p class="text-sm font-medium text-gray-800">
                                        (if let Some(Type::Text(v)) = context.get("created_at") { v.as_str() } else { "" })
                                    </p>
                                </div>
                            </div>
                        </div>

                        <a href="/admin/forgot-password" class="flex w-full items-center justify-center gap-2 rounded-full border border-gray-300 bg-white px-4 py-3 text-sm font-medium text-gray-700 shadow-theme-xs hover:bg-gray-50 hover:text-gray-800 lg:inline-flex lg:w-auto">
                            "Update Password"
                        </a>
                    </div>
                </div>
            </div>
        </div>
    </div>
    }
}
