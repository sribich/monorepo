import path from "node:path"
import type { NonEmptyArray } from "../util/typecheck.js"
import { existsSync } from "../util/fs.js"

export interface FindUpOptions {
    cwd: string
    stopDir?: string | undefined
    packageKey: string
}

export const findConfigFile = async (
    files: NonEmptyArray<string>,
    options: Partial<FindUpOptions>
): Promise<string | null> => {
    const cwd = path.resolve(options.cwd ?? process.cwd())
    const stopDir = options.stopDir ? path.resolve(options.stopDir) : path.parse(cwd).root
    const packageKey = options.packageKey ?? "tsbuild"

    return find(files, {
        cwd,
        stopDir,
        packageKey,
    })
}

const find = async (files: NonEmptyArray<string>, options: FindUpOptions): Promise<string | null> => {
    const { cwd, stopDir, packageKey } = options

    for (const filename of files) {
        const file = path.resolve(cwd, filename)

        if (existsSync(file)) {
            if (path.basename(file) !== "package.json") {
                return file
            }

            const packageJson = import(file, {
                assert: { type: "json" },
            })

            if (packageKey in packageJson) {
                return file
            }
        }
    }

    if (cwd === stopDir || path.basename(cwd) === "node_modules") {
        return null
    }

    return await find(
        files,
        {
            cwd: path.dirname(options.cwd),
            stopDir,
            packageKey,
        }
    )
}
