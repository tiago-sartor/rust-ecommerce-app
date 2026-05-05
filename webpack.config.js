"use strict";

import path from 'path';

export default {
    entry: './src/index.js',
    output: {
        path: path.resolve('.', 'public', 'js'),
        filename: 'bundle.js',
    },
}