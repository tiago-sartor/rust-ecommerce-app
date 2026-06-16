use crate::models::form_field::{FormField, map_to_options};
use crate::server::handlers::backend::AddCustomerPayload;
use crate::utils::{BrazilianStates, Context, hypertext_elements};
use hypertext::prelude::*;

pub fn admin_add_customer_template(ctx: &Context<AddCustomerPayload, ()>) -> impl Renderable {
    let id_fields = [
        FormField {
            id: "first_name",
            label: "First Name",
            input_type: "text",
            capitalize: true,
            required: true,
            ..Default::default()
        },
        FormField {
            id: "last_name",
            label: "Last Name",
            input_type: "text",
            capitalize: true,
            required: true,
            ..Default::default()
        },
        FormField {
            id: "cpf_cnpj",
            label: "CPF / CNPJ",
            input_type: "text",
            uppercase: true,
            required: true,
            mask: "$el.value.length < 15 ? '***.***.***-**' : '**.***.***/****-99'",
            ..Default::default()
        },
        FormField {
            id: "state_registration",
            label: "State Registration",
            input_type: "text",
            uppercase: true,
            required: false,
            placeholder: Some("Número ou isento"),
            nullable: true,
            null_label: "Isento",
            null_placeholder: "Isento",
            ..Default::default()
        },
        FormField {
            id: "company_name",
            label: "Company Name",
            input_type: "text",
            uppercase: true,
            required: false,
            ..Default::default()
        },
        FormField {
            id: "email",
            label: "E-mail",
            input_type: "email",
            lowercase: true,
            required: true,
            ..Default::default()
        },
        FormField {
            id: "phone",
            label: "Phone",
            input_type: "tel",
            required: true,
            placeholder: Some("(00) 00000-0000"),
            mask: "$el.value.length > 14 ? '(99) 99999-9999' : '(99) 9999-9999'",
            ..Default::default()
        },
    ];

    let address_fields = [
        FormField {
            id: "postcode",
            label: "CEP",
            input_type: "tel",
            required: true,
            placeholder: Some("00000-000"),
            mask: "'99999-999'",
            ..Default::default()
        },
        FormField {
            id: "street",
            label: "Street",
            input_type: "text",
            capitalize: true,
            required: true,
            ..Default::default()
        },
        FormField {
            id: "number",
            label: "Number",
            input_type: "tel",
            required: true,
            placeholder: Some("Ex: 123 ou S/N"),
            nullable: true,
            null_label: "Sem número",
            null_placeholder: "S/N",
            ..Default::default()
        },
        FormField {
            id: "complement",
            label: "Complement",
            input_type: "text",
            required: false,
            placeholder: Some("Apartamento, bloco, condomínio, etc."),
            ..Default::default()
        },
        FormField {
            id: "neighborhood",
            label: "Neighborhood",
            input_type: "text",
            capitalize: true,
            required: true,
            ..Default::default()
        },
        FormField {
            id: "city",
            label: "City",
            input_type: "text",
            capitalize: true,
            required: true,
            ..Default::default()
        },
        FormField {
            id: "state",
            label: "State",
            input_type: "select",
            required: true,
            placeholder: Some("Select a state"),
            options: map_to_options(BrazilianStates::all()),
            ..Default::default()
        },
    ];

    let form_field_ids: Vec<&str> = id_fields.iter().chain(address_fields.iter()).map(|f| f.id).collect();
    let form_field_ids_json = serde_json::to_string(&form_field_ids).unwrap_or("[]".to_string());
    let field_classes = "h-11 w-full rounded-lg border bg-transparent px-4 py-2.5 pr-10 text-sm text-neutral-800 shadow-xs placeholder:text-neutral-400 focus:ring-3 focus:outline-hidden";

    rsx! {
    <div class="mx-auto max-w-3xl p-4 md:p-6">
        <h2 class="mb-5 text-lg font-semibold text-neutral-800 lg:mb-7 dark:text-white/90">
            "Add New Customer"
        </h2>

        @if let Some(message) = ctx.flash_msg.get("success") {
            <div class="mb-6 rounded-lg border border-green-600 bg-green-50 p-4 text-sm font-medium text-green-600">
                (message)
            </div>
        }
        @if let Some(error) = ctx.errors.get("internal_error") {
            <div class="mb-6 rounded-lg border border-red-600 bg-red-50 p-4 text-sm font-medium text-red-600">
                (error)
            </div>
        }

        <div>
            <form id="add-new-customer-form" action="/admin/add-customer" method="POST" x-data=(format!("formFieldValidation({form_field_ids_json})"))>
                <input type="hidden" name="csrf_token" value=(ctx.csrf_token.0) />

                <div class="mb-6 rounded-2xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/3">
                    <div class="border-b border-neutral-200 px-5 py-4 dark:border-neutral-800">
                        <h4 class="text-base font-semibold text-neutral-800 dark:text-white/90">
                            "Identification"
                        </h4>
                    </div>
                    <div class="p-5 grid grid-cols-1 gap-x-6 gap-y-5 lg:grid-cols-2">
                        // ID FIELDS
                        @for field in &id_fields {
                            (field.render(ctx, field_classes))
                        }
                    </div>
                </div>

                <div class="mb-6 rounded-2xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/3">
                    <div class="border-b border-neutral-200 px-5 py-4 dark:border-neutral-800">
                        <h4 class="text-base font-semibold text-neutral-800 dark:text-white/90">
                            "Address"
                        </h4>
                    </div>
                    <div x-init="window.addressFields = $data" class="p-5 grid grid-cols-1 gap-x-6 gap-y-5 lg:grid-cols-2">
                        // ADDRESS FIELDS
                        @for field in &address_fields {
                            (field.render(ctx, field_classes))
                        }
                    </div>
                </div>

                <div class="flex justify-end gap-4">
                    <a href="/admin/customers"
                        class="flex items-center justify-center rounded-lg border border-neutral-300 bg-white px-4 py-2.5 text-sm font-medium text-neutral-700 hover:bg-neutral-50">
                        "Cancel"
                    </a>
                    <button type="submit"
                        class="flex items-center justify-center rounded-lg bg-indigo-500 px-4 py-2.5 text-sm font-medium text-white hover:bg-indigo-600">
                        "Create Customer"
                    </button>
                </div>
            </form>
        </div>
    </div>
    }
}
