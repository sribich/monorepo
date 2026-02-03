import assert from "node:assert"
import chalk from "chalk"

import { EsbuildBackend } from "./backends/esbuild/backend.js"
import type { Config } from "./config/parser.js"

import { logger } from "./logger.js"
import { diagnostics } from "./plugin/diagnostics/diagnostics.plugin.js"
import { addAsyncCleanupHandler } from "./util/process.js"
import { RolldownBackend } from "./backends/rolldown/backend.js"
import type { Backend } from "./backends/backend.js"
import { RunnerContext } from "./context/runner.js"

export const RunnerMode = {
    BUILD: "build",
    DEV: "dev",
} as const
export type RunnerMode = (typeof RunnerMode)[keyof typeof RunnerMode]

export class Runner {
    private backends: Backend[] = []
    private context: RunnerContext

    private terminated = false

    private constructor(context: RunnerContext) {
        this.context = context
    }

    public static async create(config: Config, mode: RunnerMode): Promise<Runner> {
        return new Runner(await RunnerContext.create(config, mode))
    }

    public async start(): Promise<void> {
        logger.info(
            `Starting build in ${chalk.yellowBright(
                this.context.config.watch ? "watch" : "oneshot",
            )} mode.`,
        )

        addAsyncCleanupHandler(async () => {
            await this.terminate()
        })

        await this.context.plugin.modifyConfig(this.context.config)

        // await this.createContext(mode)
        await this.createBackends()

        await this.context.plugin.onStartup()

        await this.compile()

        await this.createWatcher()
    }

    public async terminate(): Promise<void> {
        if (this.terminated) {
            return
        }

        this.terminated = true

        await this.context.plugin.onShutdown()

        await Promise.all(this.backends.map((it) => it.terminate()))

        await this.context?.terminate()
    }

    private async createBackends(): Promise<void> {
        const initialisingBackends = [] as Promise<void>[]

        let leader = true

        for (const format of this.context.config.formats) {
            const backend =
                this.context.config.backend === "rolldown"
                    ? new RolldownBackend(this.context)
                    : new EsbuildBackend({ format }, this.context, leader)

            this.backends.push(backend)

            initialisingBackends.push(backend.initialise())

            leader = false
        }

        await Promise.all(initialisingBackends)
    }

    private async createWatcher(): Promise<void> {
        if (this.context.build.mode !== "dev") {
            return await this.terminate()
        }

        this.context.watch.onChange(async () => {
            await this.compile()
        })
    }

    private async compile(): Promise<void> {
        const plugins = this.context.plugin

        const trace = diagnostics.startTrace()

        try {
            await plugins.preBuild()
            await plugins.onBuildStart()

            const buildPromise = plugins.onBuild()
            await Promise.all(this.backends.map((unit) => unit.compile()))
            await buildPromise

            await plugins.onBuildEnd()
            await plugins.postBuild()
        } catch (e) {
            if (e instanceof Error) {
                console.error(`Build failed: ${e.message}`)
            } else {
                console.error(e)
            }

            if (this.context.build.mode !== "dev" || !this.context.watch) {
                throw e
            }
        }

        trace.printDiagnostics()
    }
}
