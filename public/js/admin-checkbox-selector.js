'use strict';

function checkboxSelector(ids) {
    return {
        selected: [],
        allIds: ids,
        isAllSelected() {
            return this.allIds.length > 0 && this.selected.length === this.allIds.length;
        },
        toggleAll() {
            this.selected = this.isAllSelected() ? [] : [...this.allIds];
        },
        toggleSelect(id) {
            const idx = this.selected.indexOf(id);
            if (idx === -1) {
                this.selected.push(id);
            } else {
                this.selected.splice(idx, 1);
            }
        },
    }
}
