use crate::utils::{Context, hypertext_elements};
use hypertext::prelude::*;

#[derive(Default)]
pub struct FormField {
    pub id: &'static str,
    pub label: &'static str,
    pub input_type: &'static str,
    pub capitalize: bool, // Make the first letter uppercase when rendering the input value
    pub uppercase: bool,  // Make all letters uppercase when rendering the input value
    pub lowercase: bool,  // Make all letters lowercase when rendering the input value
    pub required: bool,
    pub placeholder: Option<&'static str>,
    pub autocomplete: &'static str,
    pub mask: &'static str,
    pub options: Option<Vec<SelectOption>>,
    pub nullable: bool,                 // Display a checkbox to set an input field to null/empty (must be handled accordingly in the backend)
    pub null_label: &'static str,       // Label to display for the nullable checkbox
    pub null_placeholder: &'static str, // Placeholder to display for the nullable field
}

pub struct SelectOption {
    pub value: &'static str,
    pub label: &'static str,
}

/// A trait that any type can implement to be rendered as a select option                                                                                                                                                      
pub trait IntoSelectOption {
    fn to_value(&self) -> &'static str;
    fn to_label(&self) -> &'static str;
}

/// Helper function to convert any slice of items implementing `IntoSelectOption` into options                                                                                                                                 
pub fn map_to_options<T: IntoSelectOption>(items: &[T]) -> Option<Vec<SelectOption>> {
    Some(
        items
            .iter()
            .map(|item| SelectOption {
                value: item.to_value(),
                label: item.to_label(),
            })
            .collect(),
    )
}

impl FormField {
    pub fn render<P: serde::Serialize, D>(&self, ctx: &Context<P, D>, field_classes: &str) -> impl Renderable {
        let payload = crate::utils::helpers::struct_to_map(&ctx.payload.0);
        let field_value = payload.get(self.id).cloned().unwrap_or_default();

        let is_valid_classes = "border-green-500 focus:border-green-500 focus:ring-green-500/10";
        let error_classes = "border-red-500 focus:border-red-500 focus:ring-red-500/10";

        let nullable_checkbox = Box::new(rsx! {
            <div class="flex items-center gap-2">
                <div class="group flex shrink-0 items-center justify-center">
                    <input
                        "x-on:change"=(format!("if ($el.checked) {{ {0} = '{1}' }} else {{ {0} = '' }}", self.id, self.null_placeholder))
                        x-bind:checked=(format!("{} === '{}'", self.id, self.null_placeholder))
                        id=(format!("nullable_{}", self.id))
                        type="checkbox"
                        class="size-3 cursor-pointer appearance-none rounded-[3px] border border-gray-500 bg-white checked:border-indigo-600 checked:bg-indigo-600 focus-visible:outline-2 focus-visible:outline-offset-1 focus-visible:outline-indigo-600 disabled:border-gray-300 disabled:bg-gray-100 disabled:checked:bg-gray-100 forced-colors:appearance-auto"
                    />
                    <svg viewBox="0 0 14 14" fill="none" class="pointer-events-none absolute size-2.5 self-center justify-self-center stroke-white group-has-disabled:stroke-gray-950/25">
                        <path d="M3 8L6 11L11 3.5" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="opacity-0 group-has-checked:opacity-100"></path>
                    </svg>
                </div>
                <label for=(format!("nullable_{}", self.id)) class="text-xs cursor-pointer text-neutral-700">(self.null_label)</label>
            </div>
        });

        rsx! {
            <div>
                <div class="mb-1.5 flex items-center justify-between">
                    <label for=(self.id) class="block text-sm font-medium text-neutral-700">
                        (self.label) @if self.required { <span class="text-red-600">" *"</span> }
                    </label>
                    @if self.nullable && self.input_type != "select" { (nullable_checkbox) }
                </div>
                <div class="relative">
                    @if self.input_type == "select" {
                        <select
                            "x-model.fill"=(self.id)
                            id=(self.id)
                            name=(self.id)
                            class=(format!("{field_classes} cursor-pointer appearance-none"))
                            x-bind:class=(format!("
                                isValid.{0} ? '{1}' : errors.{0} ? '{2}' : 'border-neutral-300 focus:border-neutral-300 focus:ring-neutral-500/10'
                            ", self.id, is_valid_classes, error_classes))
                            autocomplete=(self.autocomplete)
                            required[self.required]
                        >
                            @if let Some(options) = &self.options {
                                <option value="">(self.placeholder.unwrap_or("Select an option"))</option>
                                @for option in options {
                                    <option value=(option.value) selected[option.value == field_value]>(option.label)</option>
                                }
                            }
                        </select>
                        <svg class="pointer-events-none absolute top-1/2 right-3 size-4.25 -translate-y-1/2" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5"></path>
                        </svg>
                    } @else {
                        <input
                            "x-model.fill.debounce.900ms"=(self.id)
                            "x-mask:dynamic"=(self.mask)
                            "x-bind:readonly"=((self.nullable && self.input_type != "select").then(|| format!("{} === '{}'", self.id, self.null_placeholder)))
                            x-effect=(format!("{}; $dispatch('run_validation')", self.id))
                            x-on:run_validation=(format!("validateField('{}', '{}')", self.id, self.label))
                            x-on:blur=(if self.capitalize {
                                "$el.value = capitalizeWords($el.value)"
                            } else if self.uppercase {
                                "$el.value = $el.value.toUpperCase()"
                            } else if self.lowercase {
                                "$el.value = $el.value.toLowerCase()"
                            } else {
                                ""
                            })
                            id=(self.id)
                            name=(self.id)
                            type=(self.input_type)
                            value=(field_value)
                            class=(field_classes)
                            x-bind:class=(format!("
                                isValid.{0} ? '{1}' : errors.{0} ? '{2}' : 'border-neutral-300 focus:border-neutral-300 focus:ring-neutral-500/10'
                            ", self.id, is_valid_classes, error_classes))
                            placeholder=(self.placeholder)
                            autocomplete=(self.autocomplete)
                            required[self.required]
                        />
                        <span class="absolute top-1/2 right-3 -translate-y-1/2" aria-hidden="true">
                            <svg x-show=(format!("isValid.{}", self.id)) class="size-4.25 text-green-500" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="2.25" "stroke"="currentColor">
                                <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"></path>
                            </svg>
                            <svg x-show=(format!("errors.{}", self.id)) class="size-4.25 text-red-500" "fill"="none" "viewBox"="0 0 24 24" "stroke-width"="2.25" "stroke"="currentColor">
                                <path "stroke-linecap"="round" "stroke-linejoin"="round" "d"="M12 9v3.75m9-.75a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 3.75h.008v.008H12v-.008Z"></path>
                            </svg>
                        </span>
                    }
                </div>
                <p class="mt-1 text-xs text-red-600" x-show=(format!("errors.{}", self.id)) x-text=(format!("errors.{}", self.id)) x-bind:aria-hidden=(format!("errors.{}", self.id)) style="display: none;"></p>
                @if let Some(err) = ctx.errors.get(self.id) {
                    <p class="mt-1 text-xs text-red-600">(err)</p>
                }
            </div>
        }
    }
}
