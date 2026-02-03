import type { BuildOptions, Plugin as EsbuildPlugin } from "esbuild"

import type { Config } from "../config/parser.js"

import type { RolldownPlugin } from "rolldown"
import type { RunnerContext } from "../context/runner.js"
import type { Document } from "../builtins/html/plugin.js"

export type Plugin = (
    context: RunnerContext,
) => Promise<ActivatedPlugin | null> | ActivatedPlugin | null

export interface ActivatedPlugin {
    initialise?: () => void | Promise<void>
    terminate?: () => void | Promise<void>

    esbuild?: EsbuildPlugin | ((isMaster: boolean) => EsbuildPlugin | undefined) | undefined
    rolldown?: RolldownPlugin | (() => RolldownPlugin | undefined) | undefined

    // TODO: We should revisit this.
    runForSubBuilds?: boolean

    /**
     * Modify the config before initialization.
     */
    modifyConfig?: (config: Config) => Promise<void> | void

    transformIndexHtml?: (document: Document) => void | Promise<void>

    /**
     * Runs on startup before the first compilation.
     */
    onStartup?: () => void | Promise<void>

    /**
     * Runs before application shutdown.
     */
    onShutdown?: () => void | Promise<void>

    /**
     * This is a special callback. Plugins at this stage should not assume
     * that the underlying filesystem is configured properly for output.
     *
     * This is mainly for the necessarily plugins to set up the system.
     */
    preBuild?: () => void | Promise<void>

    /**
     * A callback which runs a single time before a build, even if multiple
     * formats are being output.
     */
    onBuildStart?: () => void | Promise<void>
    /**
     * A callback which runs a single time, concurrently with the underlying
     * esbuild build.
     */
    onBuild?: () => void | Promise<void>
    /**
     * A callback which runs a single time after a build, even if multiple
     * formats are being output.
     */
    onBuildEnd?: () => void | Promise<void>

    /**
     * This is a special callback. Plugins at this stage should not assume
     * that the underlying filesystem is configured properly for output.
     *
     * This is mainly for the necessarily plugins to tear down the system.
     */
    postBuild?: () => void | Promise<void>
}
