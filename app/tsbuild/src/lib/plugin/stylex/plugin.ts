import { extname, join } from "node:path"
import fs, { appendFile, writeFile } from "node:fs/promises"
import * as babel from "@babel/core"
// @ts-ignore
import jsxSyntaxPlugin from "@babel/plugin-syntax-jsx"
// @ts-ignore
import typescriptSyntaxPlugin from "@babel/plugin-syntax-typescript"
import stylexBabelPlugin, { type Rule } from "@stylexjs/babel-plugin"
import stylex from "@stylexjs/unplugin"

import type { Plugin } from "../plugin.js"
import { diagnostics } from "../diagnostics/diagnostics.plugin.js"
import { logger } from "../../logger.js"
import { createHash } from "node:crypto"

import assert from "node:assert"

const STYLEX_PLUGIN_ONLOAD_FILTER = /\.(jsx|js|tsx|ts|mjs|cjs|mts|cts)$/

export const stylexPlugin = (): Plugin => {
    return async (context) => {
        const useCssLayers = false
        const stylexImports = ["@stylexjs/stylex"]

        const buildContext = context.build
        const outputDirectory = buildContext.outputDirectory

        const config = {
                debug: true,
                // dev: true,
                // runtimeInjection: true,
                treeshakeCompensation: true,
                styleResolution: "application-order",
            }
        return {
            // @ts-expect-error
            rolldown: stylex.rolldown(config),
            // @ts-expect-error
            esbuild: stylex.esbuild(config)
        }

        // @ts-expect-error TS7027
        return {
            rolldown: () => {
                const dev = process.env["NODE_ENV"] === "development"
                const stylexRules: Record<string, Rule[]> = {}

                return {
                    async generateBundle(_, outputBundle) {
                        const rules = Object.values(stylexRules).flat()

                        if (rules.length === 0) {
                            return
                        }

                        // @ts-expect-error
                        const collectedCss = stylexBabelPlugin.processStylexRules(
                            rules,
                            useCssLayers,
                        )

                        // Find an existing css file to write to
                        let outfile = Object.keys(outputBundle).filter((it) =>
                            it.endsWith(".css"),
                        )?.[0]

                        if (!outfile) {
                            outfile =
                                outputDirectory.endsWith("cjs") || outputDirectory.endsWith("esm")
                                    ? join(outputDirectory, "../styles.css")
                                    : join(outputDirectory, "styles.css")

                            this.emitFile({
                                type: "asset",
                                fileName: outfile,
                                source: collectedCss,
                            })

                            return
                        }

                        // TODO: This should write to some defined "assetDir"
                        // await appendFile(outfile, collectedCss)
                        // @ts-ignore
                        outputBundle[outfile]!.source += collectedCss
                    },
                    resolveId: (source, importer) => {
                        if (source.endsWith("css")) {
                        }
                    },
                    load: async (id) => {
                        if (id.endsWith("css")) {
                        }

                        if (!STYLEX_PLUGIN_ONLOAD_FILTER.test(id)) {
                            return
                        }

                        const content = await fs.readFile(id, "utf8")

                        if (!content.includes("@stylexjs/stylex")) {
                            return
                        }

                        try {
                            const transformResult = await babel.transformAsync(content, {
                                babelrc: false,
                                filename: id,
                                plugins: [
                                    [
                                        typescriptSyntaxPlugin,
                                        {
                                            isTSX: true,
                                        },
                                    ],
                                    jsxSyntaxPlugin,
                                    // @ts-expect-error
                                    stylexBabelPlugin.withOptions({
                                        treeshakeCompensation: true,
                                        dev,
                                        unstable_moduleResolution: {
                                            type: "commonJS",
                                            rootDir: process.cwd(),
                                        },
                                        // importSources: stylexImports,
                                        // runtimeInjection: false,
                                        // useCSSLayers: true,
                                    }),
                                ],
                            })

                            const loader = getNextLoader(id)

                            if (transformResult === null) {
                                logger.warn("build", "styleX transformer returned null")

                                return {
                                    code: content,
                                    loader,
                                }
                            }

                            const { code, metadata } = transformResult

                            if (!code) {
                                logger.warn("build", "styleX transformer returned no code")

                                return {
                                    code: content,
                                    loader,
                                }
                            }

                            if (
                                !dev &&
                                metadata &&
                                metadata?.["stylex"] &&
                                metadata?.["stylex"].length > 0
                            ) {
                                stylexRules[id] = metadata["stylex"]
                            }

                            return {
                                code,
                                loader,
                            }
                        } catch (e) {
                            // Prevent an error from killing the process
                            logger.error(`Failed to transform stylex styles: ${e}`)
                            logger.error(e)
                            return null
                        }
                    },
                }
            },
            esbuild: (isMaster) => ({
                name: "tsbuild:stylex",
                setup(build) {
                    const dev = process.env["NODE_ENV"] === "development"

                    const stylexRules: Record<string, Rule[]> = {}

                    build.onLoad(
                        { filter: STYLEX_PLUGIN_ONLOAD_FILTER },
                        diagnostics.span(
                            { name: "plugin:stylex", phase: "onLoad" },
                            async (span, args) => {
                                const content = await fs.readFile(args.path, "utf8")

                                if (!content.includes("@stylexjs/stylex")) {
                                    return
                                }

                                try {
                                    const transformResult = await babel.transformAsync(content, {
                                        babelrc: false,
                                        filename: args.path,
                                        plugins: [
                                            [
                                                typescriptSyntaxPlugin,
                                                {
                                                    isTSX: true,
                                                },
                                            ],
                                            jsxSyntaxPlugin,
                                            // @ts-expect-error
                                            stylexBabelPlugin.withOptions({
                                                treeshakeCompensation: true,
                                                dev,
                                                unstable_moduleResolution: {
                                                    type: "commonJS",
                                                    rootDir: process.cwd(),
                                                },
                                                // importSources: stylexImports,
                                                // runtimeInjection: false,
                                                // useCSSLayers: true,
                                            }),
                                        ],
                                    })

                                    const loader = getNextLoader(args.path)

                                    if (transformResult === null) {
                                        logger.warn("build", "styleX transformer returned null")

                                        return {
                                            contents: content,
                                            loader,
                                        }
                                    }

                                    const { code, metadata } = transformResult

                                    if (!code) {
                                        logger.warn("build", "styleX transformer returned no code")

                                        return {
                                            contents: content,
                                            loader,
                                        }
                                    }

                                    if (
                                        !dev &&
                                        metadata &&
                                        metadata?.["stylex"] &&
                                        metadata?.["stylex"].length > 0
                                    ) {
                                        stylexRules[args.path] = metadata["stylex"]
                                    }

                                    return {
                                        contents: code,
                                        loader,
                                    }
                                } catch (e) {
                                    // Prevent an error from killing the process
                                    logger.error("build", `Failed to transform stylex styles: ${e}`)
                                    return null
                                }
                            },
                        ),
                    )

                    build.onEnd(
                        diagnostics.span(
                            { name: "plugin:stylex", phase: "onEnd" },
                            async (span, result) => {
                                if (isMaster) {
                                    const rules = Object.values(stylexRules).flat()

                                    if (rules.length === 0) {
                                        return
                                    }

                                    // @ts-expect-error
                                    const collectedCss = stylexBabelPlugin.processStylexRules(
                                        rules,
                                        useCssLayers,
                                    )
                                    const shouldWriteToDisk =
                                        build.initialOptions.write === undefined ||
                                        build.initialOptions.write

                                    if (shouldWriteToDisk) {
                                        // Find an existing css file to write to
                                        let outfile = Object.keys(
                                            result.metafile?.outputs ?? {},
                                        ).filter((it) => it.endsWith(".css"))?.[0]

                                        if (!outfile) {
                                            outfile =
                                                build.initialOptions.outdir?.endsWith("cjs") ||
                                                build.initialOptions.outdir?.endsWith("esm")
                                                    ? join(
                                                          build.initialOptions.outdir!,
                                                          "../styles.css",
                                                      )
                                                    : join(
                                                          build.initialOptions.outdir!,
                                                          "styles.css",
                                                      )
                                        }

                                        // TODO: This should write to some defined "assetDir"
                                        await appendFile(outfile, collectedCss)
                                        return
                                    }

                                    if (result.outputFiles !== undefined) {
                                        const content = new TextEncoder().encode(collectedCss)

                                        result.outputFiles.push({
                                            path: "<stdout>",
                                            contents: content,
                                            hash: createHash("sha256")
                                                .update(content)
                                                .digest("hex"),
                                            get text() {
                                                return collectedCss
                                            },
                                        })
                                    }
                                }
                            },
                        ),
                    )
                },
            }),
        }
    }
}

const getNextLoader = (fileName: string) => {
    switch (extname(fileName)) {
        case ".tsx":
            return "tsx"
        case ".jsx":
            return "jsx"
        case ".ts":
            return "ts"
    }

    return "js"
}

declare module "@babel/core" {
    interface BabelFileMetadata {
        stylex: Rule[] | null
    }
}
