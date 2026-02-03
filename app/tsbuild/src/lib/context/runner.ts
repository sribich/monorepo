import { normalizeConfig, type Config, type NormalizedConfig } from "../config/parser.js"
import type { RunnerMode } from "../runner.js"
import { BuildContext } from "./build.js"
import { EntrypointContext } from "./entrypoint.js"
import { PluginContext } from "./plugin.js"
import { RepositoryContext } from "./repository.js"
import { WatchContext } from "./watch.js"

export class RunnerContext {
    config!: NormalizedConfig
    entrypoint!: EntrypointContext
    build!: BuildContext
    repository!: RepositoryContext
    plugin!: PluginContext
    watch!: WatchContext

    private constructor() {}

    public static async create(config: Config, mode: RunnerMode): Promise<RunnerContext> {
        const context = new RunnerContext()

        context.config = normalizeConfig(config)
        context.entrypoint = new EntrypointContext(context)
        context.build = new BuildContext(mode, config)
        context.repository = new RepositoryContext()
        context.plugin = new PluginContext(context)
        context.watch = new WatchContext(config, context.build)

        await context.build.initialise()
        await context.entrypoint.initialise()
        await context.repository.initialise()
        await context.plugin.initialise()
        await context.watch.initialise()

        return context
    }

    async terminate(): Promise<void> {
        await Promise.allSettled([
            this.watch.terminate(),
            this.plugin.terminate(),
            this.repository.terminate(),
            this.build.terminate(),
        ])
    }
}
