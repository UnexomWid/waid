/**
 * ERYN :)
 */
import { dirname } from './common.js';
const __dirname = dirname(import.meta);

import eryn from 'eryn';

var engine = eryn({
    workingDirectory: __dirname,
    throwOnCompileDirError: true,
    throwOnMissingEntry: true
});

engine.compileDir("", ["**/*.eryn"]);

export default engine;