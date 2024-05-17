import fs from 'fs';
import path from 'path';

import { dirname } from './common.js'
const __dirname = dirname(import.meta);

const CONFIG_FILE = path.join(__dirname, "../config.json");

var config;
var clientsBySecret = {};

export default {
    init: async () => {
        config = JSON.parse(await fs.promises.readFile(CONFIG_FILE, 'utf-8'));

        for (const [id, client] of Object.entries(config.clients)) {
            if (!client.secret) {
                return Promise.reject(`Client '${id}' has no 'secret' field in config; please add this field to the config`);
            }
        
            if (client.secret in clientsBySecret) {
                return Promise.reject(`Client '${id}' has the same secret as '${clientsBySecret[client.secret].id}; secret must be unique for each client'`);
            }
        
            clientsBySecret[client.secret] = {
                id: id,
                alias: client.alias
            }
        }
    },
    get: (key) => {
        return config[key];
    },
    getClients: () => {
        return config.clients
    },
    getClientBySecret: (secret) => {
        return clientsBySecret[secret];
    },
    getColorForCategory: (category) => {
        if (config.categories[category]) {
            return config.categories[category].color;
        }
        return undefined;
    }
}