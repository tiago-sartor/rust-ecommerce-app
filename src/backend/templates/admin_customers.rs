use crate::server::handlers::backend::customers::CustomersData;
use crate::utils::{Context, helpers, hypertext_elements};
use hypertext::prelude::*;

pub fn admin_customers_template(ctx: &Context<(), CustomersData>) -> impl Renderable {
    let CustomersData { customers, count, page, limit } = &ctx.data;

    let showing_from = (page - 1) * limit + 1;
    let showing_to = i64::min(page * limit, *count);
    let total_pages = (*count as f64 / *limit as f64).ceil() as i64;

    let ids: Vec<i64> = customers.iter().map(|c| c.id).collect();
    let ids_json = serde_json::to_string(&ids).unwrap_or("[]".to_string());

    rsx! {
        <div x-data=(format!("checkboxSelector({ids_json})")) class="mx-auto max-w-(--breakpoint-2xl) px-5 py-4 md:p-6">
            <div class="overflow-hidden rounded-xl border border-neutral-200 bg-white dark:border-neutral-800 dark:bg-white/3">
                <div class="flex flex-col justify-between gap-5 border-b border-neutral-200 px-5 py-4 sm:flex-row sm:items-center dark:border-neutral-800">
                    <div>
                        <h3 class="text-lg font-semibold dark:text-white/90">
                            "Customers"
                        </h3>
                        <p class="text-sm text-neutral-500 dark:text-neutral-400">
                            "List of all registered customers."
                        </p>
                    </div>
                    <div class="relative">
                        <span class="absolute top-1/2 left-4 -translate-y-1/2 text-neutral-500 dark:text-neutral-400">
                            <svg class="fill-current" "width"="20" "height"="20" "viewBox"="0 0 20 20" "fill"="none">
                                <path "fill-rule"="evenodd" "clip-rule"="evenodd" "d"="M3.04199 9.37363C3.04199 5.87693 5.87735 3.04199 9.37533 3.04199C12.8733 3.04199 15.7087 5.87693 15.7087 9.37363C15.7087 12.8703 12.8733 15.7053 9.37533 15.7053C5.87735 15.7053 3.04199 12.8703 3.04199 9.37363ZM9.37533 1.54199C5.04926 1.54199 1.54199 5.04817 1.54199 9.37363C1.54199 13.6991 5.04926 17.2053 9.37533 17.2053C11.2676 17.2053 13.0032 16.5344 14.3572 15.4176L17.1773 18.238C17.4702 18.5309 17.945 18.5309 18.2379 18.238C18.5308 17.9451 18.5309 17.4703 18.238 17.1773L15.4182 14.3573C16.5367 13.0033 17.2087 11.2669 17.2087 9.37363C17.2087 5.04817 13.7014 1.54199 9.37533 1.54199Z" "fill"=""></path>
                            </svg>
                        </span>
                        <input type="text" placeholder="Search..." class="dark:bg-dark-900 shadow-xs focus:border-indigo-300 focus:ring-indigo-500/10 dark:focus:border-indigo-800 h-11 w-full rounded-lg border border-neutral-300 bg-transparent py-2.5 pr-4 pl-11 text-sm placeholder:text-neutral-400 focus:ring-3 focus:outline-hidden xl:w-[300px] dark:border-neutral-750 dark:bg-neutral-900 dark:text-white/90 dark:placeholder:text-white/30">
                    </div>
                </div>
                <div class="border-b border-neutral-200 px-5 py-4 dark:border-neutral-800">
                    <div class="flex gap-3 sm:justify-between">
                        <div class="relative" x-data="{ showBulkActions: false }">
                            <button
                                class="shadow-xs flex h-11 w-full items-center justify-center gap-2 rounded-lg border border-neutral-300 bg-white px-4 py-2.5 text-sm font-medium text-neutral-750 sm:w-auto sm:min-w-[100px] dark:border-neutral-750 dark:bg-neutral-800 dark:text-neutral-400"
                                x-on:click="showBulkActions = !showBulkActions" type="button">
                                "Bulk actions"
                                <svg class="size-4" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="2" "stroke"="currentColor">
                                    <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="m19.5 8.25-7.5 7.5-7.5-7.5"></path>
                                </svg>
                            </button>
                            <div x-cloak x-show="showBulkActions" x-on:click.away="showBulkActions = false"
                                class="absolute left-0 z-10 mt-2 w-42 rounded-lg border border-neutral-200 bg-white p-3 shadow-lg dark:border-neutral-750 dark:bg-neutral-800">
                                <form method="POST" action="/admin/customers/bulk-actions">
                                    <input type="hidden" name="csrf_token" value=(ctx.csrf_token.0) />
                                    <input type="hidden" name="ids" x-bind:value="JSON.stringify(selected)" />
                                    <button type="submit" name="action" value="resend" class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium text-neutral-750 hover:bg-neutral-100 hover:text-neutral-750">
                                        "Resend selected"
                                    </button>
                                    <button type="submit" name="action" value="delete" class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium text-neutral-750 hover:bg-red-50 hover:text-red-700">
                                        "Delete selected"
                                    </button>
                                </form>
                            </div>
                        </div>
                        <div class="flex-col gap-3 sm:flex sm:flex-row sm:items-center">
                            <div class="relative" x-data="{ showFilter: false }">
                                <button
                                    class="shadow-xs flex h-11 w-full items-center justify-center gap-2 rounded-lg border border-neutral-300 bg-white px-4 py-2.5 text-sm font-medium text-neutral-750 sm:w-auto sm:min-w-[100px] dark:border-neutral-750 dark:bg-neutral-800 dark:text-neutral-400"
                                    x-on:click="showFilter = !showFilter" type="button">
                                    <svg "width"="20" "height"="20" "viewBox"="0 0 20 20" "fill"="none">
                                        <path "d"="M14.6537 5.90414C14.6537 4.48433 13.5027 3.33331 12.0829 3.33331C10.6631 3.33331 9.51206 4.48433 9.51204 5.90415M14.6537 5.90414C14.6537 7.32398 13.5027 8.47498 12.0829 8.47498C10.663 8.47498 9.51204 7.32398 9.51204 5.90415M14.6537 5.90414L17.7087 5.90411M9.51204 5.90415L2.29199 5.90411M5.34694 14.0958C5.34694 12.676 6.49794 11.525 7.91777 11.525C9.33761 11.525 10.4886 12.676 10.4886 14.0958M5.34694 14.0958C5.34694 15.5156 6.49794 16.6666 7.91778 16.6666C9.33761 16.6666 10.4886 15.5156 10.4886 14.0958M5.34694 14.0958L2.29199 14.0958M10.4886 14.0958L17.7087 14.0958" "stroke"="currentColor" "stroke-width"="1.5" "stroke-linecap"="round" "stroke-linejoin"="round"></path>
                                    </svg>
                                    "Filter"
                                </button>
                                <div x-cloak x-show="showFilter" x-on:click.away="showFilter = false" class="absolute flex flex-col gap-1 right-0 z-10 mt-2 w-40 rounded-lg border border-neutral-200 bg-white p-3 shadow-lg dark:border-neutral-750 dark:bg-neutral-800">
                                    <a href="/admin/customers?filter_by=all" class="cursor-pointer text-sm flex items-center gap-3 rounded-lg px-3 py-2 font-medium text-neutral-750 hover:bg-neutral-100 hover:text-neutral-750">
                                        "All"
                                    </a>
                                    <a href=(format!("/admin/customers?filter_by={}", "sent")) class="cursor-pointer text-sm flex items-center gap-3 rounded-lg px-3 py-2 font-medium text-neutral-750 hover:bg-neutral-100 hover:text-neutral-750">
                                        "Subscribed"
                                    </a>
                                    <a href=(format!("/admin/customers?filter_by={}", "failed")) class="cursor-pointer text-sm flex items-center gap-3 rounded-lg px-3 py-2 font-medium text-neutral-750 hover:bg-neutral-100 hover:text-neutral-750">
                                        "Unsubscribed"
                                    </a>
                                </div>
                            </div>
                            <button class="shadow-xs flex w-full items-center justify-center gap-2 rounded-lg border border-neutral-300 bg-white px-4 py-[11px] text-sm font-medium text-neutral-750 sm:w-auto dark:border-neutral-750 dark:bg-neutral-800 dark:text-neutral-400">
                                <svg "width"="20" "height"="20" "viewBox"="0 0 20 20" "fill"="none">
                                    <path "d"="M16.6671 13.3333V15.4166C16.6671 16.1069 16.1074 16.6666 15.4171 16.6666H4.58301C3.89265 16.6666 3.33301 16.1069 3.33301 15.4166V13.3333M10.0013 3.33325L10.0013 13.3333M6.14553 7.18708L9.99958 3.33549L13.8539 7.18708" "stroke"="currentColor" "stroke-width"="1.5" "stroke-linecap"="round" "stroke-linejoin"="round"></path>
                                </svg>
                                "Export"
                            </button>
                        </div>
                    </div>
                </div>
                // Table
                <div x-data="details()" class="custom-scrollbar overflow-x-auto">
                    <table class="w-full table-auto">
                        <thead>
                            <tr class="border-b border-neutral-200 dark:divide-neutral-800 dark:border-neutral-800 text-xs font-bold text-neutral-600 dark:text-neutral-400">
                                <th class="w-14 px-5 py-4 text-left">
                                    <label
                                        class="cursor-pointer text-sm font-medium select-none">
                                        <input type="checkbox" class="sr-only" x-on:change="toggleAll()" x-bind:checked="isAllSelected()">
                                        <span
                                            x-bind:class="isAllSelected() ? 'border-indigo-500 bg-indigo-500' : 'bg-transparent border-neutral-300 dark:border-neutral-750'"
                                            class="flex h-4 w-4 items-center justify-center rounded-sm border-[1.25px]">
                                            <span x-bind:class="isAllSelected() ? '' : 'opacity-0'">
                                                <svg "width"="12" "height"="12" "viewBox"="0 0 12 12" "fill"="none">
                                                    <path "d"="M10 3L4.5 8.5L2 6" "stroke"="white" "stroke-width"="1.6666" "stroke-linecap"="round" "stroke-linejoin"="round">
                                                    </path>
                                                </svg>
                                            </span>
                                        </span>
                                    </label>
                                </th>
                                <th class="px-5 py-4 text-left">
                                    "Customer"
                                </th>
                                <th class="px-5 py-4 text-right">
                                    "Orders"
                                </th>
                                <th class="px-5 py-4 text-right">
                                    "Total spent"
                                </th>
                                <th class="px-5 py-4 text-left">
                                    "Marketing"
                                </th>
                                <th class="px-5 py-4 text-left">
                                    <div class="relative">
                                        <span class="sr-only">"Actions"</span>
                                    </div>
                                </th>
                            </tr>
                        </thead>
                        <tbody class="divide-x divide-y divide-neutral-200 dark:divide-neutral-800">

                            @for customer in customers {
                                <tr class="transition hover:bg-neutral-50 dark:hover:bg-neutral-900">
                                    <td class="w-14 px-5 py-4 whitespace-nowrap">
                                        <label for=(customer.id) x-on:click.prevent=(format!("toggleSelect({})", customer.id)) class="cursor-pointer text-sm font-medium text-neutral-750 select-none dark:text-neutral-400">
                                            <input id=(customer.id) type="checkbox" class="sr-only" x-bind:checked=(format!("selected.includes({})", customer.id)) >
                                            <span x-bind:class=(format!("selected.includes({}) ? 'border-indigo-500 bg-indigo-500' : 'bg-transparent border-neutral-300 dark:border-neutral-750'", customer.id)) class="flex h-4 w-4 items-center justify-center rounded-sm border-[1.25px]">
                                            <span x-bind:class=(format!("selected.includes({}) ? '' : 'opacity-0'", customer.id))>
                                                <svg "width"="12" "height"="12" "viewBox"="0 0 12 12" "fill"="none">
                                                    <path "d"="M10 3L4.5 8.5L2 6" "stroke"="white" "stroke-width"="1.6666" "stroke-linecap"="round" "stroke-linejoin"="round"></path>
                                                </svg>
                                            </span>
                                            </span>
                                        </label>
                                    </td>
                                    <td class="px-5 py-4 whitespace-nowrap">
                                        <div class="flex flex-col gap-1">
                                            <span class="text-sm font-semibold text-neutral-750 dark:text-neutral-400">
                                                (customer.first_name)" "(customer.last_name)
                                            </span>
                                            <span class="text-sm text-neutral-500 dark:text-neutral-400">
                                                (customer.email)
                                            </span>
                                        </div>
                                    </td>
                                    <td class="px-5 py-4 whitespace-nowrap text-right">
                                        <p class="text-sm dark:text-neutral-400">
                                            (customer.total_orders)
                                        </p>
                                    </td>
                                    <td class="px-5 py-4 whitespace-nowrap text-right">
                                        <p class="text-sm dark:text-neutral-400">
                                            (helpers::format_to_brl(customer.total_spent))
                                        </p>
                                    </td>
                                    <td class="px-5 py-4 whitespace-nowrap">
                                        @if customer.is_subscribed {
                                            <span class="text-xs rounded-full px-2 py-0.5 font-medium bg-green-50 dark:bg-green-500/15 text-green-700 dark:text-green-500">
                                                "Subscribed"
                                            </span>
                                        } @else {
                                            <span class="text-xs rounded-full px-2 py-0.5 font-medium bg-red-50 dark:bg-red-500/15 text-red-700 dark:text-red-500">
                                                "Unsubscribed"
                                            </span>
                                        }
                                    </td>
                                    <td class="px-5 py-4 whitespace-nowrap">
                                        <div x-data="actionDropdown()"  class="relative flex justify-center">
                                            <button x-on:click="toggle()" class="text-neutral-500 dark:text-neutral-400">
                                                <svg class="fill-current" "width"="24" "height"="24" "viewBox"="0 0 24 24" "fill"="none">
                                                    <path "fill-rule"="evenodd" "clip-rule"="evenodd" "d"="M5.99902 10.245C6.96552 10.245 7.74902 11.0285 7.74902 11.995V12.005C7.74902 12.9715 6.96552 13.755 5.99902 13.755C5.03253 13.755 4.24902 12.9715 4.24902 12.005V11.995C4.24902 11.0285 5.03253 10.245 5.99902 10.245ZM17.999 10.245C18.9655 10.245 19.749 11.0285 19.749 11.995V12.005C19.749 12.9715 18.9655 13.755 17.999 13.755C17.0325 13.755 16.249 12.9715 16.249 12.005V11.995C16.249 11.0285 17.0325 10.245 17.999 10.245ZM13.749 11.995C13.749 11.0285 12.9655 10.245 11.999 10.245C11.0325 10.245 10.249 11.0285 10.249 11.995V12.005C10.249 12.9715 11.0325 13.755 11.999 13.755C12.9655 13.755 13.749 12.9715 13.749 12.005V11.995Z" "fill"=""></path>
                                                </svg>
                                            </button>
                                            <div x-cloak x-show="open" x-on:click.outside="open = false" x-ref="dropdown" class="fixed z-10 w-40 rounded-lg border border-neutral-200 bg-white p-2 shadow-lg dark:border-neutral-750 dark:bg-neutral-800">
                                                <a href=(format!("/admin/customer-details/{}", customer.id)) class="text-xs flex w-full rounded-lg px-3 py-2 text-left font-medium text-neutral-500 hover:bg-neutral-100 hover:text-neutral-750 dark:text-neutral-400 dark:hover:bg-white/5 dark:hover:text-neutral-300">
                                                    "Details"
                                                </a>
                                                <form method="POST" action=(format!("/admin/customers/{}/delete", customer.id))>
                                                    <input type="hidden" name="csrf_token" value=(ctx.csrf_token.0) />
                                                    <button class="text-xs flex w-full rounded-lg px-3 py-2 text-left font-medium text-neutral-500 hover:bg-red-50 hover:text-red-700 dark:text-neutral-400 dark:hover:bg-white/5 dark:hover:text-neutral-300">
                                                        "Delete"
                                                    </button>
                                                </form>
                                            </div>
                                        </div>
                                    </td>
                                </tr>
                            }
                        </tbody>
                    </table>
                    // ===== Start Details Modal =====
                    <div x-data="{open: false}" x-on:keyup.escape.window="openDetailsModal = false">
                        // Background Overlay
                        <div x-cloak x-show="openDetailsModal" "x-transition.opacity.duration.500ms" class="fixed inset-0 flex items-center justify-center p-4 pt-22 md:p-6 md:pt-25 bg-black/75 backdrop-blur-[2px] z-999" aria-hidden="true">
                            // Modal Container
                            <div x-on:click.outside="openDetailsModal = false" "x-transition.opacity.duration.500ms" class="relative w-full max-w-5xl max-h-full flex flex-col p-5 md:p-6 rounded-lg bg-white shadow-2xl z-1000 overflow-y-auto">
                                // Loading Indicator
                                <div x-show="loading" class="absolute inset-0 z-1001 flex items-center justify-center bg-white">
                                    <div class="size-16 animate-spin border-4 border-indigo-500 border-t-transparent rounded-full"></div>
                                </div>
                                // Close Button
                                <div class="flex items-center justify-end mb-4">
                                    <button x-on:click="openDetailsModal = false" class="relative flex items-center justify-center text-neutral-400 hover:text-neutral-600" type="button" aria-label="Close details">
                                        <svg class="size-6.5" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="1.25" "stroke"="currentColor" aria-hidden="true">
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M6 18 18 6M6 6l12 12"></path>
                                        </svg>
                                    </button>
                                </div>
                                // Details Content
                                <iframe x-bind:srcdoc="html_body" class="w-full h-120 border-none rounded-md overflow-y-auto" allow="fullscreen *"></iframe>
                                <div class="mt-4 text-sm">
                                    <div class="mb-2 font-semibold">"Server response:"</div>
                                    <pre class="max-h-52 p-4 text-neutral-400 bg-neutral-800 rounded-md overflow-auto"><code class="block" x-text="server_response"></code></pre>
                                </div>
                            </div>
                        </div>
                    </div>
                    // ===== End Details Modal =====
                </div>
                <div
                    class="flex flex-col items-center justify-between border-t border-neutral-200 px-5 py-4 sm:flex-row dark:border-neutral-800">
                    <div class="pb-3 sm:pb-0">
                        <span class="block text-sm font-medium text-neutral-500 dark:text-neutral-400">
                            "Showing "
                            <span class="dark:text-white/90">(showing_from)</span>
                            " to "
                            <span class="dark:text-white/90">(showing_to)</span>
                            " of "
                            <span class="dark:text-white/90">(count)</span>
                        </span>
                    </div>
                    <div class="flex w-full items-center justify-between gap-2 rounded-lg bg-neutral-50 p-4 sm:w-auto sm:justify-normal sm:rounded-none sm:bg-transparent sm:p-0 dark:bg-neutral-900 dark:sm:bg-transparent">
                        <a
                            x-bind:href=(if *page > 1 { let p = *page - 1; format!("true ? '/admin/customers?page={p}&limit={limit}&filter_by={}' : ''", "") } else { "false".to_string() })
                            class="
                                shadow-xs flex items-center gap-2 rounded-lg border p-2 text-neutral-750 sm:p-2.5 border-neutral-300 bg-white hover:bg-neutral-50
                                dark:border-neutral-750 dark:bg-neutral-800 dark:text-neutral-400 dark:hover:bg-white/3 dark:hover:text-neutral-200
                            "
                            x-bind:class=(if *page == 1 { "true ? 'cursor-not-allowed opacity-50 pointer-events-none' : ''" } else { "false" })
                            role="button">
                            <svg class="fill-current" "width"="20" "height"="20" "viewBox"="0 0 20 20" "fill"="none">
                                <path "fill-rule"="evenodd" "clip-rule"="evenodd" "d"="M2.58203 9.99868C2.58174 10.1909 2.6549 10.3833 2.80152 10.53L7.79818 15.5301C8.09097 15.8231 8.56584 15.8233 8.85883 15.5305C9.15183 15.2377 9.152 14.7629 8.85921 14.4699L5.13911 10.7472L16.6665 10.7472C17.0807 10.7472 17.4165 10.4114 17.4165 9.99715C17.4165 9.58294 17.0807 9.24715 16.6665 9.24715L5.14456 9.24715L8.85919 5.53016C9.15199 5.23717 9.15184 4.7623 8.85885 4.4695C8.56587 4.1767 8.09099 4.17685 7.79819 4.46984L2.84069 9.43049C2.68224 9.568 2.58203 9.77087 2.58203 9.99715C2.58203 9.99766 2.58203 9.99817 2.58203 9.99868Z"></path>
                            </svg>
                        </a>

                        <span class="block text-sm font-medium text-neutral-750 sm:hidden dark:text-neutral-400">
                            "Page "
                            <span>(page)</span>
                            " of "
                            <span>(total_pages)</span>
                        </span>

                        <ul class="hidden items-center gap-0.5 sm:flex">
                            @for i in 1..=total_pages {
                            <li>
                                @if i == *page {
                                    <a class="cursor-default flex h-10 w-10 items-center justify-center rounded-lg text-sm font-medium bg-indigo-500 text-white">
                                        <span>(i)</span>
                                    </a>
                                } @else {
                                    <a href=(format!("/admin/customers?page={i}&limit={limit}&filter_by={}", "")) class="flex h-10 w-10 items-center justify-center rounded-lg text-sm font-medium hover:bg-indigo-500 text-neutral-750 dark:text-neutral-400 hover:text-white dark:hover:text-white">
                                        <span>(i)</span>
                                    </a>
                                }
                            </li>
                            }
                        </ul>

                        <a
                            x-bind:href=(if *page < total_pages { let p = *page + 1; format!("true ? '/admin/customers?page={p}&limit={limit}&filter_by={}' : ''", "") } else { "false".to_string() })
                            class="
                                shadow-xs flex items-center gap-2 rounded-lg border border-neutral-300 bg-white p-2 text-neutral-750 hover:bg-neutral-50 sm:p-2.5
                                dark:border-neutral-750 dark:bg-neutral-800 dark:text-neutral-400 dark:hover:bg-white/3 dark:hover:text-neutral-200
                            "
                            x-bind:class=(if *page == total_pages { "true ? 'cursor-not-allowed opacity-50' : ''" } else { "false" })
                            role="button">
                            <svg class="fill-current" "width"="20" "height"="20" "viewBox"="0 0 20 20" "fill"="none">
                                <path "fill-rule"="evenodd" "clip-rule"="evenodd" "d"="M17.4165 9.9986C17.4168 10.1909 17.3437 10.3832 17.197 10.53L12.2004 15.5301C11.9076 15.8231 11.4327 15.8233 11.1397 15.5305C10.8467 15.2377 10.8465 14.7629 11.1393 14.4699L14.8594 10.7472L3.33203 10.7472C2.91782 10.7472 2.58203 10.4114 2.58203 9.99715C2.58203 9.58294 2.91782 9.24715 3.33203 9.24715L14.854 9.24715L11.1393 5.53016C10.8465 5.23717 10.8467 4.7623 11.1397 4.4695C11.4327 4.1767 11.9075 4.17685 12.2003 4.46984L17.1578 9.43049C17.3163 9.568 17.4165 9.77087 17.4165 9.99715C17.4165 9.99763 17.4165 9.99812 17.4165 9.9986Z"></path>
                            </svg>
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
