import express from 'express';

import db from './db/db.js';
import log from './log.js';
import config from './config.js';

let router = express.Router();

router.get('/api/activity', async (req, res) => {
    try {
        const date = req.query.date;

        if (!date) {
            res.sendStatus(400);
            return;
        }

        res.json(await db.get(date));
    } catch (ex) {
        log.error(ex);
        res.sendStatus(500);
    }
});

router.post('/api/activity', async (req, res) => {
    try {
        const secret = req.header('x-secret');
        const client = config.getClientBySecret(secret);

        if (!client) {
            res.sendStatus(401);
            return;
        }

        if (req.body.constructor !== Object || !req.body.entries || req.body.entries.constructor !== Object) {
            res.sendStatus(400);
                return;
        }

        for (const [date, activity] of Object.entries(req.body.entries)) {
            // yyyy-mm-dd
            if (!(/^(\d{4})-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])$/.test(date))) {
                res.sendStatus(400);
                return;
            }

            if (activity.constructor !== Object) {
                res.sendStatus(400);
                return;
            }

            for (const category of Object.keys(activity)) {
                if (typeof activity[category] !== 'number') {
                    res.sendStatus(400);
                    return;
                }
            }
        }

        await db.update(client.id, req.body.entries);
        res.sendStatus(200);
    } catch (ex) {
        log.error(ex);
        res.sendStatus(500);
    }
});

export default router;