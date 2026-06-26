use crate::{models::Order, utils::{Context, hypertext_elements}};
use hypertext::prelude::*;

pub fn admin_customer_details_template(ctx: &Context<(), (Vec<Order>, i64)>) -> impl Renderable {
    let customer = ctx.customer.as_ref().unwrap();
    let address = ctx.address.as_ref().unwrap();

    rsx! {
        <div class="mx-auto max-w-(--breakpoint-2xl) p-4 pb-20 md:p-6 md:pb-6">
            <div class="space-y-6">
                <div class="grid auto-rows-min gap-6 lg:grid-cols-6">

                    <div class="min-h-[25dvh] rounded-xl border border-neutral-200 bg-white lg:col-span-4 dark:border-neutral-800 dark:bg-white/3">
                        <div class="px-6 py-5">
                            <h3 class="text-base font-medium dark:text-white/90">
                                "Customer Info"
                            </h3>
                        </div>
                        <ul class="divide-y divide-neutral-100 border-t border-neutral-200 dark:divide-neutral-800 dark:border-neutral-800">
                            <li class="flex items-start justify-start gap-6 px-6 py-4">
                                <span class="w-1/3 text-sm text-neutral-500 dark:text-neutral-400">"Nome"</span>
                                <span class="w-2/3 text-sm font-medium dark:text-neutral-400">(customer.first_name)" "(customer.last_name)</span>
                            </li>
                            <li class="flex items-start justify-start gap-6 px-6 py-4">
                                <span class="w-1/3 text-sm wrap-anywhere text-neutral-500 dark:text-neutral-400">"E-mail"</span>
                                <span class="w-2/3 text-sm font-medium dark:text-neutral-400">(customer.email)</span>
                            </li>
                            <li class="flex items-start justify-start gap-6 px-6 py-4">
                                <span class="w-1/3 text-sm text-neutral-500 dark:text-neutral-400">"Telefone"</span>
                                <span class="w-2/3 text-sm font-medium dark:text-neutral-400">(customer.phone)</span>
                            </li>
                            @if customer.cpf.is_some() {
                                <li class="flex items-start justify-start gap-6 px-6 py-4">
                                    <span class="w-1/3 text-sm text-neutral-500 dark:text-neutral-400">"CPF"</span>
                                    <span class="w-2/3 text-sm font-medium dark:text-neutral-400">(customer.cpf)</span>
                                </li>
                            }
                            @if customer.cnpj.is_some() {
                                <li class="flex items-start justify-start gap-6 px-6 py-4">
                                    <span class="w-1/3 text-sm text-neutral-500 dark:text-neutral-400">"CNPJ"</span>
                                    <span class="w-2/3 text-sm font-medium dark:text-neutral-400">(customer.cnpj)</span>
                                </li>
                                <li class="flex items-start justify-start gap-6 px-6 py-4">
                                    <span class="w-1/3 text-sm text-neutral-500 dark:text-neutral-400">"Razão Social"</span>
                                    <span class="w-2/3 text-sm font-medium dark:text-neutral-400">(customer.company_name)</span>
                                </li>
                                <li class="flex items-start justify-start gap-6 px-6 py-4">
                                    <span class="w-1/3 text-sm text-neutral-500 dark:text-neutral-400">"Inscrição Estadual"</span>
                                    <span class="w-2/3 text-sm font-medium dark:text-neutral-400">(customer.state_registration)</span>
                                </li>
                            }
                        </ul>
                        <div x-data="{isModalOpen: false}" class="mt-2 px-6 py-5">
                            <button @click="isModalOpen = !isModalOpen" type="button" class="shadow-xs flex w-full justify-center gap-2 rounded-lg border border-neutral-300 bg-white px-4 py-3 text-sm font-medium text-neutral-750 hover:bg-neutral-50 dark:border-neutral-750 dark:bg-neutral-800 dark:text-neutral-400 dark:hover:bg-white/3 dark:hover:text-neutral-200">
                                <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
                                    <path d="M12.8861 5.08135L15.4182 7.61345M16.1437 3.59219L16.908 4.35652C17.3962 4.84468 17.3962 5.63613 16.908 6.12429L8.33547 14.6968C8.19039 14.8419 8.01182 14.9491 7.81554 15.0088L4.47461 16.0256L5.49141 12.6847C5.55115 12.4884 5.65829 12.3098 5.80337 12.1647L14.3759 3.59219C14.8641 3.10404 15.6555 3.10404 16.1437 3.59219Z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                                </svg>
                                "Update Customer Info"
                            </button>
                            <div x-show="isModalOpen" class="modal fixed inset-0 z-99999 flex items-center justify-center overflow-y-auto p-5" style="display: none;">
                            <div class="modal-close-btn fixed inset-0 h-full w-full bg-neutral-400/50 backdrop-blur-[32px]"></div>
                            <div @click.outside="isModalOpen = false" class="relative w-full max-w-[558px] rounded-3xl bg-white p-6 lg:p-10 dark:bg-neutral-900">
                                // Close btn
                                <button @click="isModalOpen = false" class="absolute top-3 right-3 z-999 flex h-9.5 w-9.5 items-center justify-center rounded-full bg-neutral-100 text-neutral-400 transition-colors hover:bg-neutral-200 hover:text-neutral-750 sm:top-6 sm:right-6 sm:h-11 sm:w-11 dark:bg-neutral-800 dark:text-neutral-400 dark:hover:bg-neutral-750 dark:hover:text-white">
                                    <svg class="fill-current" width="24" height="24" viewBox="0 0 24 24" fill="none">
                                        <path fill-rule="evenodd" clip-rule="evenodd" d="M6.04289 16.5413C5.65237 16.9318 5.65237 17.565 6.04289 17.9555C6.43342 18.346 7.06658 18.346 7.45711 17.9555L11.9987 13.4139L16.5408 17.956C16.9313 18.3466 17.5645 18.3466 17.955 17.956C18.3455 17.5655 18.3455 16.9323 17.955 16.5418L13.4129 11.9997L17.955 7.4576C18.3455 7.06707 18.3455 6.43391 17.955 6.04338C17.5645 5.65286 16.9313 5.65286 16.5408 6.04338L11.9987 10.5855L7.45711 6.0439C7.06658 5.65338 6.43342 5.65338 6.04289 6.0439C5.65237 6.43442 5.65237 7.06759 6.04289 7.45811L10.5845 11.9997L6.04289 16.5413Z" fill=""></path>
                                    </svg>
                                </button>
                                <div>
                                    <h4 class="text-title-xs mb-1 font-semibold dark:text-white/90">
                                        "New integration"
                                    </h4>
                                    <p class="mb-7 text-sm leading-6 text-neutral-500 dark:text-neutral-400">
                                        "Set up an integration and add a brief explanation for the team."
                                    </p>
                                    <div class="custom-scrollbar h-[490px] overflow-y-auto sm:h-auto">
                                        <form action="#">
                                            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
                                                <div>
                                                    <label class="mb-1.5 block text-sm font-medium text-neutral-750 dark:text-neutral-400">
                                                        "First Name"
                                                    </label>
                                                    <input type="text" value="Mushafrof" class="dark:bg-dark-900 shadow-xs focus:border-indigo-300 focus:ring-indigo-500/10 dark:focus:border-indigo-800 h-11 w-full rounded-lg border border-neutral-300 bg-transparent px-4 py-2.5 text-sm placeholder:text-neutral-400 focus:ring-3 focus:outline-hidden dark:border-neutral-750 dark:bg-neutral-900 dark:text-white/90 dark:placeholder:text-white/30">
                                                </div>
                                                <div>
                                                    <label class="mb-1.5 block text-sm font-medium text-neutral-750 dark:text-neutral-400">
                                                        "Last Name"
                                                    </label>
                                                    <input type="text" value="Chowdhury" class="dark:bg-dark-900 shadow-xs focus:border-indigo-300 focus:ring-indigo-500/10 dark:focus:border-indigo-800 h-11 w-full rounded-lg border border-neutral-300 bg-transparent px-4 py-2.5 text-sm placeholder:text-neutral-400 focus:ring-3 focus:outline-hidden dark:border-neutral-750 dark:bg-neutral-900 dark:text-white/90 dark:placeholder:text-white/30">
                                                </div>
                                                <div class="sm:col-span-full">
                                                    <label class="mb-1.5 block text-sm font-medium text-neutral-750 dark:text-neutral-400">
                                                        "Street"
                                                    </label>
                                                    <input type="text" value="800 E Elcamino Real, suite #400" class="dark:bg-dark-900 shadow-xs focus:border-indigo-300 focus:ring-indigo-500/10 dark:focus:border-indigo-800 h-11 w-full rounded-lg border border-neutral-300 bg-transparent px-4 py-2.5 text-sm placeholder:text-neutral-400 focus:ring-3 focus:outline-hidden dark:border-neutral-750 dark:bg-neutral-900 dark:text-white/90 dark:placeholder:text-white/30">
                                                </div>
                                                <div class="sm:col-span-1">
                                                    <label class="mb-1.5 block text-sm font-medium text-neutral-750 dark:text-neutral-400">
                                                        "Town/City"
                                                    </label>
                                                    <div x-data="{ isOptionSelected: false }" class="relative z-20 bg-transparent">
                                                        <select class="dark:bg-dark-900 shadow-xs focus:border-indigo-300 focus:ring-indigo-500/10 dark:focus:border-indigo-800 h-11 w-full appearance-none rounded-lg border border-neutral-300 bg-transparent bg-none px-4 py-2.5 pr-11 text-sm placeholder:text-neutral-400 focus:ring-3 focus:outline-hidden dark:border-neutral-750 dark:bg-neutral-900 dark:text-white/90 dark:placeholder:text-white/30" :class="isOptionSelected &amp;&amp; 'text-neutral-750 dark:text-white/90'" @change="isOptionSelected = true">
                                                            <option value="" class="text-neutral-750 dark:bg-neutral-900 dark:text-neutral-400">
                                                                "Select Option"
                                                            </option>
                                                            <option value="" class="text-neutral-750 dark:bg-neutral-900 dark:text-neutral-400">
                                                                "New York"
                                                            </option>
                                                            <option value="" class="text-neutral-750 dark:bg-neutral-900 dark:text-neutral-400">
                                                                "Tokyo"
                                                            </option>
                                                            <option value="" class="text-neutral-750 dark:bg-neutral-900 dark:text-neutral-400">
                                                                "Chicago"
                                                            </option>
                                                            <option value="" class="text-neutral-750 dark:bg-neutral-900 dark:text-neutral-400">
                                                                "Los Angeles"
                                                            </option>
                                                            <option value="" class="text-neutral-750 dark:bg-neutral-900 dark:text-neutral-400">
                                                                "Berlin"
                                                            </option>
                                                        </select>
                                                        <span class="pointer-events-none absolute top-1/2 right-4 z-30 -translate-y-1/2 text-neutral-500 dark:text-neutral-400">
                                                            <svg class="stroke-current" width="20" height="20" viewBox="0 0 20 20" fill="none">
                                                                <path d="M4.79175 7.396L10.0001 12.6043L15.2084 7.396" stroke="" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                                                            </svg>
                                                        </span>
                                                    </div>
                                                </div>
                                                <div>
                                                    <label class="mb-1.5 block text-sm font-medium text-neutral-750 dark:text-neutral-400">
                                                        "Postcode"
                                                    </label>
                                                    <input type="text" value="19029" class="dark:bg-dark-900 shadow-xs focus:border-indigo-300 focus:ring-indigo-500/10 dark:focus:border-indigo-800 h-11 w-full rounded-lg border border-neutral-300 bg-transparent px-4 py-2.5 text-sm placeholder:text-neutral-400 focus:ring-3 focus:outline-hidden dark:border-neutral-750 dark:bg-neutral-900 dark:text-white/90 dark:placeholder:text-white/30">
                                                </div>
                                            </div>
                                            <p class="mt-4 text-sm text-neutral-500 dark:text-neutral-400">
                                                "Click “Update Info” to update your billing information."
                                            </p>
                                        </form>
                                    </div>
                                <div class="mt-8 flex items-center justify-end gap-3">
                                    <button @click="isModalOpen = false" type="button" class="shadow-xs flex justify-center rounded-lg border border-neutral-300 bg-white px-4 py-3 text-sm font-medium text-neutral-750 hover:bg-neutral-50 dark:border-neutral-750 dark:bg-neutral-800 dark:text-neutral-400 dark:hover:bg-white/[0.03] dark:hover:text-neutral-200">
                                        "Cancel"
                                    </button>
                                    <button @click="isModalOpen = false" type="button" class="bg-indigo-500 shadow-xs hover:bg-indigo-600 flex justify-center rounded-lg px-4 py-3 text-sm font-medium text-white">
                                        "Update"
                                    </button>
                                </div>
                                </div>
                            </div>
                            </div>
                        </div>
                    </div>

                    <div class="min-h-[25dvh] rounded-xl border border-neutral-200 bg-white lg:col-span-2 dark:border-neutral-800 dark:bg-white/3">
                        <div class="px-6 py-5">
                            <h3 class="text-base font-medium dark:text-white/90">
                                "Billing Address"
                            </h3>
                        </div>
                        <ul class="divide-y divide-neutral-100 border-t border-neutral-200 dark:divide-neutral-800 dark:border-neutral-800">
                            <li class="flex items-start justify-start gap-6 px-6 py-4">
                                <span class="w-1/3 text-sm text-neutral-500 dark:text-neutral-400">"Endereço"</span>
                                <span class="w-2/3 text-sm font-medium dark:text-neutral-400">(address.street)", "(address.number)</span>
                            </li>
                            <li class="flex items-start justify-start gap-6 px-6 py-4">
                                <span class="w-1/3 text-sm text-neutral-500 dark:text-neutral-400">"Complemento"</span>
                                <span class="w-2/3 text-sm font-medium dark:text-neutral-400">(address.complement)</span>
                            </li>
                            <li class="flex items-start justify-start gap-6 px-6 py-4">
                                <span class="w-1/3 text-sm text-neutral-500 dark:text-neutral-400">"Bairro"</span>
                                <span class="w-2/3 text-sm font-medium dark:text-neutral-400">(address.neighborhood)</span>
                            </li>
                            <li class="flex items-start justify-start gap-6 px-6 py-4">
                                <span class="w-1/3 text-sm text-neutral-500 dark:text-neutral-400">"Cidade / Estado"</span>
                                <span class="w-2/3 text-sm font-medium dark:text-neutral-400">(address.city)" / "(address.state.unwrap().symbol())</span>
                            </li>
                            <li class="flex items-start justify-start gap-6 px-6 py-4">
                                <span class="w-1/3 text-sm text-neutral-500 dark:text-neutral-400">"CEP"</span>
                                <span class="w-2/3 text-sm font-medium dark:text-neutral-400">(address.postcode)</span>
                            </li>
                        </ul>
                        <div x-data="{isModalOpen: false}" class="mt-2 px-6 py-5">
                            <button @click="isModalOpen = !isModalOpen" type="button" class="shadow-xs flex w-full justify-center gap-2 rounded-lg border border-neutral-300 bg-white px-4 py-3 text-sm font-medium text-neutral-750 hover:bg-neutral-50 dark:border-neutral-750 dark:bg-neutral-800 dark:text-neutral-400 dark:hover:bg-white/3 dark:hover:text-neutral-200">
                                <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
                                    <path d="M12.8861 5.08135L15.4182 7.61345M16.1437 3.59219L16.908 4.35652C17.3962 4.84468 17.3962 5.63613 16.908 6.12429L8.33547 14.6968C8.19039 14.8419 8.01182 14.9491 7.81554 15.0088L4.47461 16.0256L5.49141 12.6847C5.55115 12.4884 5.65829 12.3098 5.80337 12.1647L14.3759 3.59219C14.8641 3.10404 15.6555 3.10404 16.1437 3.59219Z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                                </svg>
                                "Update Address"
                            </button>
                            <div x-show="isModalOpen" class="modal fixed inset-0 z-99999 flex items-center justify-center overflow-y-auto p-5" style="display: none;">
                            <div class="modal-close-btn fixed inset-0 h-full w-full bg-neutral-400/50 backdrop-blur-[32px]"></div>
                            <div @click.outside="isModalOpen = false" class="relative w-full max-w-[558px] rounded-3xl bg-white p-6 lg:p-10 dark:bg-neutral-900">
                                // Close btn
                                <button @click="isModalOpen = false" class="absolute top-3 right-3 z-999 flex h-9.5 w-9.5 items-center justify-center rounded-full bg-neutral-100 text-neutral-400 transition-colors hover:bg-neutral-200 hover:text-neutral-750 sm:top-6 sm:right-6 sm:h-11 sm:w-11 dark:bg-neutral-800 dark:text-neutral-400 dark:hover:bg-neutral-750 dark:hover:text-white">
                                    <svg class="fill-current" width="24" height="24" viewBox="0 0 24 24" fill="none">
                                        <path fill-rule="evenodd" clip-rule="evenodd" d="M6.04289 16.5413C5.65237 16.9318 5.65237 17.565 6.04289 17.9555C6.43342 18.346 7.06658 18.346 7.45711 17.9555L11.9987 13.4139L16.5408 17.956C16.9313 18.3466 17.5645 18.3466 17.955 17.956C18.3455 17.5655 18.3455 16.9323 17.955 16.5418L13.4129 11.9997L17.955 7.4576C18.3455 7.06707 18.3455 6.43391 17.955 6.04338C17.5645 5.65286 16.9313 5.65286 16.5408 6.04338L11.9987 10.5855L7.45711 6.0439C7.06658 5.65338 6.43342 5.65338 6.04289 6.0439C5.65237 6.43442 5.65237 7.06759 6.04289 7.45811L10.5845 11.9997L6.04289 16.5413Z" fill=""></path>
                                    </svg>
                                </button>
                                <div>
                                    <h4 class="text-title-xs mb-1 font-semibold dark:text-white/90">
                                        "New integration"
                                    </h4>
                                    <p class="mb-7 text-sm leading-6 text-neutral-500 dark:text-neutral-400">
                                        "Set up an integration and add a brief explanation for the team."
                                    </p>
                                    <div class="custom-scrollbar h-[490px] overflow-y-auto sm:h-auto">
                                        <form action="#">
                                            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
                                                <div>
                                                    <label class="mb-1.5 block text-sm font-medium text-neutral-750 dark:text-neutral-400">
                                                        "First Name"
                                                    </label>
                                                    <input type="text" value="Mushafrof" class="dark:bg-dark-900 shadow-xs focus:border-indigo-300 focus:ring-indigo-500/10 dark:focus:border-indigo-800 h-11 w-full rounded-lg border border-neutral-300 bg-transparent px-4 py-2.5 text-sm placeholder:text-neutral-400 focus:ring-3 focus:outline-hidden dark:border-neutral-750 dark:bg-neutral-900 dark:text-white/90 dark:placeholder:text-white/30">
                                                </div>
                                                <div>
                                                    <label class="mb-1.5 block text-sm font-medium text-neutral-750 dark:text-neutral-400">
                                                        "Last Name"
                                                    </label>
                                                    <input type="text" value="Chowdhury" class="dark:bg-dark-900 shadow-xs focus:border-indigo-300 focus:ring-indigo-500/10 dark:focus:border-indigo-800 h-11 w-full rounded-lg border border-neutral-300 bg-transparent px-4 py-2.5 text-sm placeholder:text-neutral-400 focus:ring-3 focus:outline-hidden dark:border-neutral-750 dark:bg-neutral-900 dark:text-white/90 dark:placeholder:text-white/30">
                                                </div>
                                                <div class="sm:col-span-full">
                                                    <label class="mb-1.5 block text-sm font-medium text-neutral-750 dark:text-neutral-400">
                                                        "Street"
                                                    </label>
                                                    <input type="text" value="800 E Elcamino Real, suite #400" class="dark:bg-dark-900 shadow-xs focus:border-indigo-300 focus:ring-indigo-500/10 dark:focus:border-indigo-800 h-11 w-full rounded-lg border border-neutral-300 bg-transparent px-4 py-2.5 text-sm placeholder:text-neutral-400 focus:ring-3 focus:outline-hidden dark:border-neutral-750 dark:bg-neutral-900 dark:text-white/90 dark:placeholder:text-white/30">
                                                </div>
                                                <div class="sm:col-span-1">
                                                    <label class="mb-1.5 block text-sm font-medium text-neutral-750 dark:text-neutral-400">
                                                        "Town/City"
                                                    </label>
                                                    <div x-data="{ isOptionSelected: false }" class="relative z-20 bg-transparent">
                                                        <select class="dark:bg-dark-900 shadow-xs focus:border-indigo-300 focus:ring-indigo-500/10 dark:focus:border-indigo-800 h-11 w-full appearance-none rounded-lg border border-neutral-300 bg-transparent bg-none px-4 py-2.5 pr-11 text-sm placeholder:text-neutral-400 focus:ring-3 focus:outline-hidden dark:border-neutral-750 dark:bg-neutral-900 dark:text-white/90 dark:placeholder:text-white/30" :class="isOptionSelected &amp;&amp; 'text-neutral-750 dark:text-white/90'" @change="isOptionSelected = true">
                                                            <option value="" class="text-neutral-750 dark:bg-neutral-900 dark:text-neutral-400">
                                                                "Select Option"
                                                            </option>
                                                            <option value="" class="text-neutral-750 dark:bg-neutral-900 dark:text-neutral-400">
                                                                "New York"
                                                            </option>
                                                            <option value="" class="text-neutral-750 dark:bg-neutral-900 dark:text-neutral-400">
                                                                "Tokyo"
                                                            </option>
                                                            <option value="" class="text-neutral-750 dark:bg-neutral-900 dark:text-neutral-400">
                                                                "Chicago"
                                                            </option>
                                                            <option value="" class="text-neutral-750 dark:bg-neutral-900 dark:text-neutral-400">
                                                                "Los Angeles"
                                                            </option>
                                                            <option value="" class="text-neutral-750 dark:bg-neutral-900 dark:text-neutral-400">
                                                                "Berlin"
                                                            </option>
                                                        </select>
                                                        <span class="pointer-events-none absolute top-1/2 right-4 z-30 -translate-y-1/2 text-neutral-500 dark:text-neutral-400">
                                                            <svg class="stroke-current" width="20" height="20" viewBox="0 0 20 20" fill="none">
                                                                <path d="M4.79175 7.396L10.0001 12.6043L15.2084 7.396" stroke="" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                                                            </svg>
                                                        </span>
                                                    </div>
                                                </div>
                                                <div>
                                                    <label class="mb-1.5 block text-sm font-medium text-neutral-750 dark:text-neutral-400">
                                                        "Postcode"
                                                    </label>
                                                    <input type="text" value="19029" class="dark:bg-dark-900 shadow-xs focus:border-indigo-300 focus:ring-indigo-500/10 dark:focus:border-indigo-800 h-11 w-full rounded-lg border border-neutral-300 bg-transparent px-4 py-2.5 text-sm placeholder:text-neutral-400 focus:ring-3 focus:outline-hidden dark:border-neutral-750 dark:bg-neutral-900 dark:text-white/90 dark:placeholder:text-white/30">
                                                </div>
                                            </div>
                                            <p class="mt-4 text-sm text-neutral-500 dark:text-neutral-400">
                                                "Click “Update Info” to update your billing information."
                                            </p>
                                        </form>
                                    </div>
                                <div class="mt-8 flex items-center justify-end gap-3">
                                    <button @click="isModalOpen = false" type="button" class="shadow-xs flex justify-center rounded-lg border border-neutral-300 bg-white px-4 py-3 text-sm font-medium text-neutral-750 hover:bg-neutral-50 dark:border-neutral-750 dark:bg-neutral-800 dark:text-neutral-400 dark:hover:bg-white/[0.03] dark:hover:text-neutral-200">
                                        "Cancel"
                                    </button>
                                    <button @click="isModalOpen = false" type="button" class="bg-indigo-500 shadow-xs hover:bg-indigo-600 flex justify-center rounded-lg px-4 py-3 text-sm font-medium text-white">
                                        "Update"
                                    </button>
                                </div>
                                </div>
                            </div>
                            </div>
                        </div>
                    </div>
                    <div class="min-h-[50dvh] rounded-xl border border-neutral-200 bg-white lg:col-span-full dark:border-neutral-800 dark:bg-white/3"></div>
                </div>
            </div>
        </div>
    }
}
