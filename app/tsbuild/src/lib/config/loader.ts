import path from "node:path"
import type { BuildOptions, Plugin } from "esbuild"
import esbuild from "esbuild"
import { writeFile } from "node:fs/promises"
import { temporaryFile } from "tempy"
import type { Config } from "./parser.js"
import { findConfigFile } from "./locator.js"
import { ESM_REQUIRE_SHIM } from "../backends/esbuild/options.js"
import { pathToFileURL } from "node:url"
import { unlink, unlinkSync } from "node:fs"
import type { NonEmptyArray } from "../util/typecheck.js"

export type TsbuildConfig = Partial<Config>

export const loadConfig = async (cwd: string, configFile?: string): Promise<TsbuildConfig> => {
    const files: NonEmptyArray<string> = configFile
        ? [configFile]
        : [
              "tsbuild.config.js",
              "tsbuild.config.ts",
              "tsbuild.config.cjs",
              "tsbuild.config.mjs",
              "tsbuild.config.cts",
              "tsbuild.config.mts",
              "tsbuild.config.json",
              "package.json",
          ]

    const config = await findConfigFile(files, {
        cwd,
        stopDir: configFile ? path.dirname(cwd) : undefined,
        packageKey: "tsbuild",
    })

    if (configFile && !config) {
        throw new Error(`Could not find tsbuild config file at ${path.resolve(cwd, configFile)}.`)
    }

    return await resolveConfig(config, cwd)
}

const resolveConfig = async (file: string | null, cwd: string): Promise<TsbuildConfig> => {
    if (file?.endsWith(".json")) {
        let jsonData = await import(file, {
            assert: { type: "json" },
        })

        if (file.endsWith("package.json")) {
            jsonData = jsonData.tsbuild
        }

        return jsonData
        /*
        return {
            path: file,
            data: jsonData,
        }
        */
    }

    if (!file) {
        return {}
    }

    const text = (await requireConfig(file, cwd)) as string

    const JS_EXT_RE = /\.([mc]?[tj]s|[tj]sx)$/
    const tempFile = file.replace(
        JS_EXT_RE,
        `.bundled_${Math.random().toString(36).substring(2, 15)}.mjs`,
    )

    // const tempFile = temporaryFile({ extension: "mjs" })
    await writeFile(tempFile, text, "utf-8")

    let result

    try {
        result = await import(pathToFileURL(tempFile).href)
    } finally {
        unlinkSync(tempFile)
    }

    if ("default" in result) {
        return result.default
    }

    throw new Error(`Unable to find exported configs in ${file}`)
}

const requireConfig = async (file: string, cwd: string) => {
    const buildOptions = {
        bundle: true,
        metafile: true,
        format: "esm",
        minify: false,
        entryPoints: [file],
        absWorkingDir: cwd,
        outfile: "__internal_do_not_write__.js",
        sourcemap: "inline",
        write: false,
        platform: "node",
        loader: {
            ".node": "copy",
        },
        banner: {
            js: ESM_REQUIRE_SHIM,
        },
        plugins: [
            {
                name: "mark-non-local-external",
                setup(ctx) {
                    ctx.onResolve({ filter: /.*/ }, async (args) => {
                        if (args.path[0] === "." || path.isAbsolute(args.path)) {
                            return undefined
                        }

                        return { external: true }
                    })
                },
            },
        ],
    } as BuildOptions

    const createNotifyPlugin = (
        resolve: (value: unknown) => void,
        reject: (value: unknown) => void,
    ): Plugin => ({
        name: "notify-plugin",
        setup(build) {
            build.onEnd(async (result) => {
                if (result.errors.length !== 0) {
                    return reject(result.errors[0])
                }

                const outputFile = result.outputFiles?.[0]

                if (!outputFile) {
                    return reject(new Error("No output file"))
                }

                return resolve(outputFile.text)
            })
        },
    })

    return await new Promise((resolve, reject) => {
        const notifyPlugin = createNotifyPlugin(resolve, reject)
        buildOptions.plugins?.push(notifyPlugin)

        const build = esbuild.context({
            ...buildOptions,
        })

        build.then((build) => {
            build.rebuild()
            build.dispose()
        })
    })
}
