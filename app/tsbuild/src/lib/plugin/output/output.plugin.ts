import { writeFile } from "node:fs/promises"
import path from "node:path"

import type { Plugin } from "../plugin.js"
import type { OutputPluginConfig } from "./output.types.js"

export const outputPlugin =
    (options: OutputPluginConfig): Plugin =>
    async () => {
        const output = options

        return {
            modifyConfig: (config) => {
                config.esbuild ??= {}
                config.esbuild.write = false
            },
            esbuild: {
                name: "output-plugin",
                async setup(build) {
                    const assetFileNames = output.assetFileNames

                    if (!assetFileNames) {
                        return
                    }

                    build.onEnd(async (result) => {
                        for (const file of result.outputFiles ?? []) {
                            if (file.path.includes("__internal_do_not_write__")) {
                                continue
                            }

                            const dirname = path.dirname(file.path)
                            const basename = path.basename(file.path)

                            const nextname = assetFileNames({ name: basename })

                            if (!nextname) {
                                continue
                            }

                            const promises = [] as Array<Promise<void>>

                            promises.push(
                                write(
                                    path.join(dirname, nextname === basename ? basename : nextname),
                                    file.contents,
                                ),
                            )

                            await Promise.all(promises)
                        }
                    })
                },
            },
        }
    }

const write = async (path: string, content: string | Uint8Array): Promise<void> => {
    return writeFile(path, content)
}
