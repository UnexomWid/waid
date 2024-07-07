import config from '../config.js';
import eryn from '../eryn.js';
import fs from 'fs';
import path from  'path';

import { dirname } from '../common.js'
const __dirname = dirname(import.meta);

async function renderPages()
{
    const pageConfig = config.get("page");

    const page = eryn.render('templates/index.eryn', {
        title_1: pageConfig["title_1"],
        title_2: pageConfig["title_2"]
    }).toString('utf8');

    const indexPath = path.join(__dirname, "../../public/index.html")
    await fs.promises.writeFile(indexPath, page)
}

export default renderPages;