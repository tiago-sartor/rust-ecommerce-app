use crate::utils::{Context, hypertext_elements};
use hypertext::prelude::*;

pub fn admin_layout<P, D>(title: &str, content: impl Renderable, context: &Context<P, D>, scripts: Option<Vec<&str>>) -> impl Renderable {
    let full_title = format!("{} | Admin Dashboard", title);
    let admin = context.admin.as_ref().expect("An admin user must be logged in.");

    rsx! {
        <!DOCTYPE html>
        <html lang="en-US">

        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            // CSRF
            <meta name="csrf-token" content=(context.csrf_token.0)>
            // Title
            <title>(full_title)</title>
            // Favicon
            <link href="/assets/favicon.webp" rel="icon" type="image/webp">
            // CSS
            <link href="/assets/css/admin.css" rel="stylesheet" type="text/css">

            // Render optional scripts if they exist
            @if let Some(s) = &scripts {
                @for script in s {
                    <script defer src=(format!("/assets/js/{script}.js"))></script>
                }
            }

            // AlpineJS
            <script defer src="/assets/js/app.js"></script>
        </head>

        <body x-data="{ page: 'dashboard', 'loading': true, 'stickyMenu': false, 'sidebarToggle': false, 'scrollTop': false }">

            // ===== Loading Indicator =====
            <div x-show="loading" class="absolute inset-0 z-999999 flex items-center justify-center bg-white" x-init="window.addEventListener('DOMContentLoaded', () => {setTimeout(() => loading = false, 500)})">
                <div class="size-16 animate-spin border-4 border-indigo-500 border-t-transparent rounded-full"></div>
            </div>

            // ===== Page Wrapper Start =====
            <div class="flex h-screen overflow-hidden">
                // ===== Sidebar Start =====
                <aside
                    x-bind:class="sidebarToggle ? 'translate-x-0 xl:w-[90px]' : '-translate-x-full'"
                    class="fixed top-0 left-0 z-9999 flex h-screen w-[290px] flex-col overflow-y-hidden border-r border-neutral-200 bg-white p-4 transition-all duration-300 xl:static xl:translate-x-0">
                    // SIDEBAR HEADER
                    <div x-bind:class="sidebarToggle ? 'justify-center' : 'justify-between'"
                        class="sidebar-header flex items-center gap-2 pt-4 pb-6">
                        <a href="index.html">
                            <img x-bind:class="sidebarToggle ? 'hidden' : ''" class="logo" src="src/images/logo/logo.svg" alt="Logo" />
                            <img x-bind:class="sidebarToggle ? 'xl:block' : 'hidden'" class="logo-icon" src="src/images/logo/logo-icon.svg" alt="Logo" />
                        </a>
                    </div>
                    // SIDEBAR HEADER

                    // Sidebar Menu
                    <nav x-data="{selected: $persist('Dashboard')}" class="flex flex-col flex-1 gap-6 overflow-y-auto duration-300 ease-linear hide-scrollbar">
                        // Menu Group
                        <div class="menu-group flex-1">
                            <ul class="flex flex-col gap-3">
                                // Menu Item Dashboard
                                <li>
                                    <a href="/admin/dashboard" x-on:click="selected = (selected === 'Dashboard' ? '' : 'Dashboard')"
                                        class="menu-item group"
                                        x-bind:class="(selected === 'Dashboard') && (page === 'dashboard') ? 'menu-item-active' : 'menu-item-inactive'">
                                        <svg class="size-6" x-bind:class="(selected === 'Dashboard') && (page === 'dashboard') ? 'menu-item-icon-active' : 'menu-item-icon-inactive'" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="1.25" "stroke"="currentColor">
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round"
                                                "d"="M3.75 6A2.25 2.25 0 0 1 6 3.75h2.25A2.25 2.25 0 0 1 10.5 6v2.25a2.25 2.25 0 0 1-2.25 2.25H6a2.25 2.25 0 0 1-2.25-2.25V6ZM3.75 15.75A2.25 2.25 0 0 1 6 13.5h2.25a2.25 2.25 0 0 1 2.25 2.25V18a2.25 2.25 0 0 1-2.25 2.25H6A2.25 2.25 0 0 1 3.75 18v-2.25ZM13.5 6a2.25 2.25 0 0 1 2.25-2.25H18A2.25 2.25 0 0 1 20.25 6v2.25A2.25 2.25 0 0 1 18 10.5h-2.25a2.25 2.25 0 0 1-2.25-2.25V6ZM13.5 15.75a2.25 2.25 0 0 1 2.25-2.25H18a2.25 2.25 0 0 1 2.25 2.25V18A2.25 2.25 0 0 1 18 20.25h-2.25A2.25 2.25 0 0 1 13.5 18v-2.25Z"></path>
                                        </svg>
                                        <span class="menu-item-text" x-bind:class="sidebarToggle ? 'lg:hidden' : ''">
                                            "Dashboard"
                                        </span>
                                    </a>
                                </li>
                                // Menu Item Dashboard

                                // Menu Item Orders
                                <li>
                                    <a
                                        href="#" x-on:click.prevent="selected = (selected === 'Orders' ? '':'Orders')"
                                        class="menu-item group"
                                        x-bind:class="(selected === 'Orders') || (page === 'list-orders' || page === 'create-order') ? 'menu-item-active' : 'menu-item-inactive'">
                                        <svg
                                            class="size-6"
                                            x-bind:class="(selected === 'Orders') || (page === 'list-orders' || page === 'create-order') ?  'menu-item-icon-active'  :'menu-item-icon-inactive'"
                                            "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="1.5" "stroke"="currentColor">
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M2.25 3h1.386c.51 0 .955.343 1.087.835l.383 1.437M7.5 14.25a3 3 0 0 0-3 3h15.75m-12.75-3h11.218c1.121-2.3 2.1-4.684 2.924-7.138a60.114 60.114 0 0 0-16.536-1.84M7.5 14.25 5.106 5.272M6 20.25a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0Zm12.75 0a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0Z"></path>
                                        </svg>
                                        <span class="menu-item-text" x-bind:class="sidebarToggle ? 'lg:hidden' : ''">
                                            "Orders"
                                        </span>
                                        <svg class="menu-item-arrow size-4" x-bind:class="[(selected === 'Orders') ? 'menu-item-arrow-active' : 'menu-item-arrow-inactive', sidebarToggle ? 'lg:hidden' : '' ]" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="2" "stroke"="currentColor">
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="m19.5 8.25-7.5 7.5-7.5-7.5"></path>
                                        </svg>
                                    </a>

                                    // Dropdown Menu Start
                                    <div
                                        class="menu-dropdown"
                                        x-bind:class="(selected === 'Orders') ? 'h-fit opacity-100' :'h-0 opacity-0'">
                                        <ul
                                            class="flex flex-col gap-1 mt-2 pl-9"
                                            x-bind:class="sidebarToggle ? 'lg:hidden' : 'flex'">
                                            <li>
                                                <a href="#"
                                                    class="menu-dropdown-item group"
                                                    x-bind:class="(page === 'list-orders') ? 'menu-dropdown-item-active' : 'menu-dropdown-item-inactive'">
                                                    "View orders"
                                                </a>
                                            </li>
                                            <li>
                                                <a href="#"
                                                    class="menu-dropdown-item group"
                                                    x-bind:class="(page === 'create-order') ? 'menu-dropdown-item-active' : 'menu-dropdown-item-inactive'">
                                                    "Add new order"
                                                </a>
                                            </li>
                                        </ul>
                                    </div>
                                    // Dropdown Menu End
                                </li>
                                // Menu Item Orders

                                // Menu Item Customers
                                <li>
                                    <a
                                        href="#"
                                        x-on:click="selected = (selected === 'Customers' ? '' : 'Customers')"
                                        class="menu-item group"
                                        x-bind:class="(selected === 'Customers') || (page === 'list-customers' || page === 'create-customer') ? 'menu-item-active' : 'menu-item-inactive'">
                                        <svg
                                            class="size-6"
                                            x-bind:class="(selected === 'Customers') || (page === 'list-customers' || page === 'create-customer') ?  'menu-item-icon-active'  :'menu-item-icon-inactive'"
                                            "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="1.25" "stroke"="currentColor">
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M18 18.72a9.094 9.094 0 0 0 3.741-.479 3 3 0 0 0-4.682-2.72m.94 3.198.001.031c0 .225-.012.447-.037.666A11.944 11.944 0 0 1 12 21c-2.17 0-4.207-.576-5.963-1.584A6.062 6.062 0 0 1 6 18.719m12 0a5.971 5.971 0 0 0-.941-3.197m0 0A5.995 5.995 0 0 0 12 12.75a5.995 5.995 0 0 0-5.058 2.772m0 0a3 3 0 0 0-4.681 2.72 8.986 8.986 0 0 0 3.74.477m.94-3.197a5.971 5.971 0 0 0-.94 3.197M15 6.75a3 3 0 1 1-6 0 3 3 0 0 1 6 0Zm6 3a2.25 2.25 0 1 1-4.5 0 2.25 2.25 0 0 1 4.5 0Zm-13.5 0a2.25 2.25 0 1 1-4.5 0 2.25 2.25 0 0 1 4.5 0Z"></path>
                                        </svg>
                                        <span class="menu-item-text" x-bind:class="sidebarToggle ? 'lg:hidden' : ''">
                                            "Customers"
                                        </span>
                                        <svg class="menu-item-arrow size-4" x-bind:class="(selected === 'Customers') ? 'menu-item-arrow-active' : 'menu-item-arrow-inactive', sidebarToggle ? 'lg:hidden' : '' " "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="2" "stroke"="currentColor">
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="m19.5 8.25-7.5 7.5-7.5-7.5"></path>
                                        </svg>
                                    </a>

                                    // Dropdown Menu Start
                                    <div
                                        class="menu-dropdown"
                                        x-bind:class="(selected === 'Customers') ? 'h-fit opacity-100' :'h-0 opacity-0'">
                                        <ul
                                            class="flex flex-col gap-1 mt-2 pl-9"
                                            x-bind:class="sidebarToggle ? 'lg:hidden' : 'flex'">
                                            <li>
                                                <a href="/admin/customers"
                                                    class="menu-dropdown-item group"
                                                    x-bind:class="(page === 'customers') ? 'menu-dropdown-item-active' : 'menu-dropdown-item-inactive'">
                                                    "View customers"
                                                </a>
                                            </li>
                                            <li>
                                                <a href="/admin/add-customer"
                                                    class="menu-dropdown-item group"
                                                    x-bind:class="(page === 'add-customer') ? 'menu-dropdown-item-active' : 'menu-dropdown-item-inactive'">
                                                    "Add new customer"
                                                </a>
                                            </li>
                                        </ul>
                                    </div>
                                    // Dropdown Menu End
                                </li>
                                // Menu Item Customers

                                // Menu Item Products
                                <li>
                                    <a
                                        href="#"
                                        x-on:click="selected = (selected === 'Products' ? '' : 'Products')"
                                        class="menu-item group"
                                        x-bind:class="(selected === 'Products') || (page === 'list-products' || page === 'create-product') ? 'menu-item-active' : 'menu-item-inactive'">
                                        <svg
                                            class="size-6"
                                            x-bind:class="(selected === 'Products') || (page === 'list-products' || page === 'create-product') ?  'menu-item-icon-active'  :'menu-item-icon-inactive'"
                                            "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="1.25" "stroke"="currentColor">
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="m21 7.5-9-5.25L3 7.5m18 0-9 5.25m9-5.25v9l-9 5.25M3 7.5l9 5.25M3 7.5v9l9 5.25m0-9v9"></path>
                                        </svg>
                                        <span class="menu-item-text" x-bind:class="sidebarToggle ? 'lg:hidden' : ''">
                                            "Products"
                                        </span>
                                        <svg class="menu-item-arrow size-4" x-bind:class="(selected === 'Products') ? 'menu-item-arrow-active' : 'menu-item-arrow-inactive', sidebarToggle ? 'lg:hidden' : '' " "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="2" "stroke"="currentColor">
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="m19.5 8.25-7.5 7.5-7.5-7.5"></path>
                                        </svg>
                                    </a>

                                    // Dropdown Menu Start
                                    <div
                                        class="menu-dropdown"
                                        x-bind:class="(selected === 'Products') ? 'h-fit opacity-100' :'h-0 opacity-0'">
                                        <ul
                                            class="flex flex-col gap-1 mt-2 pl-9"
                                            x-bind:class="sidebarToggle ? 'lg:hidden' : 'flex'">
                                            <li>
                                                <a href="#"
                                                    class="menu-dropdown-item group"
                                                    x-bind:class="(page === 'list-products') ? 'menu-dropdown-item-active' : 'menu-dropdown-item-inactive'">
                                                    "View products"
                                                </a>
                                            </li>
                                            <li>
                                                <a href="#"
                                                    class="menu-dropdown-item group"
                                                    x-bind:class="(page === 'create-product') ? 'menu-dropdown-item-active' : 'menu-dropdown-item-inactive'">
                                                    "Add new product"
                                                </a>
                                            </li>
                                            <li>
                                                <a href="#" class="menu-dropdown-item group"
                                                    x-bind:class="(page === 'list-categories') ? 'menu-dropdown-item-active' : 'menu-dropdown-item-inactive'">
                                                    "Categories"
                                                </a>
                                            </li>
                                            <li>
                                                <a href="#" class="menu-dropdown-item group"
                                                    x-bind:class="(page === 'list-attributes') ? 'menu-dropdown-item-active' : 'menu-dropdown-item-inactive'">
                                                    "Attributes"
                                                </a>
                                            </li>
                                        </ul>
                                    </div>
                                    // Dropdown Menu End
                                </li>
                                // Menu Item Products

                                // Menu Item Marketing
                                <li>
                                    <a
                                        href="#"
                                        x-on:click="selected = (selected === 'Marketing' ? '' : 'Marketing')"
                                        class="menu-item group"
                                        x-bind:class="(selected === 'Marketing') || (page === 'list-marketing' || page === 'create-marketing') ? 'menu-item-active' : 'menu-item-inactive'">
                                        <svg
                                            class="size-6"
                                            x-bind:class="(selected === 'Marketing') || (page === 'list-marketing' || page === 'create-marketing') ?  'menu-item-icon-active'  :'menu-item-icon-inactive'"
                                            "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="1.25" "stroke"="currentColor">
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M19.114 5.636a9 9 0 0 1 0 12.728M16.463 8.288a5.25 5.25 0 0 1 0 7.424M6.75 8.25l4.72-4.72a.75.75 0 0 1 1.28.53v15.88a.75.75 0 0 1-1.28.53l-4.72-4.72H4.51c-.88 0-1.704-.507-1.938-1.354A9.009 9.009 0 0 1 2.25 12c0-.83.112-1.633.322-2.396C2.806 8.756 3.63 8.25 4.51 8.25H6.75Z"></path>
                                        </svg>
                                        <span class="menu-item-text" x-bind:class="sidebarToggle ? 'lg:hidden' : ''">
                                            "Marketing"
                                        </span>
                                        <svg class="menu-item-arrow size-4" x-bind:class="(selected === 'Marketing') ? 'menu-item-arrow-active' : 'menu-item-arrow-inactive', sidebarToggle ? 'lg:hidden' : '' " "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="2" "stroke"="currentColor">
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="m19.5 8.25-7.5 7.5-7.5-7.5"></path>
                                        </svg>
                                    </a>

                                    // Dropdown Menu Start
                                    <div
                                        class="menu-dropdown"
                                        x-bind:class="(selected === 'Marketing') ? 'h-fit opacity-100' :'h-0 opacity-0'">
                                        <ul
                                            class="flex flex-col gap-1 mt-2 pl-9"
                                            x-bind:class="sidebarToggle ? 'lg:hidden' : 'flex'">
                                            <li>
                                                <a href="#"
                                                    class="menu-dropdown-item group"
                                                    x-bind:class="(page === 'list-campaigns') ? 'menu-dropdown-item-active' : 'menu-dropdown-item-inactive'">
                                                    "View campaigns"
                                                </a>
                                            </li>
                                            <li>
                                                <a href="#"
                                                    class="menu-dropdown-item group"
                                                    x-bind:class="(page === 'create-campaign') ? 'menu-dropdown-item-active' : 'menu-dropdown-item-inactive'">
                                                    "Add new campaign"
                                                </a>
                                            </li>
                                        </ul>
                                    </div>
                                    // Dropdown Menu End
                                </li>
                                // Menu Item Marketing

                                // Menu Item Analytics
                                <li>
                                    <a href="#" x-on:click="selected = (selected === 'Analytics' ? '' : 'Analytics')"
                                        class="menu-item group"
                                        x-bind:class=" (selected === 'Analytics') && (page === 'analytics') ? 'menu-item-active' : 'menu-item-inactive'">
                                        <svg class="size-6" x-bind:class="(selected === 'Analytics') && (page === 'analytics') ? 'menu-item-icon-active' : 'menu-item-icon-inactive'" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="1.25" "stroke"="currentColor">
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M2.25 18 9 11.25l4.306 4.306a11.95 11.95 0 0 1 5.814-5.518l2.74-1.22m0 0-5.94-2.281m5.94 2.28-2.28 5.941"></path>
                                        </svg>
                                        <span class="menu-item-text" x-bind:class="sidebarToggle ? 'lg:hidden' : ''">
                                            "Analytics"
                                        </span>
                                    </a>
                                </li>
                                // Menu Item Analytics

                                // Menu Item Emails
                                <li>
                                    <a href="/admin/emails" x-on:click="selected = (selected === 'Emails' ? '' : 'Emails')"
                                        class="menu-item group"
                                        x-bind:class=" (selected === 'Emails') && (page === 'emails') ? 'menu-item-active' : 'menu-item-inactive'">
                                        <svg class="size-6" x-bind:class="(selected === 'Emails') && (page === 'emails') ? 'menu-item-icon-active' : 'menu-item-icon-inactive'" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="1.25" "stroke"="currentColor">
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M21.75 6.75v10.5a2.25 2.25 0 0 1-2.25 2.25h-15a2.25 2.25 0 0 1-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0 0 19.5 4.5h-15a2.25 2.25 0 0 0-2.25 2.25m19.5 0v.243a2.25 2.25 0 0 1-1.07 1.916l-7.5 4.615a2.25 2.25 0 0 1-2.36 0L3.32 8.91a2.25 2.25 0 0 1-1.07-1.916V6.75"></path>
                                        </svg>
                                        <span class="menu-item-text" x-bind:class="sidebarToggle ? 'lg:hidden' : ''">
                                            "E-mails"
                                        </span>
                                    </a>
                                </li>
                                // Menu Item Emails
                            </ul>
                        </div>
                        <div class="menu-group">
                            <ul class="flex flex-col gap-3">
                                // Menu Item Settings
                                <li>
                                    <a href="#" x-on:click="selected = (selected === 'Settings' ? '' : 'Settings')"
                                        class="menu-item group"
                                        x-bind:class=" (selected === 'Settings') && (page === 'settings') ? 'menu-item-active' : 'menu-item-inactive'">
                                        <svg class="size-6" x-bind:class="(selected === 'Settings') && (page === 'settings') ? 'menu-item-icon-active' : 'menu-item-icon-inactive'" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="1.25" "stroke"="currentColor">
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M10.343 3.94c.09-.542.56-.94 1.11-.94h1.093c.55 0 1.02.398 1.11.94l.149.894c.07.424.384.764.78.93.398.164.855.142 1.205-.108l.737-.527a1.125 1.125 0 0 1 1.45.12l.773.774c.39.389.44 1.002.12 1.45l-.527.737c-.25.35-.272.806-.107 1.204.165.397.505.71.93.78l.893.15c.543.09.94.559.94 1.109v1.094c0 .55-.397 1.02-.94 1.11l-.894.149c-.424.07-.764.383-.929.78-.165.398-.143.854.107 1.204l.527.738c.32.447.269 1.06-.12 1.45l-.774.773a1.125 1.125 0 0 1-1.449.12l-.738-.527c-.35-.25-.806-.272-1.203-.107-.398.165-.71.505-.781.929l-.149.894c-.09.542-.56.94-1.11.94h-1.094c-.55 0-1.019-.398-1.11-.94l-.148-.894c-.071-.424-.384-.764-.781-.93-.398-.164-.854-.142-1.204.108l-.738.527c-.447.32-1.06.269-1.45-.12l-.773-.774a1.125 1.125 0 0 1-.12-1.45l.527-.737c.25-.35.272-.806.108-1.204-.165-.397-.506-.71-.93-.78l-.894-.15c-.542-.09-.94-.56-.94-1.109v-1.094c0-.55.398-1.02.94-1.11l.894-.149c.424-.07.765-.383.93-.78.165-.398.143-.854-.108-1.204l-.526-.738a1.125 1.125 0 0 1 .12-1.45l.773-.773a1.125 1.125 0 0 1 1.45-.12l.737.527c.35.25.807.272 1.204.107.397-.165.71-.505.78-.929l.15-.894Z"></path>
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"></path>
                                        </svg>
                                        <span class="menu-item-text" x-bind:class="sidebarToggle ? 'lg:hidden' : ''">
                                            "Settings"
                                        </span>
                                    </a>
                                </li>
                                // Menu Item Settings
                            </ul>
                        </div>
                    </nav>
                    // Sidebar Menu
                </aside>

                // ===== Sidebar End =====

                // ===== Content Area Start =====
                <div
                    class="relative flex flex-col flex-1 overflow-x-hidden overflow-y-auto">
                    // Small Device Overlay Start
                    <div
                        x-on:click="sidebarToggle = false"
                        x-bind:class="sidebarToggle ? 'block lg:hidden' : 'hidden'"
                        class="fixed w-full h-screen z-9 bg-neutral-900/50"></div>
                    // Small Device Overlay End

                    // ===== Header Start =====
                    <header
                        x-data="{menuToggle: false}"
                        class="sticky top-0 z-99999 flex w-full border-neutral-200 bg-white lg:border-b  ">
                        <div
                            class="flex grow flex-col items-center justify-between lg:flex-row lg:px-6">
                            <div
                                class="flex w-full items-center justify-between gap-2 border-b border-neutral-200 px-3 py-3 sm:gap-4 lg:justify-normal lg:border-b-0 lg:px-0 lg:py-4 ">
                                // Hamburger Toggle BTN
                                <button
                                    x-bind:class="sidebarToggle ? 'lg:bg-transparent  bg-neutral-100 ' : ''"
                                    class="z-99999 flex h-10 w-10 items-center justify-center rounded-lg border-neutral-200 text-neutral-500 lg:h-11 lg:w-11 lg:border  "
                                    x-on:click.stop="sidebarToggle = !sidebarToggle">
                                    <svg
                                        class="hidden fill-current lg:block"
                                        "width"="16"
                                        "height"="12"
                                        "viewBox"="0 0 16 12"
                                        "fill"="none">
                                        <path
                                            "fill-rule"="evenodd"
                                            "clip-rule"="evenodd"
                                            "d"="M0.583252 1C0.583252 0.585788 0.919038 0.25 1.33325 0.25H14.6666C15.0808 0.25 15.4166 0.585786 15.4166 1C15.4166 1.41421 15.0808 1.75 14.6666 1.75L1.33325 1.75C0.919038 1.75 0.583252 1.41422 0.583252 1ZM0.583252 11C0.583252 10.5858 0.919038 10.25 1.33325 10.25L14.6666 10.25C15.0808 10.25 15.4166 10.5858 15.4166 11C15.4166 11.4142 15.0808 11.75 14.6666 11.75L1.33325 11.75C0.919038 11.75 0.583252 11.4142 0.583252 11ZM1.33325 5.25C0.919038 5.25 0.583252 5.58579 0.583252 6C0.583252 6.41421 0.919038 6.75 1.33325 6.75L7.99992 6.75C8.41413 6.75 8.74992 6.41421 8.74992 6C8.74992 5.58579 8.41413 5.25 7.99992 5.25L1.33325 5.25Z"
                                            "fill"=""></path>
                                    </svg>

                                    <svg
                                        x-bind:class="sidebarToggle ? 'hidden' : 'block lg:hidden'"
                                        class="fill-current lg:hidden"
                                        "width"="24"
                                        "height"="24"
                                        "viewBox"="0 0 24 24"
                                        "fill"="none">
                                        <path
                                            "fill-rule"="evenodd"
                                            "clip-rule"="evenodd"
                                            "d"="M3.25 6C3.25 5.58579 3.58579 5.25 4 5.25L20 5.25C20.4142 5.25 20.75 5.58579 20.75 6C20.75 6.41421 20.4142 6.75 20 6.75L4 6.75C3.58579 6.75 3.25 6.41422 3.25 6ZM3.25 18C3.25 17.5858 3.58579 17.25 4 17.25L20 17.25C20.4142 17.25 20.75 17.5858 20.75 18C20.75 18.4142 20.4142 18.75 20 18.75L4 18.75C3.58579 18.75 3.25 18.4142 3.25 18ZM4 11.25C3.58579 11.25 3.25 11.5858 3.25 12C3.25 12.4142 3.58579 12.75 4 12.75L12 12.75C12.4142 12.75 12.75 12.4142 12.75 12C12.75 11.5858 12.4142 11.25 12 11.25L4 11.25Z"
                                            "fill"=""></path>
                                    </svg>

                                    // cross icon
                                    <svg
                                        x-bind:class="sidebarToggle ? 'block lg:hidden' : 'hidden'"
                                        class="fill-current"
                                        "width"="24"
                                        "height"="24"
                                        "viewBox"="0 0 24 24"
                                        "fill"="none">
                                        <path
                                            "fill-rule"="evenodd"
                                            "clip-rule"="evenodd"
                                            "d"="M6.21967 7.28131C5.92678 6.98841 5.92678 6.51354 6.21967 6.22065C6.51256 5.92775 6.98744 5.92775 7.28033 6.22065L11.999 10.9393L16.7176 6.22078C17.0105 5.92789 17.4854 5.92788 17.7782 6.22078C18.0711 6.51367 18.0711 6.98855 17.7782 7.28144L13.0597 12L17.7782 16.7186C18.0711 17.0115 18.0711 17.4863 17.7782 17.7792C17.4854 18.0721 17.0105 18.0721 16.7176 17.7792L11.999 13.0607L7.28033 17.7794C6.98744 18.0722 6.51256 18.0722 6.21967 17.7794C5.92678 17.4865 5.92678 17.0116 6.21967 16.7187L10.9384 12L6.21967 7.28131Z"
                                            "fill"=""></path>
                                    </svg>
                                </button>
                                // Hamburger Toggle BTN

                                <a href="index.html" class="lg:hidden">
                                    <img class="" src="src/images/logo/logo.svg" alt="Logo" />
                                </a>

                                // Application nav menu button
                                <button
                                    class="z-99999 flex h-10 w-10 items-center justify-center rounded-lg text-neutral-700 hover:bg-neutral-100 lg:hidden  "
                                    x-bind:class="menuToggle ? 'bg-neutral-100 ' : ''"
                                    x-on:click.stop="menuToggle = !menuToggle">
                                    <svg
                                        class="fill-current"
                                        "width"="24"
                                        "height"="24"
                                        "viewBox"="0 0 24 24"
                                        "fill"="none">
                                        <path
                                            "fill-rule"="evenodd"
                                            "clip-rule"="evenodd"
                                            "d"="M5.99902 10.4951C6.82745 10.4951 7.49902 11.1667 7.49902 11.9951V12.0051C7.49902 12.8335 6.82745 13.5051 5.99902 13.5051C5.1706 13.5051 4.49902 12.8335 4.49902 12.0051V11.9951C4.49902 11.1667 5.1706 10.4951 5.99902 10.4951ZM17.999 10.4951C18.8275 10.4951 19.499 11.1667 19.499 11.9951V12.0051C19.499 12.8335 18.8275 13.5051 17.999 13.5051C17.1706 13.5051 16.499 12.8335 16.499 12.0051V11.9951C16.499 11.1667 17.1706 10.4951 17.999 10.4951ZM13.499 11.9951C13.499 11.1667 12.8275 10.4951 11.999 10.4951C11.1706 10.4951 10.499 11.1667 10.499 11.9951V12.0051C10.499 12.8335 11.1706 13.5051 11.999 13.5051C12.8275 13.5051 13.499 12.0051V11.9951Z"
                                            "fill"=""></path>
                                    </svg>
                                </button>
                                // Application nav menu button

                                <div class="hidden lg:block">
                                    <form>
                                        <div class="relative">
                                            <span class="absolute top-1/2 left-4 -translate-y-1/2">
                                                <svg
                                                    class="fill-neutral-500 "
                                                    "width"="20"
                                                    "height"="20"
                                                    "viewBox"="0 0 20 20"
                                                    "fill"="none">
                                                    <path
                                                        "fill-rule"="evenodd"
                                                        "clip-rule"="evenodd"
                                                        "d"="M3.04175 9.37363C3.04175 5.87693 5.87711 3.04199 9.37508 3.04199C12.8731 3.04199 15.7084 5.87693 15.7084 9.37363C15.7084 12.8703 12.8731 15.7053 9.37508 15.7053C5.87711 15.7053 3.04175 12.8703 3.04175 9.37363ZM9.37508 1.54199C5.04902 1.54199 1.54175 5.04817 1.54175 9.37363C1.54175 13.6991 5.04902 17.2053 9.37508 17.2053C11.2674 17.2053 13.003 16.5344 14.357 15.4176L17.177 18.238C17.4699 18.5309 17.9448 18.5309 18.2377 18.238C18.5306 17.9451 18.5306 17.4703 18.2377 17.1774L15.418 14.3573C16.5365 13.0033 17.2084 11.2669 17.2084 9.37363C17.2084 5.04817 13.7011 1.54199 9.37508 1.54199Z"
                                                        "fill"=""></path>
                                                </svg>
                                            </span>
                                            <input
                                                type="text"
                                                placeholder="Search or type command..."
                                                id="search-input"
                                                class=" shadow-xs focus:border-brand-300 focus:ring-brand-500/10  h-11 w-full rounded-lg border border-neutral-200 bg-transparent py-2.5 pr-14 pl-12 text-sm text-neutral-800 placeholder:text-neutral-400 focus:ring-3 focus:outline-hidden xl:w-[430px] " />
                                        </div>
                                    </form>
                                </div>
                            </div>

                            <div
                                x-bind:class="menuToggle ? 'flex' : 'hidden'"
                                class="shadow-md w-full items-center justify-between gap-4 px-5 py-4 lg:flex lg:justify-end lg:px-0 lg:shadow-none">
                                <div class="2xsm:gap-3 flex items-center gap-2">
                                    // Notification Menu Area
                                    <div
                                        class="relative"
                                        x-data="{ dropdownOpen: false, notifying: true }"
                                        x-on:click.outside="dropdownOpen = false">
                                        <button
                                            class="relative flex h-11 w-11 items-center justify-center rounded-full border border-neutral-200 bg-white text-neutral-500 transition-colors hover:bg-neutral-100 hover:text-neutral-700"
                                            x-on:click.prevent="dropdownOpen = ! dropdownOpen; notifying = false">
                                            <span
                                                x-bind:class="!notifying ? 'hidden' : 'flex'"
                                                class="absolute top-0.5 right-0 z-1 h-2 w-2 rounded-full bg-orange-400">
                                                <span
                                                    class="absolute -z-1 inline-flex h-full w-full animate-ping rounded-full bg-orange-400 opacity-75"></span>
                                            </span>
                                            <svg
                                                class="fill-current"
                                                "width"="20"
                                                "height"="20"
                                                "viewBox"="0 0 20 20"
                                                "fill"="none">
                                                <path
                                                    "fill-rule"="evenodd"
                                                    "clip-rule"="evenodd"
                                                    "d"="M10.75 2.29248C10.75 1.87827 10.4143 1.54248 10 1.54248C9.58583 1.54248 9.25004 1.87827 9.25004 2.29248V2.83613C6.08266 3.20733 3.62504 5.9004 3.62504 9.16748V14.4591H3.33337C2.91916 14.4591 2.58337 14.7949 2.58337 15.2091C2.58337 15.6234 2.91916 15.9591 3.33337 15.9591H4.37504H15.625H16.6667C17.0809 15.9591 17.4167 15.6234 17.4167 15.2091C17.4167 14.7949 17.0809 14.4591 16.6667 14.4591H16.375V9.16748C16.375 5.9004 13.9174 3.20733 10.75 2.83613V2.29248ZM14.875 14.4591V9.16748C14.875 6.47509 12.6924 4.29248 10 4.29248C7.30765 4.29248 5.12504 6.47509 5.12504 9.16748V14.4591H14.875ZM8.00004 17.7085C8.00004 18.1228 8.33583 18.4585 8.75004 18.4585H11.25C11.6643 18.4585 12 18.1228 12 17.7085C12 17.2943 11.6643 16.9585 11.25 16.9585H8.75004C8.33583 16.9585 8.00004 17.2943 8.00004 17.7085Z"
                                                    "fill"=""></path>
                                            </svg>
                                        </button>

                                        // Dropdown Start
                                        <div
                                            x-show="dropdownOpen"
                                            class="shadow-lg  absolute -right-[240px] mt-[17px] flex h-[480px] w-[350px] flex-col rounded-2xl border border-neutral-200 bg-white p-3 sm:w-[361px] lg:right-0 ">
                                            <div
                                                class="mb-3 flex items-center justify-between border-b border-neutral-100 pb-3 ">
                                                <h5
                                                    class="text-lg font-semibold text-neutral-800 ">
                                                    "Notification"
                                                </h5>

                                                <button
                                                    x-on:click="dropdownOpen = false"
                                                    class="text-neutral-500 ">
                                                    <svg
                                                        class="fill-current"
                                                        "width"="24"
                                                        "height"="24"
                                                        "viewBox"="0 0 24 24"
                                                        "fill"="none">
                                                        <path
                                                            "fill-rule"="evenodd"
                                                            "clip-rule"="evenodd"
                                                            "d"="M6.21967 7.28131C5.92678 6.98841 5.92678 6.51354 6.21967 6.22065C6.51256 5.92775 6.98744 5.92775 7.28033 6.22065L11.999 10.9393L16.7176 6.22078C17.0105 5.92789 17.4854 5.92788 17.7782 6.22078C18.0711 6.51367 18.0711 6.98855 17.7782 7.28144L13.0597 12L17.7782 16.7186C18.0711 17.0115 18.0711 17.4863 17.7782 17.7792C17.4854 18.0721 17.0105 18.0721 16.7176 17.7792L11.999 13.0607L7.28033 17.7794C6.98744 18.0722 6.51256 18.0722 6.21967 17.7794C5.92678 17.4865 5.92678 17.0116 6.21967 16.7187L10.9384 12L6.21967 7.28131Z"
                                                            "fill"=""></path>
                                                    </svg>
                                                </button>
                                            </div>

                                            <ul class="custom-scrollbar flex h-auto flex-col overflow-y-auto">
                                                <li>
                                                    <a
                                                        class="flex gap-3 rounded-lg border-b border-neutral-100 p-3 px-4.5 py-3 hover:bg-neutral-100  "
                                                        href="#">
                                                        <span
                                                            class="relative z-1 block h-10 w-full max-w-10 rounded-full">
                                                            <img
                                                                src="src/images/user/user-02.jpg"
                                                                alt="User"
                                                                class="overflow-hidden rounded-full" />
                                                            <span
                                                                class="bg-success-500 absolute right-0 bottom-0 z-10 h-2.5 w-full max-w-2.5 rounded-full border-[1.5px] border-white "></span>
                                                        </span>

                                                        <span class="block">
                                                            <span
                                                                class="text-sm mb-1.5 block text-neutral-500 ">
                                                                <span class="font-medium text-neutral-800 ">"Terry Franci"</span>
                                                                " requests permission to change "
                                                                <span class="font-medium text-neutral-800 ">"Project - Nganter App"</span>
                                                            </span>

                                                            <span
                                                                class="text-xs flex items-center gap-2 text-neutral-500 ">
                                                                <span>"Project"</span>
                                                                <span class="h-1 w-1 rounded-full bg-neutral-400"></span>
                                                                <span>"5 min ago"</span>
                                                            </span>
                                                        </span>
                                                    </a>
                                                </li>

                                                <li>
                                                    <a
                                                        class="flex gap-3 rounded-lg border-b border-neutral-100 p-3 px-4.5 py-3 hover:bg-neutral-100  "
                                                        href="#">
                                                        <span
                                                            class="relative z-1 block h-10 w-full max-w-10 rounded-full">
                                                            <img
                                                                src="src/images/user/user-03.jpg"
                                                                alt="User"
                                                                class="overflow-hidden rounded-full" />
                                                            <span
                                                                class="bg-success-500 absolute right-0 bottom-0 z-10 h-2.5 w-full max-w-2.5 rounded-full border-[1.5px] border-white "></span>
                                                        </span>

                                                        <span class="block">
                                                            <span
                                                                class="text-sm mb-1.5 block text-neutral-500 ">
                                                                <span class="font-medium text-neutral-800 ">"Alena Franci"</span>
                                                                " requests permission to change "
                                                                <span class="font-medium text-neutral-800 ">"Project - Nganter App"</span>
                                                            </span>

                                                            <span
                                                                class="text-xs flex items-center gap-2 text-neutral-500 ">
                                                                <span>"Project"</span>
                                                                <span class="h-1 w-1 rounded-full bg-neutral-400"></span>
                                                                <span>"8 min ago"</span>
                                                            </span>
                                                        </span>
                                                    </a>
                                                </li>

                                                <li>
                                                    <a
                                                        class="flex gap-3 rounded-lg border-b border-neutral-100 p-3 px-4.5 py-3 hover:bg-neutral-100  "
                                                        href="#">
                                                        <span
                                                            class="relative z-1 block h-10 w-full max-w-10 rounded-full">
                                                            <img
                                                                src="src/images/user/user-04.jpg"
                                                                alt="User"
                                                                class="overflow-hidden rounded-full" />
                                                            <span
                                                                class="bg-success-500 absolute right-0 bottom-0 z-10 h-2.5 w-full max-w-2.5 rounded-full border-[1.5px] border-white "></span>
                                                        </span>

                                                        <span class="block">
                                                            <span
                                                                class="text-sm mb-1.5 block text-neutral-500 ">
                                                                <span class="font-medium text-neutral-800 ">"Jocelyn Kenter"</span>
                                                                " requests permission to change "
                                                                <span class="font-medium text-neutral-800 ">"Project - Nganter App"</span>
                                                            </span>

                                                            <span
                                                                class="text-xs flex items-center gap-2 text-neutral-500 ">
                                                                <span>"Project"</span>
                                                                <span class="h-1 w-1 rounded-full bg-neutral-400"></span>
                                                                <span>"15 min ago"</span>
                                                            </span>
                                                        </span>
                                                    </a>
                                                </li>

                                                <li>
                                                    <a
                                                        class="flex gap-3 rounded-lg border-b border-neutral-100 p-3 px-4.5 py-3 hover:bg-neutral-100  "
                                                        href="#">
                                                        <span
                                                            class="relative z-1 block h-10 w-full max-w-10 rounded-full">
                                                            <img
                                                                src="src/images/user/user-05.jpg"
                                                                alt="User"
                                                                class="overflow-hidden rounded-full" />
                                                            <span
                                                                class="bg-error-500 absolute right-0 bottom-0 z-10 h-2.5 w-full max-w-2.5 rounded-full border-[1.5px] border-white "></span>
                                                        </span>

                                                        <span class="block">
                                                            <span
                                                                class="text-sm mb-1.5 block text-neutral-500 ">
                                                                <span class="font-medium text-neutral-800 ">"Brandon Philips"</span>
                                                                " requests permission to change "
                                                                <span class="font-medium text-neutral-800 ">"Project - Nganter App"</span>
                                                            </span>

                                                            <span
                                                                class="text-xs flex items-center gap-2 text-neutral-500 ">
                                                                <span>"Project"</span>
                                                                <span class="h-1 w-1 rounded-full bg-neutral-400"></span>
                                                                <span>"1 hr ago"</span>
                                                            </span>
                                                        </span>
                                                    </a>
                                                </li>
                                            </ul>

                                            <a
                                                href="#"
                                                class="text-sm shadow-xs mt-3 flex justify-center rounded-lg border border-neutral-300 bg-white p-3 font-medium text-neutral-700 hover:bg-neutral-50 hover:text-neutral-800     ">
                                                "View All Notifications"
                                            </a>
                                        </div>
                                        // Dropdown Start End
                                    </div>
                                    // Notification Menu Area
                                </div>

                                // User Area
                                <div
                                    class="relative"
                                    x-data=(format!(r#"{{ dropdownOpen: false, profileImage: '{}' }}"#, admin.profile_image_url.as_deref().unwrap_or("")))
                                    x-on:click.outside="dropdownOpen = false"
                                >
                                    <a
                                        class="flex items-center text-neutral-700 "
                                        href="#"
                                        x-on:click.prevent="dropdownOpen = ! dropdownOpen">
                                        <span class="mr-3 h-11 w-11 border border-neutral-200 rounded-full">
                                            <template x-if="profileImage">
                                                <img x-bind:src="profileImage" alt="admin profile image" class="object-cover size-full rounded-full" />
                                            </template>
                                            <template x-if="!profileImage">
                                                <svg class="object-cover size-full rounded-full" "viewBox"="312.81 0 401 401">
                                                    <path "fill"="#e4e6e7" "d"="M268.073-44.735h490.423v490.423H268.073z"></path>
                                                    <path "fill"="#aeb4b7" "d"="M513.81 267.142c-103.361 0-187.754 58.93-192.475 132.842h384.988c-4.733-73.918-89.157-132.842-192.512-132.842m96.605-109.116c0 57.17-42.935 103.516-95.896 103.516s-95.895-46.346-95.895-103.516S461.559 54.51 514.52 54.51c52.968 0 95.896 46.352 95.896 103.515z"></path>
                                                </svg>
                                            </template>
                                        </span>

                                        <span class="text-sm mr-1 block font-medium">
                                            (admin.first_name)" "(admin.last_name)
                                        </span>

                                        <svg
                                            x-bind:class="dropdownOpen && 'rotate-180'"
                                            class="stroke-neutral-500 "
                                            "width"="18"
                                            "height"="20"
                                            "viewBox"="0 0 18 20"
                                            "fill"="none">
                                            <path
                                                "d"="M4.3125 8.65625L9 13.3437L13.6875 8.65625"
                                                "stroke"=""
                                                "stroke-width"="1.5"
                                                "stroke-linecap"="round"
                                                "stroke-linejoin"="round"></path>
                                        </svg>
                                    </a>

                                    // Dropdown Start
                                    <div
                                        x-show="dropdownOpen"
                                        class="shadow-lg absolute right-0 mt-[17px] flex w-[260px] flex-col rounded-2xl border border-neutral-200 bg-white p-3">
                                        <div>
                                            <span class="text-sm block font-medium text-neutral-700">
                                                (admin.first_name)" "(admin.last_name)
                                            </span>
                                            <span class="text-xs mt-0.5 block text-neutral-500">
                                                (admin.email)
                                            </span>
                                        </div>

                                        <ul class="flex flex-col gap-1 border-b border-neutral-200 pt-4 pb-3 ">
                                            <li>
                                                <a href="/admin/account" class="group text-sm flex items-center gap-3 rounded-lg px-3 py-2 font-medium text-neutral-700 hover:bg-neutral-100 hover:text-neutral-700">
                                                    <svg class="fill-neutral-500 group-hover:fill-neutral-700" "fill"="none" "width"="24" "height"="24" "viewBox"="0 0 24 24">
                                                        <path "fill"="" "fill-rule"="evenodd" "clip-rule"="evenodd" "d"="M10.4858 3.5L13.5182 3.5C13.9233 3.5 14.2518 3.82851 14.2518 4.23377C14.2518 5.9529 16.1129 7.02795 17.602 6.1682C17.9528 5.96567 18.4014 6.08586 18.6039 6.43667L20.1203 9.0631C20.3229 9.41407 20.2027 9.86286 19.8517 10.0655C18.3625 10.9253 18.3625 13.0747 19.8517 13.9345C20.2026 14.1372 20.3229 14.5859 20.1203 14.9369L18.6039 17.5634C18.4013 17.9142 17.9528 18.0344 17.602 17.8318C16.1129 16.9721 14.2518 18.0471 14.2518 19.7663C14.2518 20.1715 13.9233 20.5 13.5182 20.5H10.4858C10.0804 20.5 9.75182 20.1714 9.75182 19.766C9.75182 18.0461 7.88983 16.9717 6.40067 17.8314C6.04945 18.0342 5.60037 17.9139 5.39767 17.5628L3.88167 14.937C3.67903 14.586 3.79928 14.1372 4.15026 13.9346C5.63949 13.0748 5.63946 10.9253 4.15025 10.0655C3.79926 9.86282 3.67901 9.41401 3.88165 9.06303L5.39764 6.43725C5.60034 6.08617 6.04943 5.96581 6.40065 6.16858C7.88982 7.02836 9.75182 5.9539 9.75182 4.23399C9.75182 3.82862 10.0804 3.5 10.4858 3.5ZM13.5182 2L10.4858 2C9.25201 2 8.25182 3.00019 8.25182 4.23399C8.25182 4.79884 7.64013 5.15215 7.15065 4.86955C6.08213 4.25263 4.71559 4.61859 4.0986 5.68725L2.58261 8.31303C1.96575 9.38146 2.33183 10.7477 3.40025 11.3645C3.88948 11.647 3.88947 12.3531 3.40026 12.6355C2.33184 13.2524 1.96578 14.6186 2.58263 15.687L4.09863 18.3128C4.71562 19.3814 6.08215 19.7474 7.15067 19.1305C7.64015 18.8479 8.25182 19.2012 8.25182 19.766C8.25182 20.9998 9.25201 22 10.4858 22H13.5182C14.7519 22 15.7518 20.9998 15.7518 19.7663C15.7518 19.2015 16.3632 18.8487 16.852 19.1309C17.9202 19.7476 19.2862 19.3816 19.9029 18.3134L21.4193 15.6869C22.0361 14.6185 21.6701 13.2523 20.6017 12.6355C20.1125 12.3531 20.1125 11.647 20.6017 11.3645C21.6701 10.7477 22.0362 9.38152 21.4193 8.3131L19.903 5.68667C19.2862 4.61842 17.9202 4.25241 16.852 4.86917C16.3632 5.15138 15.7518 4.79856 15.7518 4.23377C15.7518 3.00024 14.7519 2 13.5182 2ZM9.6659 11.9999C9.6659 10.7103 10.7113 9.66493 12.0009 9.66493C13.2905 9.66493 14.3359 10.7103 14.3359 11.9999C14.3359 13.2895 13.2905 14.3349 12.0009 14.3349C10.7113 14.3349 9.6659 13.2895 9.6659 11.9999ZM12.0009 8.16493C9.88289 8.16493 8.1659 9.88191 8.1659 11.9999C8.1659 14.1179 9.88289 15.8349 12.0009 15.8349C14.1189 15.8349 15.8359 14.1179 15.8359 11.9999C15.8359 9.88191 14.1189 8.16493 12.0009 8.16493Z"></path>
                                                    </svg>
                                                    "Account settings"
                                                </a>
                                            </li>
                                        </ul>
                                        <a href="/admin/logout" class="group text-sm mt-3 flex items-center gap-3 rounded-lg px-3 py-2 font-medium text-neutral-700 hover:bg-neutral-100 hover:text-neutral-700   ">
                                            <svg "fill"="none" class="fill-neutral-500 group-hover:fill-neutral-700 " "width"="24" "height"="24" "viewBox"="0 0 24 24">
                                                <path "fill"="" "fill-rule"="evenodd" "clip-rule"="evenodd" "d"="M15.1007 19.247C14.6865 19.247 14.3507 18.9112 14.3507 18.497L14.3507 14.245H12.8507V18.497C12.8507 19.7396 13.8581 20.747 15.1007 20.747H18.5007C19.7434 20.747 20.7507 19.7396 20.7507 18.497L20.7507 5.49609C20.7507 4.25345 19.7433 3.24609 18.5007 3.24609H15.1007C13.8581 3.24609 12.8507 4.25345 12.8507 5.49609V9.74501L14.3507 9.74501V5.49609C14.3507 5.08188 14.6865 4.74609 15.1007 4.74609L18.5007 4.74609C18.9149 4.74609 19.2507 5.08188 19.2507 5.49609L19.2507 18.497C19.2507 18.9112 18.9149 19.247 18.5007 19.247H15.1007ZM3.25073 11.9984C3.25073 12.2144 3.34204 12.4091 3.48817 12.546L8.09483 17.1556C8.38763 17.4485 8.86251 17.4487 9.15549 17.1559C9.44848 16.8631 9.44863 16.3882 9.15583 16.0952L5.81116 12.7484L16.0007 12.7484C16.4149 12.7484 16.7507 12.4127 16.7507 11.9984C16.7507 11.5842 16.4149 11.2484 16.0007 11.2484L5.81528 11.2484L9.15585 7.90554C9.44864 7.61255 9.44847 7.13767 9.15547 6.84488C8.86248 6.55209 8.3876 6.55226 8.09481 6.84525L3.52309 11.4202C3.35673 11.5577 3.25073 11.7657 3.25073 11.7657 11.9984Z"></path>
                                            </svg>
                                            "Logout"
                                        </a>
                                    </div>
                                    // Dropdown Start End
                                </div>
                                // User Area End
                            </div>
                        </div>
                    </header>
                    // ===== Header End =====

                    // ===== Main Content Start =====
                    <main class="relative size-full">
                        (content)
                    </main>
                    // ===== Main Content End =====
                </div>
                // ===== Content Area End =====
            </div>
            // ===== Page Wrapper End =====
        </body>

        </html>
    }
}
