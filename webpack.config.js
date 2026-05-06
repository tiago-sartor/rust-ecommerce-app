"use strict";

import path from 'path';

export default {
    entry: './npm_src/index.js',
    output: {
        path: path.resolve('.', 'public', 'js'),
        filename: 'app.js',
    },
}