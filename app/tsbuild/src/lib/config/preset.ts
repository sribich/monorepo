import type { Config } from "./parser.js"

/**
 * app - nodeApp webApp
 * lib - lib nodeLib webLib
 */
export const configurePresets = (config: Partial<Config>): void => {
    if (!config.preset) {
        configureApp(config)
        return
    }

    switch (config.preset) {
        case "nodeApp":
            configureApp(config)
            return
        case "lib":
        case "nodeLib":
            configureLib(config)
            return
        case "webApp":
        case "webLib":
            configureWeb(config)
            return
        default:
            throw new Error(`Unable to configure unknown preset '${config.preset}'.`)
    }
}

const configureApp = (config: Partial<Config>) => {
    config.platform ??= "node"
}

const configureLib = (config: Partial<Config>) => {
    config.platform ??= "neutral"
    config.formats ??= ["esm"]
    config.mainFields ??= ["browser", "module", "main"]
}

const configureWeb = (config: Partial<Config>) => {
    config.platform ??= "browser"
}
