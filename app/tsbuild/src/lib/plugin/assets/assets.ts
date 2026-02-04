import type { Stats } from "node:fs"
import fs, { stat } from "node:fs/promises"
import path, { join, resolve } from "node:path"

import chalk from "chalk"
import type { FSWatcher } from "chokidar"
import chokidar from "chokidar"
import isGlob from "is-glob"
import picomatch from "picomatch"

import { diagnostics } from "../../plugin/diagnostics/diagnostics.plugin.js"
import type { Plugin } from "../../plugin/plugin.js"
import type { Asset, AssetConfig, AssetGlob } from "./assets.types.js"

export const assetsPlugin = (options: AssetConfig): Plugin =>
    async (context) => {
        if (!options || options.enabled === false || options.globs.length === 0) {
            return null
        }

        const buildContext = context.build

        const assetGlobs = options.globs

        const fromDirectory = buildContext.rootDirectory
        const toDirectory = buildContext.outputDirectory

        if (!path.isAbsolute(fromDirectory) || !path.isAbsolute(toDirectory)) {
            console.error(
                `plugin:assets: ${chalk.redBright(
                    "Options 'fromDirectory' and 'toDirectory' should resolve to an absolute path.",
                )}`,
            )

            console.info(`plugin:assets: ${chalk.gray(`fromDirectory = ${fromDirectory}`)}`)
            console.info(`plugin:assets: ${chalk.gray(`toDirectory = ${toDirectory}`)}`)

            console.info(`plugin:assets: ${chalk.yellowBright("Assets will not be copied")}`)

            return null
        }

        const assets = await normalizeGlobs(fromDirectory, assetGlobs)

        const watchPatterns = assets.map((asset) => path.join(asset.input, asset.glob || ""))
        const patternMatchers = watchPatterns.map((it) => picomatch(it))

        const copies = new Set()

        let watcher: FSWatcher

        const handleFile = (path: string): void => {
            const assetIndex = patternMatchers.findIndex((isMatch) => isMatch(path))

            if (assetIndex === -1) {
                console.warn(
                    `plugin:assets: ${chalk.gray(`'${path}' does match any asset patterns. Skipping`)}`,
                )
                return
            }

            const assetMetadata = assets[assetIndex]

            if (!assetMetadata) {
                throw new Error(`plugin:assets: Asset does not exist at index ${assetIndex}`)
            }

            const resolvedPath = resolve(join(fromDirectory, path))
            const resolvedDist = resolve(
                join(
                    toDirectory,
                    assetMetadata.output || "",
                    path.replace(assetMetadata.input, ""),
                ),
            )

            if (resolvedDist.indexOf(toDirectory) !== 0) {
                throw new Error(
                    `plugin:assets: Asset destination path '${toDirectory}' has escaped the root fromDirectory.`,
                )
            }

            const copyOperation = copy(resolvedPath, resolvedDist)

            copies.add(copyOperation)

            copyOperation.finally(() => {
                copies.delete(copyOperation)
            })
        }

        const drainCopies = async (): Promise<void> => {
            await Promise.all(copies.values())
        }

        return {
            onBuildEnd: diagnostics.span(
                { name: "plugin:assets", phase: "onBuildEnd" },
                async () => {
                    if (watcher) {
                        return drainCopies()
                    }

                    watcher = chokidar.watch(watchPatterns, {
                        cwd: fromDirectory,
                    })

                    watcher.on("add", handleFile)
                    watcher.on("change", handleFile)

                    await new Promise((resolve) => {
                        watcher.once("ready", () => resolve(void 0))
                    })
                    await drainCopies()

                    if (buildContext.mode !== "dev") {
                        await watcher.close()
                    }
                },
            ),
        }
    }

const normalizeGlobs = async (
    fromDirectory: string,
    globs: readonly AssetGlob[],
): Promise<Asset[]> => {
    return Promise.all(globs.map((glob) => normalizeGlob(glob, fromDirectory)))
}

const normalizeGlob = async (glob: AssetGlob, fromDirectory: string): Promise<Asset> => {
    if (typeof glob === "string") {
        const stats = await getStats(path.join(fromDirectory, glob))
        const isDirectory = stats.isDirectory()

        return {
            input: glob,
            glob: isDirectory ? "**/*" : "", // TODO: undefined
            output: path.basename(glob),
        }
    }

    if (isGlob(glob.input)) {
        throw new Error(
            "plugin:assets: An asset input may not contain globs. Please define the glob pattern in the glob field instead.",
        )
    }

    const stats = await getStats(path.join(fromDirectory, glob.input))
    const isDirectory = stats.isDirectory()

    if (!isDirectory && glob.glob) {
        throw new Error("plugin:assets: Using globs with file inputs is not supported.")
    }

    return glob
}

const copy = async (from: string, to: string): Promise<void> => {
    try {
        return fs.cp(from, to, {
            dereference: true,
            errorOnExist: false,
            recursive: true,
            preserveTimestamps: true,
        })
    } catch (e) {
        console.error(e)
    }
}

const isNodeError = (value: unknown): value is NodeJS.ErrnoException => {
    return typeof value === "object" && value instanceof Error && "code" in value
}

const getStats = async (path: string): Promise<Stats> => {
    try {
        return stat(path)
    } catch (e) {
        if (isNodeError(e) && e.code === "ENOENT") {
            throw new Error(
                `plugin:assets: Asset glob '${path}' could not be found. Does this file exist?`,
            )
        }

        throw e
    }
}
