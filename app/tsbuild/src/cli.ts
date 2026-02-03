#!/usr/bin/env -S node --no-warnings=ExperimentalWarning
import "@total-typescript/ts-reset"

import { lt } from "semver"
import { binary, run, subcommands } from "cmd-ts"

import { logger } from "./lib/logger.js"
import { build } from "./commands/build.js"
import { dev } from "./commands/dev.js"

const MINIMUM_REQUIRED_NODE_VERSION = "18.0.0" as const

const TSBUILD_VERSION: string = await (async () => {
    try {
        const packageJsonPath = new URL("../package.json", import.meta.url).toString()
        const packageJsonData = await import(packageJsonPath, {
            with: { type: "json" },
        })

        return packageJsonData?.default?.version ?? packageJsonData?.version ?? "unknown"
    } catch (_) {}

    return "unknown"
})()

const main = async () => {
    logger.info(`tsbuild v${TSBUILD_VERSION}`)

    // TODO: This needs to be moved somewhere else.
    process.env["NODE_ENV"] ??= "production"

    if (lt(process.versions.node, MINIMUM_REQUIRED_NODE_VERSION)) {
        throw new Error(
            `Node version ${process.versions.node} is not supported by tsbuild. The minimum supported version is ${MINIMUM_REQUIRED_NODE_VERSION}.`,
        )
    }

    const validEnvironments = ["production", "development", "test"]

    if (!validEnvironments.includes(process.env["NODE_ENV"] ?? "production")) {
        logger.warn(
            `An unknown 'NODE_ENV' value '${process.env["NODE_ENV"]}' was passed to tsbuild. Supported values are 'production', 'development', and 'test'.`,
        )
    }

    const program = subcommands({
        name: "tsbuild",
        version: TSBUILD_VERSION,
        cmds: {
            build,
            dev,
        },
    })

    await run(binary(program), process.argv)
}

await main()
