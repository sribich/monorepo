import type { Plugin as EsbuildPlugin } from "esbuild"
// import { pluginList } from "../../plugin/list.js"
import type { ActivatedPlugin, Plugin } from "../plugin/plugin.js"
import type { RolldownPlugin } from "rolldown"
import type { Config } from "../config/parser.js"
import { Context } from "./context.js"
import type { RunnerContext } from "./runner.js"
import { ignoreOptionalDependenciesPlugin } from "../plugin/ignore-optional-deps/plugin.js"
import { htmlPlugin, type Document } from "../builtins/html/plugin.js"
import { servePlugin } from "../builtins/serve/plugin.js"

export class PluginContext extends Context {
    private context: RunnerContext

    private plugins: ActivatedPlugin[] = []

    constructor(context: RunnerContext) {
        super()

        this.context = context
    }

    /**
     * Initializes the list of plugins from the application context.
     */
    override async initialise(): Promise<void> {
        const configContext = this.context.config

        const realList = configContext.plugins ?? []

        realList.push(ignoreOptionalDependenciesPlugin)
        realList.push(htmlPlugin)
        realList.push(servePlugin)

        const pluginPromises = realList.map((plugin) => plugin(this.context))

        const plugins = await Promise.all(pluginPromises)

        this.plugins = plugins.filter(Boolean)
    }

    override async terminate(): Promise<void> {
        await Promise.allSettled(this.plugins.map((plugin) => plugin.terminate?.()))
    }

    getSubbuildPlugins(): Array<EsbuildPlugin> {
        return this.plugins
            .filter((it) => !!it.runForSubBuilds)
            .map((it) => it.esbuild)
            .filter(Boolean)
            .map((plugin) => (typeof plugin === "function" ? plugin(false) : plugin))
            .filter(Boolean)
    }

    getEsbuildPlugins(isMaster: boolean): Array<EsbuildPlugin> {
        return this.plugins
            .map((plugin) => plugin.esbuild)
            .filter(Boolean)
            .map((plugin) => (typeof plugin === "function" ? plugin(isMaster) : plugin))
            .filter(Boolean)
    }

    getRolldownPlugins(): Array<RolldownPlugin> {
        return this.plugins
            .map((plugin) => plugin.rolldown)
            .filter(Boolean)
            .map((plugin) => (typeof plugin === "function" ? plugin() : plugin))
            .filter(Boolean)
    }

    async transformIndexHtml(document: Document): Promise<void> {
        for (const plugin of this.plugins) {
            await plugin.transformIndexHtml?.(document)
        }
    }

    async modifyConfig(initialConfig: Config): Promise<Config> {
        for (const plugin of this.plugins) {
            await plugin.modifyConfig?.(initialConfig)
        }

        return initialConfig
    }

    async onStartup(): Promise<Array<void>> {
        const pluginPromises = []

        for (const plugin of this.plugins) {
            pluginPromises.push(plugin.onStartup?.())
        }

        return Promise.all(pluginPromises)
    }

    async onShutdown(): Promise<Array<void>> {
        const pluginPromises = []

        for (const plugin of this.plugins) {
            pluginPromises.push(plugin.onShutdown?.())
        }

        return Promise.all(pluginPromises)
    }

    async preBuild(): Promise<Array<void>> {
        const pluginPromises = []

        for (const plugin of this.plugins) {
            pluginPromises.push(plugin.preBuild?.())
        }

        return Promise.all(pluginPromises)
    }

    async onBuildStart(): Promise<Array<void>> {
        const pluginPromises = []

        for (const plugin of this.plugins) {
            pluginPromises.push(plugin.onBuildStart?.())
        }

        return Promise.all(pluginPromises)
    }

    async onBuild(): Promise<Array<void>> {
        const pluginPromises = []

        for (const plugin of this.plugins) {
            pluginPromises.push(plugin.onBuild?.())
        }

        return Promise.all(pluginPromises)
    }

    async onBuildEnd(): Promise<Array<void>> {
        const pluginPromises = []

        for (const plugin of this.plugins) {
            pluginPromises.push(plugin.onBuildEnd?.())
        }

        return Promise.all(pluginPromises)
    }

    async postBuild(): Promise<Array<void>> {
        const pluginPromises = []

        for (const plugin of this.plugins) {
            pluginPromises.push(plugin.postBuild?.())
        }

        return Promise.all(pluginPromises)
    }
}
