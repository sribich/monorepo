import { existsSync } from "fs"
import { dirname } from "path"
import { inspect } from "util"
import { findUp } from "find-up"
import ts from "typescript"
import { getTypescriptConfig } from "../../codegen/typescript/config.js"
import type { Plugin } from "../plugin.js"

type PluginOptions = {
    /**
     * The absolute path to a `tsconfig.json` file.
     */
    tsconfigPath: string
}

export const ignoreOptionalDependenciesPlugin: Plugin = async () => {
    return {
        esbuild: {
            name: "ignore-optional-dependencies-plugin",
            async setup(build) {
                const parsedConfig = await getTypescriptConfig("tsconfig.json")
                // const parsedConfig = parseConfig(options.tsconfigPath)

                const cache: Map<string, null | { path: string; external?: boolean }> = new Map()

                build.onResolve({ namespace: "file", filter: /.*/ }, async (args) => {
                    if (args.path.startsWith(".") || args.path.startsWith("/")) {
                        return null
                    }

                    if (cache.has(args.path)) {
                        return cache.get(args.path)
                    }

                    const moduleName = getModuleName(args.path)

                    // Find package.json
                    const packageJson = await findUp("package.json", {
                        type: "file",
                        cwd: args.resolveDir,
                    })

                    if (!packageJson) {
                        throw new Error(`No package.json found for ${args.path}`)
                    }

                    let data = await import(packageJson, {
                        with: { type: "json" },
                    })

                    if ("default" in data && !("version" in data)) {
                        data = data["default"]
                    }

                    const peerMeta = data["peerDependenciesMeta"] || {}

                    if (peerMeta[moduleName] && peerMeta[moduleName]["optional"] === true) {
                        const { resolvedModule } = ts.nodeModuleNameResolver(
                            args.path,
                            args.importer,
                            parsedConfig.options,
                            ts.sys,
                        )

                        if (!resolvedModule) {
                            cache.set(args.path, {
                                path: args.path,
                                external: true,
                            })

                            return cache.get(args.path)
                        }
                    }

                    cache.set(args.path, null)
                    return null
                })
            },
        },
    }
}

const parseConfig = (tsconfigPath: string): ts.ParsedCommandLine => {
    if (!existsSync(tsconfigPath)) {
        throw new Error(`The tsconfig path "${tsconfigPath}" does not exist.`)
    }

    const text = ts.sys.readFile(tsconfigPath)

    if (!text) {
        throw new Error(`Failed to read "${tsconfigPath}"`)
    }

    const result = ts.parseConfigFileTextToJson(tsconfigPath, text)

    if (result.error) {
        console.error(inspect(result.error, false, 8, true))

        throw new Error(`Failed to parse "${tsconfigPath}"`)
    }

    const loadedConfig = result.config
    const parsedConfig = ts.parseJsonConfigFileContent(loadedConfig, ts.sys, dirname(tsconfigPath))

    if (parsedConfig.errors.length > 0) {
        console.error(inspect(parsedConfig.errors, false, 8, true))
    }

    return parsedConfig
}

const getModuleName = (path: string): string => {
    let moduleName = path.split("/")[0]

    if (path.startsWith("/")) {
        return path
    }

    if (path.startsWith("@")) {
        const split = path.split("/")

        moduleName = `${split[0]}/${split[1]}`
    }

    if (!moduleName) {
        throw new Error(`Unable to resolve module name for ${path}`)
    }

    return moduleName
}
