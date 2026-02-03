import swc from "@swc/core"

import { Backend } from "../../backends/backend.js"

export class SwcBackend extends Backend {
    override compile(): Promise<void> {
        throw new Error("Method not implemented.")
    }
    private compiler!: swc.Compiler

    override async initialise(): Promise<void> {
        this.compiler = new swc.Compiler()
    }

    override terminate(): Promise<void> {
        throw new Error("Method not implemented.")
    }

    async transform(path: string, options: swc.Options): Promise<swc.Output> {
        return await this.compiler.transformFile(path, options)
    }
}
