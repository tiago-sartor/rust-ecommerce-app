use crate::utils::context::Context;
use crate::utils::hypertext_elements;
use hypertext::prelude::*;
use std::collections::HashMap;

pub struct FormField {
    pub id: &'static str,
    pub label: &'static str,
    pub input_type: &'static str,
    pub required: bool,
    pub placeholder: Option<&'static str>,
    pub autocomplete: Option<&'static str>,
    pub mask: Option<&'static str>,
    pub options: Option<HashMap<&'static str, &'static str>>,
}

impl FormField {
    pub fn render<P: serde::Serialize, D>(&self, ctx: &Context<P, D>, field_classes: &str) -> impl Renderable {
        let payload = crate::utils::helpers::struct_to_map(&ctx.payload.0);
        let field_value = payload.get(self.id).cloned().unwrap_or_default();

        rsx! {
            <div>
                <label for=(self.id) class="mb-1.5 block text-sm font-medium text-neutral-700">
                    (self.label) @if self.required { <span class="text-red-600">" *"</span> }
                </label>
                @if self.input_type == "select" {
                    <div class="relative">
                        <select
                            "x-model.fill"=(self.id)
                            id=(self.id)
                            name=(self.id)
                            class=(format!("{field_classes} appearance-none cursor-pointer"))
                            autocomplete=(self.autocomplete)
                            x-bind:required=(self.required)
                        >
                        @if let Some(options) = &self.options {
                            <option value="">"Select an option"</option>
                            @for (value, label) in options {
                                <option value=(*value) selected=(*value == field_value)>(*label)</option>
                            }
                        }
                        </select>
                        <svg class="pointer-events-none absolute top-1/2 right-3 size-4.25 -translate-y-1/2" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5"></path>
                        </svg>
                    </div>
                } @else {
                   <input
                        "x-model.fill"=(self.id)
                        x-effect=(format!("{}; {}", self.id, "$dispatch('run_validation')"))
                        id=(self.id)
                        name=(self.id)
                        type=(self.input_type)
                        value=(field_value)
                        class=(field_classes)
                        placeholder=(self.placeholder)
                        autocomplete=(self.autocomplete)
                        x-bind:required=(self.required)
                        x-mask=(self.mask)
                    />
                }
                @if let Some(err) = ctx.errors.get(self.id) {
                    <p class="mt-1 text-xs text-red-600">(err)</p>
                }
            </div>
        }
    }
}
