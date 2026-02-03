import assert from "node:assert"
import ts, { type Program } from "typescript"

import type { Config } from "../../config/parser.js"
import { Backend } from "../../backends/backend.js"
import { getTypescriptConfig } from "./config.js"

/**
 * TODO: https://github.com/microsoft/TypeScript-wiki/blob/main/Using-the-Compiler-API.md#incremental-build-support-using-the-language-services
 */
export class TypescriptBackend extends Backend {
    private tsconfig: ts.ParsedCommandLine | undefined

    private host: ts.CompilerHost | undefined
    private program: ts.EmitAndSemanticDiagnosticsBuilderProgram | undefined

    private stop = () => {}

    private options: Partial<ts.ParsedCommandLine>

    constructor(config: Config, options: Partial<ts.ParsedCommandLine>) {
        // const context = new RunnerContext()
        // context.add(new ConfigContext(config))

        // super(RunnerContext.create(config))
        // @ts-ignore
        super()

        this.options = options
    }

    override async initialise(): Promise<void> {
        await this.createProgram()
    }

    override async terminate(): Promise<void> {
        this.stop()
    }

    async compile(): Promise<void> {
        console.info(`Compiling TypeScript files for project ""...`)
        await this.createProgram()

        assert(this.program)

        // console.log(this.program)

        // this.program.emit()
        //. this.program.aff

        const compilation = this.program.emit()
        const diagnostics = ts.getPreEmitDiagnostics(this.program as never as Program)
        //           .concat(compilation.diagnostics)

        const mergedDiagnostics = [...compilation.diagnostics, ...diagnostics]

        if (mergedDiagnostics.length > 0) {
            const currentDirectory = ts.sys.getCurrentDirectory()

            console.error(
                ts.formatDiagnosticsWithColorAndContext(mergedDiagnostics, {
                    getCurrentDirectory: () => currentDirectory,
                    getNewLine: () => ts.sys.newLine,
                    getCanonicalFileName: (name) => name,
                }),
            )

            // return { success: false }
        }

        console.info(`Done compiling TypeScript files for project "".`)
        // return { success: true }
    }

    private async createProgram(): Promise<void> {
        if (!this.tsconfig) {
            this.tsconfig = await getTypescriptConfig("tsconfig.json")
        }

        const persistedConfig = this.tsconfig

        const optionNames = this.options.fileNames?.filter((it) => !it.endsWith("css"))

        const config = {
            ...persistedConfig,
            ...this.options,
            fileNames: optionNames ?? persistedConfig.fileNames,
            options: {
                ...persistedConfig.options,
                ...this.options.options,
            },
        } satisfies ts.ParsedCommandLine

        // console.log(this.options.fileNames)
        // if (["node16", "nodenext", "bundler"].includes(tsconfig.options.moduleResolution?.toString().toLocaleLowerCase() || "")) {
        //     (tsconfig.options.customConditions ??= []).unshift("development")
        // }

        if (!this.host) {
            // this.host = ts.createIncrementalCompilerHost(config.options)
            this.host = ts.createCompilerHost(config.options)

            const orig = this.host.getSourceFile

            this.host.getSourceFile = (...args) => {
                const result = orig(...args)

                // @ts-ignore
                versions[args[0]] ??= 0
                // @ts-ignore
                result.version = versions[args[0]]++

                // @ts-ignore
                // console.log(result.version)
                return result
            }
        }

        if (this.host.getSourceFileByPath && !this.setPath) {
            this.setPath = true

            const origa = this.host.getSourceFileByPath
            // console.log(this.host, origa)
            this.host.getSourceFileByPath = (fileName, path, languageVersionOrOptions) => {
                // console.log("path", origa)
                // @ts-ignore
                const result = origa(fileName, path, languageVersionOrOptions)
                return result
            }
        }

        this.program = ts.createEmitAndSemanticDiagnosticsBuilderProgram(
            config.fileNames,
            config.options,
            this.host,
            this.program,
        )
    }
    private setPath = false
    // https://github.com/microsoft/TypeScript-wiki/blob/main/Using-the-Compiler-API.md#incremental-build-support-using-the-language-services
}

const versions = {}
