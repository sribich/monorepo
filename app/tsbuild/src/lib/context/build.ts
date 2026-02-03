import { resolve } from "node:path"
import { cwd } from "node:process"
import type { Config } from "../config/parser.js"
import { Context } from "./context.js"

export class BuildContext extends Context {
    public readonly mode: "build" | "dev"

    public readonly rootDirectory: string
    public readonly outputDirectory: string

    constructor(mode: "build" | "dev", config: Config) {
        super()

        this.mode = mode

        this.rootDirectory = cwd()
        this.outputDirectory = resolve(config.outdir ?? "dist")
    }
}
