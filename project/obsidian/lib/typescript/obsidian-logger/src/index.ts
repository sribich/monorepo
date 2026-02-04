export class Logger {
    private static pluginName: string = "basalt"

    static setPluginName(name: string) {
        const shortName = name.replace("basalt-", "")

        Logger.pluginName = shortName.charAt(0).toUpperCase() + shortName.slice(1)
    }

    constructor(private name: string) {}

    log(message: unknown, ...optionalParams: unknown[]) {
        if (process.env["IN_TEST_RUNNER"]) {
            return
        }

        console.log(`[Basalt:${Logger.pluginName}:${this.name}] ${message}`, ...optionalParams)
    }

    error(message: unknown, ...optionalParams: unknown[]) {
        if (process.env["IN_TEST_RUNNER"]) {
            return
        }

        console.error(`[Basalt:${Logger.pluginName}:${this.name}] ${message}`, ...optionalParams)
    }

    warn(message: unknown, ...optionalParams: unknown[]) {
        if (process.env["IN_TEST_RUNNER"]) {
            return
        }

        console.warn(`[Basalt:${Logger.pluginName}:${this.name}] ${message}`, ...optionalParams)
    }

    debug(message: unknown, ...optionalParams: unknown[]) {
        if (process.env["IN_TEST_RUNNER"]) {
            return
        }

        console.debug(`[Basalt:${Logger.pluginName}:${this.name}] ${message}`, ...optionalParams)
    }

    verbose(message: unknown, ...optionalParams: unknown[]) {
        if (process.env["IN_TEST_RUNNER"]) {
            return
        }

        console.trace(`[Basalt:${Logger.pluginName}:${this.name}] ${message}`, ...optionalParams)
    }
}
