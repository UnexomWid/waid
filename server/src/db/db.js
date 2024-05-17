import fs from 'fs';
import path from  'path';

import sqlite3 from 'sqlite3';
import { open } from 'sqlite';

import eryn from '../eryn.js';

import config from '../config.js';

import { dirname } from '../common.js'
const __dirname = dirname(import.meta);

var db = null;

export default {
    init: async () => {
        // Relative to config.json
        const file = path.join(__dirname, '../..', config.get('database'));

        try {
            await fs.promises.mkdir(path.dirname(file), { recursive: true });

            db = await open({
                filename: file,
                driver: sqlite3.Database
            });
        } catch(ex) {
            return Promise.reject(`Failed to create database file '${file}'\n${ex}`);
        }

        try {
            for (const client of Object.keys(config.getClients())) {
                if (!(/^[a-zA-Z]+$/.test(client))) {
                    return Promise.reject(`Client id '${client}' is invalid; must only contain a-zA-Z\n`);
                }

                const create_table = eryn.render('db/create.eryn', {
                    table_name: client
                }).toString('utf8');

                await db.exec(create_table);
            }

            return;
        } catch(ex) {
            await db.close();
            return Promise.reject(`Failed to create database tables; file 'db/create.eryn' is missing or contains an invalid query\n${ex}`);
        }
    },
    uninit: async () => {
        if (db !== null) {
            await db.close();
        }
    },
    get: async (date) => {
        try {
            let data = {};

            for (const client of Object.keys(config.getClients())) {
                try {
                    // We trust the client from the config, but not the date
                    data[client] = await db.get(`SELECT activity FROM data_${client} WHERE date=?`, date);

                    if (data[client] && data[client].activity) {
                        data[client] = JSON.parse(data[client].activity);

                        for (const category of Object.keys(data[client])) {
                            data[client][category] = {
                                time: data[client][category],
                                color: config.getColorForCategory(category)
                            };
                        }
                    }
                } catch (ex) {
                    return Promise.reject(`Failed to retrieve activity for client '${client}' on ${date}\n${ex}`);
                }
            }

            return data;
        } catch(ex) {
            return Promise.reject(`Failed to retrieve activity on ${date}\n${ex}`);
        }
    },
    update: async(id, data) => {
        try {
            for (const [date, activity] of Object.entries(data)) {
                let entry = await db.get(`SELECT activity FROM data_${id} WHERE date=?`, date);
                let insert = false;

                if (entry && entry.activity) {
                    entry = JSON.parse(entry.activity);
                } else {
                    entry = {};
                    insert = true;
                }

                for (const [category, time] of Object.entries(activity)) {
                    if (!(category in entry)) {
                        entry[category] = 0
                    }

                    entry[category] += time;
                }

                if (insert) {
                    await db.run(`INSERT INTO data_${id}(date, activity) VALUES(?, ?)`, date, JSON.stringify(entry));
                } else {
                    await db.run(`UPDATE data_${id} SET activity=? WHERE date=?`, JSON.stringify(entry), date);
                }
            }
        } catch(ex) {
            return Promise.reject(`Failed to update data for client '${id}' in the database\n${ex}`);
        }
    }
};