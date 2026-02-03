import { scope, type Type, type } from "arktype"

import type { Plugin } from "../plugin/plugin.js"
import type { InputOptions, OutputOptions } from "rolldown"
import type { BuildOptions } from "esbuild"
import { cwd, exit } from "node:process"
import { join } from "node:path"
import { existsSync } from "../util/fs.js"
import { logger } from "../logger.js"
import type { IncomingMessage } from "node:http"
import type { ServePluginConfig } from "../builtins/serve/config.js"

export type CliConfigs = (typeof configParser)["infer"]

export const defaultConfig: Config = {
    entrypoints: [],
    formats: ["esm" as const],
    watch: {
        excludeGlobs: [],
    },
    bundle: false,
    externals: [],
    outdir: "dist",
    serve: false,
    // platform: "node",
}

const servePluginConfigParser = scope({
    proxy: {
        target: "string",
        "bypass?": "Function" as never as Type<(req: IncomingMessage) => boolean>,
    },
    plugin: {
        "host?": "string",
        "port?": "number",
        "proxy?": "Record<string, proxy>",
        "entrypoint?": "string",
        "reload?": "boolean",
        "nodeArgs?": "string[]",
    },
}).export().plugin

export const configParser: Type<Config> = type({
    "root?": "string",

    // type: "'app' | 'lib'",
    "platform?": "'node' | 'browser' | 'neutral'",

    "entrypoints?": "string[]",
    "formats?": "('cjs' | 'esm')[]",

    "serve?": "boolean",

    "preset?": "'nodeApp' | 'webApp' | 'lib' | 'nodeLib' | 'webLib'",

    "release?": "boolean",

    "charset?": "'ascii' | 'utf8'",

    "sourcemap?": "'linked' | 'external' | 'inline' | 'both'",

    "mainFields?": "string[]",

    "backend?": "'esbuild' | 'rolldown'",

    "bundle?": "boolean",
    "externals?": "string[]",
    "outdir?": "string",

    // "platform?": "'browser' | 'node' | 'neutral'",

    "watch?": type({
        "excludeGlobs?": "string[]",
        "additionalGlobs?": "string[]",
    }),
    "minify?": "boolean",
    "conditions?": "string[]",

    "server?": servePluginConfigParser,

    "plugins?": type("Function").array().or("undefined") as Type<Plugin[] | undefined>,

    "esbuild?": type("object").or("undefined") as Type<BuildOptions | undefined>,
    "rolldown?": type({
        "input?": type("object").or("undefined"),
        "output?": type("object").or("undefined"),
    }).or("undefined") as Type<
        { input?: InputOptions | undefined; output?: OutputOptions | undefined } | undefined
    >,
}) as Type<Config>

export interface Config {
    root?: string

    serve?: boolean
    preset?: "nodeApp" | "webApp" | "lib" | "nodeLib" | "webLib"
    release?: boolean
    charset?: "ascii" | "utf8"
    sourcemap?: "linked" | "external" | "inline" | "both"
    mainFields?: string[]
    entrypoints: string[]
    formats: ("cjs" | "esm")[]
    bundle?: boolean
    backend?: "esbuild" | "rolldown"
    externals?: string[]
    outdir?: string
    platform?: "browser" | "node" | "neutral" // "browser" | "node" | "neutral"
    watch?: {
        excludeGlobs?: string[]
        additionalGlobs?: string[]
    }
    minify?: boolean
    conditions?: string[]

    //
    server?: ServePluginConfig

    //
    plugins?: Plugin[] | undefined
    //
    esbuild?: BuildOptions | undefined
    rolldown?: {
        input?: InputOptions | undefined
        output?: OutputOptions | undefined
    }
}

export interface NormalizedConfig extends Config {
    root: string
}

export const normalizeConfig = (config: Config): NormalizedConfig => {
    const root = config.root ?? cwd()

    return {
        ...config,
        root,
        entrypoints: getEntrypoints(config, root),
    }
}

const getEntrypoints = (config: Config, rootDir: string): Array<string> => {
    if (!!config.entrypoints && config.entrypoints.length !== 0) {
        return config.entrypoints
    }

    const htmlFile = join(rootDir, "index.html")

    if (!existsSync(htmlFile)) {
        logger.error(`Could not locate an index.html file in the defined root '${rootDir}'`)
        exit(1)
    }

    return [htmlFile]
}
