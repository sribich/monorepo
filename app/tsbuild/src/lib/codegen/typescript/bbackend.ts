/*
import { writeFile } from "fs/promises";
import { Backend } from "../backend.js";
import { getTypescriptConfig } from "./config.js";
import ts from "typescript";
import type { CliConfigs } from "src/lib/config/parser.js";

export class TypescriptBackend extends Backend {
    async build(files: string[], configs: CliConfigs): Promise<void> {
        console.log(files)
        if (files.length > 1) {
            throw new Error(`Cannot supply more than 1 entrypoint to a typescript build.`)
        }

        const tsconfig = await getTypescriptConfig("tsconfig.json")

        for (const file of files) {
            for (const format of (configs.build.formats || [])) {
                switch (format) {
                    case "cjs":
                        // promises.push($`tsc --module commonjs --verbatimModuleSyntax false --outDir dist/cjs ${file}`)
                        emitCjs(tsconfig, file)
                        continue
                    case "esm":
                        // promises.push($`tsc --module node16 --outDir dist/esm ${file}`)
                        emitEsm(tsconfig, file)
                        continue
                }
            }

            if (configs.build.formats?.includes("cjs")) {
                await writeFile("dist/cjs/package.json", '{ "type": "commonjs" }')
            }
        }
    }
}

const emitCjs = (tsConfig: ts.ParsedCommandLine, _file: string) => {
    const cjsConfig = {
        ...tsConfig,
        options: {
            ...tsConfig.options,
            module: ts.ModuleKind.CommonJS,
            moduleResolution: ts.ModuleResolutionKind.Node10,
            composite: false,
            declaration: true,
            // rootDir: options.sourceDir,
            // outDir: path.join(options.outputPath, "cjs"),
            verbatimModuleSyntax: false,
            outDir: "dist/cjs"
        },
    } satisfies ts.ParsedCommandLine

    /*const program =* createProgram(cjsConfig)
}

const emitEsm = (tsConfig: ts.ParsedCommandLine, _file: string) => {
    const esmConfig = {
        ...tsConfig,
        options: {
            ...tsConfig.options,
            module: ts.ModuleKind.Node16,
            composite: false,
            declaration: true,
            // rootDir: options.sourceDir,
            // outDir: path.join(options.outputPath, "esm"),
            outDir: "dist/esm",
        },
    }

    /*const program =* createProgram(esmConfig)
    // addPackageForModule("commonjs", cjsConfig.options.outDir)
}


function createProgram(
    tsconfig: ts.ParsedCommandLine,
    options: { projectName: string } = { projectName: "" },
) {
    if (["node16", "nodenext", "bundler"].includes(tsconfig.options.moduleResolution?.toString().toLocaleLowerCase() || "")) {
        (tsconfig.options.customConditions ??= []).unshift("development")
    }

    const program = ts.createProgram({
        rootNames: tsconfig.fileNames,
        options: {
            ...tsconfig.options,
        },
        host: ts.createCompilerHost(tsconfig.options),
    })

    console.info(`Compiling TypeScript files for project "${options.projectName}"...`)

    const compilation = program.emit()
    const diagnostics = ts.getPreEmitDiagnostics(program).concat(compilation.diagnostics)

    if (diagnostics.length > 0) {
        const currentDirectory = ts.sys.getCurrentDirectory()

        console.error(
            ts.formatDiagnosticsWithColorAndContext(diagnostics, {
                getCurrentDirectory: () => currentDirectory,
                getNewLine: () => ts.sys.newLine,
                getCanonicalFileName: (name) => name,
            }),
        )

        return { success: false }
    }

    console.info(`Done compiling TypeScript files for project "${options.projectName}".`)

    return { success: true }
}

/*
export const compileTypeScript = (options: TypeScriptCompilationOptions): ExecutorResult => {
    const tsConfig = getNormalizedTsConfig(options)

    rmSync(options.outputPath, { recursive: true, force: true })

    if (options.legacyBuild) {
        return emitLegacyBuild(tsConfig, options)
    }

    return createProgram(tsConfig, options)
}

export interface TypeScriptCompilationOptions {
    projectName: string
    /**
     * The root directory for the TypeScript compilation. This should
     * most often be the default nx "projectRoot" directory.
     *
    rootDir: string
    sourceDir: string
    errorCount: number
}



*/

/*
const emitLegacyBuild = (tsConfig: ts.ParsedCommandLine, options: TypeScriptCompilationOptions) => {
    const typeConfig = {
        ...tsConfig,
        options: {
            ...tsConfig.options,
            emitDeclarationOnly: true,
            declaration: true,
            rootDir: options.sourceDir,
            outDir: path.join(options.outputPath, "types"),
        },
    }
    const esmConfig = {
        ...tsConfig,
        options: {
            ...tsConfig.options,
            module: ts.ModuleKind.ESNext,
            composite: false,
            declaration: true,
            rootDir: options.sourceDir,
            outDir: path.join(options.outputPath, "esm"),
        },
    }
    const cjsConfig = {
        ...tsConfig,
        options: {
            ...tsConfig.options,
            module: ts.ModuleKind.CommonJS,
            composite: false,
            declaration: true,
            rootDir: options.sourceDir,
            outDir: path.join(options.outputPath, "cjs"),
            verbatimModuleSyntax: false,
        },
    }

    // const typeProgram = createProgram(typeConfig, options)
    const esmProgram = createProgram(esmConfig, options)
    const cjsProgram = createProgram(cjsConfig, options)

    if (cjsProgram.success) {
        addPackageForModule("commonjs", cjsConfig.options.outDir)
        // renameToCjs(cjsConfig.options.outDir)
    }

    if (esmProgram.success) {
        addPackageForModule("module", esmConfig.options.outDir)
    }

    return {
        success: /*typeProgram.success &&* / esmProgram.success && cjsProgram.success,
    }
}

export const compileTypeScriptWatcher = (
    options: TypeScriptCompilationOptions,
    callback: (
        diagnostic: ts.Diagnostic,
        newLine: string,
        options: ts.CompilerOptions,
        errorCount: number | undefined,
    ) => void | Promise<void>,
): ts.WatchOfFilesAndCompilerOptions<ts.BuilderProgram> => {
    const tsConfig = getNormalizedTsConfig(options)

    rmSync(options.outputPath, { recursive: true, force: true })

    const host = ts.createWatchCompilerHost(tsConfig.fileNames, tsConfig.options, ts.sys)

    const originalOnWatchStatusChange = host.onWatchStatusChange

    host.onWatchStatusChange = async (a, b, c, d) => {
        originalOnWatchStatusChange?.(a, b, c, d)
        await callback?.(a, b, c, d)
    }

    return ts.createWatchProgram(host)
}
*/
