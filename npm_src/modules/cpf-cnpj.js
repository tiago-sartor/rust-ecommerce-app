"use strict";

export default function CPFandCNPJvalidation() {
    return {

        init() {
            this.$watch('cpf_cnpj', value => this.toggleCnpjFields(value));
            // Initialize state
            this.toggleCnpjFields(this.cpf_cnpj);
        },

        toggleCnpjFields(value) {
            const isCnpj = (value || '').length > 14;
            const fields = ['state_registration', 'company_name'];

            fields.forEach(id => {
                const input = document.getElementById(id);
                if (input) {
                    const container = input.closest('.grid > div');
                    if (container) container.style.display = isCnpj ? 'block' : 'none';
                    input.required = isCnpj;
                }
            });
        },

        validateCPForCNPJ(input) {
            // Clean the input sequence and remove formatting, allowing only alphanumeric characters.
            const sanitizedInput = String(input).toUpperCase().replace(/[^A-Z0-9]/g, '');

            // Check if the sequence consists of the same character (e.g., '00...0', 'AA...A').
            if (/^([A-Z0-9])\1+$/.test(sanitizedInput)) {
                return false;
            }

            // Check length and structure (11 digits for CPF or 12 alphanumeric chars + 2 digits for CNPJ).
            if (sanitizedInput.length === 11 && /^[0-9]{11}$/.test(sanitizedInput)) {
                return this.validateCPF(sanitizedInput);
            } else if (sanitizedInput.length === 14 && /^[A-Z0-9]{12}[0-9]{2}$/.test(sanitizedInput)) {
                return this.validateCNPJ(sanitizedInput);
            } else {
                return false;
            }
        },

        validateCPF(cpf) {
            // Helper to calculate a check digit
            const weightedSum = (cpfSlice, weight) => {
                let total = 0;
                for (let i = 0; i < cpfSlice.length; i++) {
                    const digit = Number(cpfSlice.charAt(i));
                    total += digit * (weight - i);
                }
                let remainder = (total * 10) % 11;
                if (remainder === 10 || remainder === 11) remainder = 0;
                return remainder;
            }

            // Calculate first check digit (for positions 1..9)
            const firstDigit = weightedSum(cpf.substring(0, 9), 10);
            if (firstDigit !== Number(cpf.charAt(9))) {
                return false;
            }

            // Calculate second check digit (for positions 1..10)
            const secondDigit = weightedSum(cpf.substring(0, 10), 11);
            if (secondDigit !== Number(cpf.charAt(10))) {
                return false;
            }

            return true;
        },

        validateCNPJ(cnpj) {
            const base = cnpj.substring(0, 12);

            // Weighted sum helper: receives a string slice and an array of weights to calculate a check digit.
            const weightedSum = (cnpjSlice, weights) => {
                let total = 0;
                for (let i = 0; i < cnpjSlice.length; i++) {
                    const char = cnpjSlice[i];
                    const value = char.charCodeAt(0) - 48;
                    total += value * weights[i];
                }
                const remainder = total % 11;
                return remainder < 2 ? 0 : 11 - remainder;
            };

            const firstDigitWeights = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
            const secondDigitWeights = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

            // Calculate first check digit.
            const firstDigit = weightedSum(base, firstDigitWeights);

            // Append first check digit to the input and calculate second check digit.
            const secondDigit = weightedSum(base + firstDigit.toString(), secondDigitWeights);

            // Return both check digits as a string and compare with the last two digits of the input.
            if ((firstDigit.toString() + secondDigit.toString()) === cnpj.substring(12)) {
                return true;
            }

            return false;
        }
    }
}