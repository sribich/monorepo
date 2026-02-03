import { existsSync } from "node:fs"
import { dirname } from "node:path"
import { inspect } from "node:util"
import ts from "typescript"

export const getTypescriptConfig = async (tsconfigPath: string): Promise<ts.ParsedCommandLine> => {
    if (!existsSync(tsconfigPath)) {
        throw new Error(`Could not find a tsconfig file at "${tsconfigPath}".`)
    }

    const configContent = ts.sys.readFile(tsconfigPath)

    if (!configContent) {
        throw new Error(`Unable to read tsconfig file at "${tsconfigPath}".`)
    }

    const result = ts.parseConfigFileTextToJson(tsconfigPath, configContent)

    if (result.error) {
        console.error(inspect(result.error, false, 8, true))
        console.error("foobar")

        throw new Error(`Unable to parse tsconfig file at "${tsconfigPath}"`)
    }

    const loadedConfig = result.config
    // const resolvedPackageExtends = await resolveExtends(loadedConfig.extends ?? [])

    // loadedConfig.extends = resolvedPackageExtends.extends

    loadedConfig.exclude = [
        "node_modules",
        "**/*.stories.tsx",
        "**/*.test.tsx",
        "**/*.perf-test.tsx",
        "src/test.ts",
    ]

    const parsedConfig = ts.parseJsonConfigFileContent(
        loadedConfig,
        ts.sys,
        dirname(tsconfigPath) /*, resolvedPackageExtends.compilerOptions */,
    )

    if (parsedConfig.errors.length > 0) {
        console.error(".")
        console.error(loadedConfig)
        console.error(inspect(parsedConfig.errors, false, 8, true))
    }

    return parsedConfig
}

/*
const resolveExtends = async (oldExtends: string[]) => {
    let compilerOptions = {}
    const newExtends = []

    for (const clause of oldExtends) {
        if (clause.startsWith(".")) {
            newExtends.push(clause)
            continue
        }

        const data = await import(clause)
        const importPath = await import.meta.resolve?.(clause) ?? "."

        if ("compilerOptions" in data) {
            const parsedConfig = ts.parseJsonConfigFileContent(data.compilerOptions, ts.sys, dirname(importPath))

            compilerOptions = {
                ...compilerOptions,
                ...parsedConfig,
            }
        }
    }

    return {
        compilerOptions,
        extends: newExtends,
    }
}
*/
