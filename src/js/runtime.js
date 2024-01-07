((globalThis) => {
    const core = Deno.core;

    function argsToMessage(...args) {
        return args.map((arg) => JSON.stringify(arg)).join(" ");
    }

    globalThis.console = {
        log: (...args) => {
            return core.opAsync("op_console_message", "info", argsToMessage(...args));
        },
        warn: (...args) => {
            return core.opAsync("op_console_message", "warn", argsToMessage(...args));
        },
        error: (...args) => {
            return core.opAsync("op_console_message", "error", argsToMessage(...args));
        },
    };
})(globalThis);