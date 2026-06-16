'use strict';

document.addEventListener('DOMContentLoaded', addressAutocomplete);

function addressAutocomplete() {
    const postcodeField = document.getElementById('postcode');

    if (postcodeField) {
        postcodeField.addEventListener('input', function () {
            const postcode = postcodeField.value;
            if (validate(postcode)) {
                autofillAddress(postcode);
            }
        });
    }

    // Validate the postcode input
    function validate(postcode) {
        const sanitized = postcode.replace(/[^0-9]/g, '');
        return sanitized.length === 8;
    }

    // Set the value of a field
    // This is intended to be used together with the x-model in Alpine.js
    function setFieldValue(key, value) {
        // window.addressFields is initialized with the x-init directive on the checkout form.
        if (window.addressFields && key in window.addressFields) {
            window.addressFields[key] = value;
        }
    }

    // Fetch address data from ViaCEP API and fill-in the correspondent fields.
    async function autofillAddress(postcode) {
        const url = `https://viacep.com.br/ws/${postcode}/json/`;

        toggleLoading(true);

        try {
            const response = await fetch(url);
            if (!response.ok) {
                throw new Error(`Request failed for URL: '${url}' >> Status code: ${response.status}`);
            }

            const data = await response.json();
            const isValid = data && !data.erro;

            const fields = {
                'street': isValid ? data.logradouro : '',
                'neighborhood': isValid ? data.bairro : '',
                'city': isValid ? data.localidade : '',
                'state': isValid ? data.uf : '',
            };

            for (let [key, value] of Object.entries(fields)) {
                setFieldValue(key, value);
            }

            document.body.dispatchEvent(
                new CustomEvent('update_checkout', { detail: { update_shipping_method: true } }),
            );
        } catch (error) {
            console.error('Address Autocomplete Error:', error);
        } finally {
            toggleLoading(false);
        }
    }

    // Set loading state via Alpine's $data
    function toggleLoading(value) {
        const target = window.addressFields;
        if (target && 'loading' in target) {
            target.loading = value;
            // console.log('Loading state updated:', target.loading);
        }
    }
}
