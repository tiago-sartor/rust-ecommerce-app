'use strict';

function actionDropdown() {
    return {
        open: false,
        toggle() {
            this.open = !this.open;
            if (this.open) this.position();
        },
        position() {
            this.$nextTick(() => {
                const button = this.$el;
                const dropdown = this.$refs.dropdown;
                const rect = button.getBoundingClientRect();
                dropdown.style.top = `${rect.bottom + window.scrollY}px`;
                dropdown.style.right = `${window.innerWidth - rect.right}px`;

                // Reposition if would overflow viewport
                const dropdownRect = dropdown.getBoundingClientRect();
                if (dropdownRect.bottom > window.innerHeight) {
                    dropdown.style.top = `${rect.top + window.scrollY - dropdownRect.height}px`;
                }
            });
        }
    }
}
