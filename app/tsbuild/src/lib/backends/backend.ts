import type { RunnerContext } from "../context/runner.js"

export abstract class Backend {
    protected context: RunnerContext

    constructor(context: RunnerContext) {
        this.context = context
    }

    abstract initialise(): Promise<void>

    abstract terminate(): Promise<void>

    abstract compile(): Promise<void>
}
