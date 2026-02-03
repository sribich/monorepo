import { extendType, string, type Runner, type Type } from "cmd-ts"
import type { ArgParser } from "cmd-ts/dist/cjs/argparser.js"
import type {
    Aliased,
    Descriptive,
    Named,
    PrintHelp,
    ProvidesHelp,
    Versioned,
} from "cmd-ts/dist/cjs/helpdoc.js"
import { existsSync, statSync } from "node:fs"
import { resolve } from "node:path"

export type Command<T> = ArgParser<T> &
    PrintHelp &
    ProvidesHelp &
    Named &
    Runner<T, Promise<void>> &
    Partial<Versioned & Descriptive & Aliased>

export const ExistingPath: Type<string, string> = extendType(string, {
    displayName: "path",
    description: "An existing path on the filesystem",
    async from(path) {
        const resolved = resolve(path)

        if (!existsSync(resolved)) {
            throw new Error(`Path '${path}' does not exist`)
        }

        return resolved
    },
})

export const Directory: Type<string, string> = extendType(ExistingPath, {
    displayName: "dir",
    description: "A path to an existing directory on the filesystem",
    async from(path) {
        const stat = statSync(path)

        if (stat.isDirectory()) {
            return path
        }

        throw new Error(`Path '${path}' is not a directory`)
    },
})

export const File: Type<string, string> = extendType(ExistingPath, {
    displayName: "file",
    description: "A path to an existing file on the filesystem",
    async from(path) {
        const stat = statSync(path)

        if (stat.isFile()) {
            return path
        }

        throw new Error(`Path '${path}' is not a file`)
    },
})
