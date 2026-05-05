"use strict";

export default function countdown(targetDate) {
    return {
        targetDate: new Date(targetDate),
        days: '00',
        hours: '00',
        minutes: '00',
        seconds: '00',
        timer: null,

        startTimer() {
            this.updateTimer();
            this.timer = setInterval(() => this.updateTimer(), 1000);
        },

        updateTimer() {
            const now = new Date();
            const timeDiff = this.targetDate - now;

            if (timeDiff <= 0) {
                clearInterval(this.timer);
                this.days = '00';
                this.hours = '00';
                this.minutes = '00';
                this.seconds = '00';
                return;
            }

            this.days = String(Math.floor(timeDiff / (1000 * 60 * 60 * 24))).padStart(2, '0');
            this.hours = String(Math.floor((timeDiff % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60))).padStart(2, '0');
            this.minutes = String(Math.floor((timeDiff % (1000 * 60 * 60)) / (1000 * 60))).padStart(2, '0');
            this.seconds = String(Math.floor((timeDiff % (1000 * 60)) / 1000)).padStart(2, '0');
        },
    };
}
