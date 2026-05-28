use crate::models::form_field::FormField;
use crate::utils::brazilian_states::BrazilianStates;
use crate::utils::context::Context;
use crate::utils::hypertext_elements;
use hypertext::prelude::*;

pub fn admin_add_customer_template(ctx: &Context) -> impl Renderable {
    let id_fields = [
        FormField {
            id: "first_name",
            label: "First Name",
            input_type: "text",
            required: true,
            autocomplete: Some("given-name"),
            placeholder: None,
            mask: None,
            options: None,
        },
        FormField {
            id: "last_name",
            label: "Last Name",
            input_type: "text",
            required: true,
            autocomplete: Some("family-name"),
            placeholder: None,
            mask: None,
            options: None,
        },
        FormField {
            id: "cpf_or_cnpj",
            label: "CPF / CNPJ",
            input_type: "text",
            required: true,
            autocomplete: None,
            placeholder: None,
            mask: None,
            options: None,
        },
        FormField {
            id: "state_registration",
            label: "State Registration",
            input_type: "text",
            required: false,
            autocomplete: None,
            placeholder: None,
            mask: None,
            options: None,
        },
        FormField {
            id: "company_name",
            label: "Company Name",
            input_type: "text",
            required: false,
            autocomplete: None,
            placeholder: None,
            mask: None,
            options: None,
        },
        FormField {
            id: "email",
            label: "E-mail",
            input_type: "email",
            required: true,
            autocomplete: Some("email"),
            placeholder: None,
            mask: None,
            options: None,
        },
        FormField {
            id: "phone",
            label: "Phone",
            input_type: "tel",
            required: true,
            autocomplete: None,
            placeholder: Some("(00) 00000-0000"),
            mask: Some("(99) 99999-9999"),
            options: None,
        },
    ];

    let address_fields = [
        FormField {
            id: "postcode",
            label: "CEP",
            input_type: "tel",
            required: true,
            autocomplete: None,
            placeholder: Some("00000-000"),
            mask: Some("99999-999"),
            options: None,
        },
        FormField {
            id: "street",
            label: "Street",
            input_type: "text",
            required: true,
            autocomplete: None,
            placeholder: None,
            mask: None,
            options: None,
        },
        FormField {
            id: "number",
            label: "Number",
            input_type: "tel",
            required: true,
            autocomplete: None,
            placeholder: None,
            mask: None,
            options: None,
        },
        FormField {
            id: "complement",
            label: "Complement",
            input_type: "text",
            required: false,
            autocomplete: None,
            placeholder: Some("Apartamento, casa, condomínio, etc."),
            mask: None,
            options: None,
        },
        FormField {
            id: "neighborhood",
            label: "Neighborhood",
            input_type: "text",
            required: true,
            autocomplete: None,
            placeholder: None,
            mask: None,
            options: None,
        },
        FormField {
            id: "city",
            label: "City",
            input_type: "text",
            required: true,
            autocomplete: None,
            placeholder: None,
            mask: None,
            options: None,
        },
        FormField {
            id: "state",
            label: "State",
            input_type: "select",
            required: true,
            autocomplete: None,
            placeholder: None,
            mask: None,
            options: None,
        },
    ];

    let form_field_ids: Vec<&str> = id_fields.iter().chain(address_fields.iter()).map(|f| f.id).collect();
    let form_field_ids_json = serde_json::to_string(&form_field_ids).unwrap_or("[]".to_string());
    let field_classes = "h-11 w-full rounded-lg border bg-transparent px-4 py-2.5 pr-10 text-sm text-neutral-800 shadow-xs placeholder:text-neutral-400 focus:ring-3 focus:outline-hidden";

    rsx! {
    <div class="p-4 mx-auto max-w-(--breakpoint-2xl) md:p-6">
        <div class="rounded-2xl border border-neutral-200 bg-white p-5 lg:p-6">
            <h3 class="mb-5 text-lg font-semibold text-neutral-800 lg:mb-7">
                "Add New Customer"
            </h3>

            <form action="/admin/add-customer" method="POST" x-data=(format!("formFieldValidation({form_field_ids_json})"))>
                <input type="hidden" name="csrf_token" value=(ctx.csrf_token.0) />

                <div class="p-5 mb-6 border border-neutral-200 rounded-2xl lg:p-6">
                    <h4 class="text-lg font-semibold text-neutral-800 mb-4 lg:mb-6">
                        "Identification"
                    </h4>

                    <div class="grid grid-cols-1 gap-x-6 gap-y-5 lg:grid-cols-2">
                        // ID FIELDS
                        @for field in &id_fields {
                            (field.render(ctx, field_classes))
                            <div>
                                <label for=(field.id) class="mb-1.5 block text-sm font-medium text-neutral-700">
                                    (field.label) @if field.required { <span class="text-red-600">" *"</span> }
                                </label>
                                <div class="relative">
                                    // @if field.id == "cpf_or_cnpj" {
                                    //     <input
                                    //         "x-model.fill"=(field.id)
                                    //         x-effect=(format!("{} {}", field.id, "; validateCPForCNPJ($el.value)"))
                                    //         id=(field.id)
                                    //         name=(field.id)
                                    //         type=(field.input_type)
                                    //         placeholder=(field.placeholder)
                                    //         "x-mask:dynamic"="$el.value.length < 15 ? '999.999.999-99' : '**.***.***/****-99'"
                                    //         x-bind:required=(field.required)
                                    //         x-bind:class="
                                    //             isValid_cpf_cnpj ? 'border-green-500 focus:border-green-500 focus:ring-green-500/10'
                                    //             : error_cpf_cnpj ? 'border-red-500 focus:border-red-500 focus:ring-red-500/10'
                                    //             : 'border-neutral-300 focus:border-neutral-300 focus:ring-neutral-500/10'
                                    //         "
                                    //         class=(field_classes)
                                    //     />
                                    // } @else {
                                        <input
                                            "x-model.fill"=(field.id)
                                            x-effect=(format!("{}; {}", field.id, "$dispatch('run_validation')"))
                                            x-on:run_validation=(format!("$el.value = formatTextInput($el.value); validateField('{}', '{}');", field.id, field.label))
                                            id=(field.id)
                                            name=(field.id)
                                            type=(field.input_type)
                                            placeholder=(field.placeholder)
                                            x-mask=(field.mask)
                                            x-bind:required=(field.required)
                                            x-bind:class=(format!("
                                                isValid.{0} ? 'border-green-500 focus:border-green-500 focus:ring-green-500/10'
                                                : errors.{0} ? 'border-red-500 focus:border-red-500 focus:ring-red-500/10'
                                                : 'border-neutral-300 focus:border-neutral-300 focus:ring-neutral-500/10'
                                            ", field.id))
                                            class=(field_classes)
                                        />
                                    // }
                                    <span class="absolute top-1/2 right-3 -translate-y-1/2" aria-hidden="true">
                                        <svg x-show=(format!("isValid.{}", field.id)) class="size-4.25 text-green-500" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="2.25" "stroke"="currentColor">
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"></path>
                                        </svg>
                                        <svg x-show=(format!("errors.{}", field.id)) class="size-4.25 text-red-500" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="2.25" "stroke"="currentColor">
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M12 9v3.75m9-.75a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 3.75h.008v.008H12v-.008Z"></path>
                                        </svg>
                                    </span>
                                </div>
                                <p class="mt-1 text-xs text-red-600" x-show=(format!("errors.{}", field.id)) x-text=(format!("errors.{}", field.id)) x-bind:aria-hidden=(format!("errors.{}", field.id)) style="display: none;"></p>
                                @if let Some(err) = ctx.errors.get(field.id) {
                                    <p class="mt-1 text-xs text-red-600">(err)</p>
                                }
                            </div>
                        }
                        // // CPF/CNPJ
                        // <div x-data="CPFandCNPJvalidation()">
                        //     <label for="cpf_or_cnpj" class="mb-1.5 block text-sm font-medium text-neutral-700">
                        //         "CPF / CNPJ" <span class="text-red-600">" *"</span>
                        //     </label>
                        //     <div class="relative">
                        //         <input type="text" id="cpf_or_cnpj" name="cpf_or_cnpj" placeholder="000.000.000-00" required
                        //             x-model="cpf_or_cnpj"
                        //             x-on:input="validateCPForCNPJ($el.value)"
                        //             x-on:blur="validateCPForCNPJ($el.value)"
                        //             "x-mask:dynamic"="$el.value.length < 15 ? '999.999.999-99' : '**.***.***/****-99'"
                        //             x-bind:class="success ? 'border-green-500 focus:border-green-500 focus:ring-green-500/10' : error ? 'border-red-500 focus:border-red-500 focus:ring-red-500/10' : 'border-neutral-300 focus:border-neutral-300 focus:ring-neutral-500/10'"
                        //             class="h-11 w-full rounded-lg border bg-transparent px-4 py-2.5 pr-10 text-sm text-neutral-800 shadow-xs placeholder:text-neutral-400 focus:ring-3 focus:outline-hidden" />
                        //         <span class="absolute top-1/2 right-3 -translate-y-1/2" aria-hidden="true">
                        //             <svg x-show="success" class="size-4.25 text-green-500" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="2.25" "stroke"="currentColor">
                        //                 <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"></path>
                        //             </svg>
                        //             <svg x-show="error" class="size-4.25 text-red-500" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="2.25" "stroke"="currentColor">
                        //                 <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M12 9v3.75m9-.75a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 3.75h.008v.008H12v-.008Z"></path>
                        //             </svg>
                        //         </span>
                        //     </div>
                        //     <p class="mt-1 text-xs text-red-600" x-show="error" x-text="error" x-bind:aria-hidden="error" style="display: none;"></p>
                        //     @if let Some(err) = ctx.errors.get("cpf_or_cnpj") {
                        //         <p class="mt-1 text-xs text-red-600">(err)</p>
                        //     }
                        // </div>
                    </div>
                </div>

                <div class="p-5 mb-6 border border-neutral-200 rounded-2xl lg:p-6">
                    <h4 class="text-lg font-semibold text-neutral-800 mb-4 lg:mb-6">
                        "Address"
                    </h4>

                    <div class="grid grid-cols-1 gap-x-6 gap-y-5 lg:grid-cols-2">
                        // ADDRESS FIELDS
                        @for field in &address_fields {
                            <div>
                                <label for=(field.id) class="mb-1.5 block text-sm font-medium text-neutral-700">
                                    (field.label) @if field.required { <span class="text-red-600">" *"</span> }
                                </label>
                                <div class="relative">
                                    @if field.id == "state" {
                                        <select
                                            id=(field.id)
                                            name=(field.id)
                                            class=(format!("{field_classes} appearance-none cursor-pointer"))
                                            x-bind:required=(field.required)
                                        >
                                            <option value="">"Select a state"</option>
                                            @for state in BrazilianStates::all() {
                                                <option value=(state.symbol())>(state.to_string())</option>
                                            }
                                        </select>
                                        <svg class="pointer-events-none absolute top-1/2 right-3 size-4.25 -translate-y-1/2" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="2" "stroke"="currentColor">
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="m19.5 8.25-7.5 7.5-7.5-7.5"></path>
                                        </svg>
                                    } @else {
                                        <input
                                            "x-model.fill"=(field.id)
                                            x-effect=(format!("{}; {}", field.id, "$dispatch('run_validation')"))
                                            x-on:run_validation=(format!("$el.value = formatTextInput($el.value); validateField('{}', '{}');", field.id, field.label))
                                            id=(field.id)
                                            name=(field.id)
                                            type=(field.input_type)
                                            placeholder=(field.placeholder)
                                            x-mask=(field.mask)
                                            x-bind:required=(field.required)
                                            x-bind:class=(format!("
                                                isValid.{0} ? 'border-green-500 focus:border-green-500 focus:ring-green-500/10'
                                                : errors.{0} ? 'border-red-500 focus:border-red-500 focus:ring-red-500/10'
                                                : 'border-neutral-300 focus:border-neutral-300 focus:ring-neutral-500/10'
                                            ", field.id))
                                            class=(field_classes)
                                        />
                                    }
                                    <span class="absolute top-1/2 right-3 -translate-y-1/2" aria-hidden="true">
                                        <svg x-show=(format!("isValid.{}", field.id)) class="size-4.25 text-green-500" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="2.25" "stroke"="currentColor">
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"></path>
                                        </svg>
                                        <svg x-show=(format!("errors.{}", field.id)) class="size-4.25 text-red-500" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="2.25" "stroke"="currentColor">
                                            <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M12 9v3.75m9-.75a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 3.75h.008v.008H12v-.008Z"></path>
                                        </svg>
                                    </span>
                                </div>
                                <p class="mt-1 text-xs text-red-600" x-show=(format!("errors.{}", field.id)) x-text=(format!("errors.{}", field.id)) x-bind:aria-hidden=(format!("errors.{}", field.id)) style="display: none;"></p>
                                @if let Some(err) = ctx.errors.get(field.id) {
                                    <p class="mt-1 text-xs text-red-600">(err)</p>
                                }
                            </div>
                        }
                    </div>
                </div>

                <div class="flex justify-end gap-4">
                    <a href="/admin/customers"
                        class="flex items-center justify-center rounded-lg border border-neutral-300 bg-white px-4 py-2.5 text-sm font-medium text-neutral-700 hover:bg-neutral-50">
                        "Cancel"
                    </a>
                    <button type="submit"
                        class="flex items-center justify-center rounded-lg bg-neutral-900 px-4 py-2.5 text-sm font-medium text-white hover:bg-neutral-800">
                        "Create Customer"
                    </button>
                </div>
            </form>
        </div>
    </div>
    }
}
