"use strict";

import Alpine from 'alpinejs';
import collapse from '@alpinejs/collapse';
import intersect from '@alpinejs/intersect';
import persist from '@alpinejs/persist'
import resize from '@alpinejs/resize';
import mask from '@alpinejs/mask';
import sort from '@alpinejs/sort';

// Search module
import search from './modules/search.js';
Alpine.data('search', search);

// Countdown module
import countdown from './modules/countdown.js';
Alpine.data('countdown', countdown);

// Register Alpine Plugins
Alpine.plugin(collapse);
Alpine.plugin(intersect);
Alpine.plugin(persist);
Alpine.plugin(resize);
Alpine.plugin(mask);
Alpine.plugin(sort);

// Initialize Alpine.js
Alpine.start();