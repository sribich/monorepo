import { loadConfig } from "../lib/config/loader.js"
import { configParser, defaultConfig } from "../lib/config/parser.js"
import { type } from "arktype"
import { Runner, RunnerMode } from "../lib/runner.js"
import { configurePresets } from "../lib/config/preset.js"
import { merge } from "../lib/util/merge.js"
import { command, option, optional } from "cmd-ts"
import { File, type Command } from "../lib/util/cmd-ts.js"

export interface DevArgs {
    configFile: string | undefined
}

export const dev: Command<DevArgs> = command({
    name: "dev",
    aliases: ["serve"],
    description: "",
    args: {
        configFile: option({
            type: optional(File),
            long: "config",
            short: "c",
            description: "path to a tsbuild config file",
        }),
    },
    handler: async ({ configFile }) => {
        const persistedConfig = await loadConfig(process.cwd(), configFile)
        const mergedConfig = merge(defaultConfig, persistedConfig)

        const config = configParser(mergedConfig)

        if (config instanceof type.errors) {
            throw new Error(config.summary)
        }

        configurePresets(config)

        const runner = await Runner.create(config, RunnerMode.DEV)
        await runner.start()
    },
})
