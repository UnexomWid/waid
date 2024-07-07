/**
 * WAID Server
 */
import path from 'path';
import http from 'http';
import termkit from 'terminal-kit';
import express from 'express';

import db from './db/db.js';
import log from './log.js';
import router from './router.js';
import config from './config.js';

import renderPages from './templates/templates.js';

import { dirname } from './common.js'
const __dirname = dirname(import.meta);

process.on('SIGTERM', () => {
    db.uninit();
});

termkit.terminal.cyan('WA');
termkit.terminal.magenta('ID\n');
console.log();

// Load config
try {
    await config.init();
} catch (ex) {
    log.error(`Failed to load 'config.json'\n${ex}`);
    process.exit(1);
}

log.ok("Config loaded");

// Init database
try {
    await db.init();

    log.ok("Database initialized");
} catch (ex) {
    log.error(ex);
    process.exit(3);
}

try {
    const app = express();

    app.disable('x-powered-by');

    app.use(express.urlencoded({ extended: true }));
    app.use(express.json());
    app.use(express.static(path.join(__dirname, '..', 'public')));
    app.use(router);

    await renderPages();

    const server = http.createServer(app);
    log.ok('HTTP server initialized');

    server.listen(config.get('port'), config.get('host'));

    log.ok("All good");
    console.log();

    log.ok(`Magic is happening on http://${config.get('host')}:${config.get('port')}\n`);
} catch (ex) {
    log.error(`Server initialization failed\n${ex.stack}`);
}