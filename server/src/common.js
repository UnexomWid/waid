import path from 'path';
import { fileURLToPath } from 'url';

export function filename(meta) { return fileURLToPath(meta.url) }
export function dirname(meta) { return path.dirname(filename(meta)); }