"use strict";

export default function formFieldValidation(fieldKeys) {
    return {

        // Dynamically initialize all fields as empty strings
        // and spread them as properties of the object
        ...Object.fromEntries(fieldKeys.map(key => [key, ''])),

        isValid: {},
        errors: {},

        validateField(key, label) {
            if (!key || !label) {
                console.error('Key and label are required for validation.');
                return;
            }
            if (key === 'email') {
                const emailPattern = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
                if (this[key].trim() === '') {
                    this.setErrorRequired(key, label);
                } else if (!emailPattern.test(this[key])) {
                    this.setErrorInvalid(key, label);
                } else {
                    this.setSuccess(key);
                }
            } else if (['postcode', 'phone'].includes(key)) {
                const sanitizedInput = this[key].replace(/[^0-9]/g, '');
                if (sanitizedInput === '') {
                    this.setErrorRequired(key, label);
                } else if (
                    (key === 'postcode' && sanitizedInput.length !== 8) ||
                    (key === 'phone' && (sanitizedInput.length !== 10 && sanitizedInput.length !== 11))
                ) {
                    this.setErrorInvalid(key, label);
                } else {
                    this.setSuccess(key);
                }
            } else {
                if (this[key].trim() === '') {
                    this.setErrorRequired(key, label);
                } else {
                    this.setSuccess(key);
                }
            }
        },
        setErrorRequired(key, label) {
            this.errors[key] = `${label} é um campo obrigatório.`;
            this.isValid[key] = false;
        },
        setErrorInvalid(key, label) {
            this.errors[key] = `${label} não é válido.`;
            this.isValid[key] = false;
        },
        setSuccess(key) {
            this.isValid[key] = true;
            this.errors[key] = false;
        },
        formatTextInput(input) {
            return input
                .trim()
                .replace(/\s+/g, ' ')
                .toLowerCase()
                .split(' ')
                .map(word => word.charAt(0).toUpperCase() + word.slice(1))
                .join(' ');
        },
    }
}