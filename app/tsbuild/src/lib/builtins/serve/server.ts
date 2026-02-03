import type { ServePluginConfig } from "./config.js"
import type { Immutable } from "../../util/immutable.js"
import type { RunnerContext } from "../../context/runner.js"

export abstract class Server {
    protected config: Immutable<ServePluginConfig>
    protected context: RunnerContext

    private buildLock = undefined as Promise<void> | undefined
    private buildLockResolver = undefined as (() => void) | undefined

    constructor(config: Immutable<ServePluginConfig>, context: RunnerContext) {
        this.config = config
        this.context = context
    }

    abstract start(/* result: BuildResult<BuildOptions>, entrypoint: string */): Promise<void>

    getLock(): Promise<void> {
        return this.buildLock ?? Promise.resolve()
    }

    acquireLock(): void {
        if (this.buildLock) {
            throw new Error("Build lock has been acquired twice")
        }

        this.buildLock = new Promise((resolve) => {
            this.buildLockResolver = resolve
        })
    }

    releaseLock(): void {
        this.buildLockResolver?.()

        this.buildLock = undefined
        this.buildLockResolver = undefined
    }
}
