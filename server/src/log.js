import termkit from 'terminal-kit';

export default {
    error: (msg) => {
        termkit.terminal.red(`[ERROR] ${msg}\n`);
    },
    
    ok: (msg) => {
        termkit.terminal.green(`[OK] ${msg}\n`);
    }
}