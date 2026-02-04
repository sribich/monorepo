import { Plugin, PluginSettingTab } from "obsidian"

import { SchemaSettingProvider, type SchemaSettings } from "../schema/schema-settings"

export class SettingsManager extends PluginSettingTab {
    private displays: Record<
        string,
        (settings: SchemaSettings, containerEl: HTMLElement, save: () => Promise<void>) => void
    > = {}
    private settings: Record<string, SchemaSettings> = {}

    private constructor(private readonly plugin: Plugin) {
        super(plugin.app, plugin)
    }

    static async create(plugin: Plugin) {
        const manager = new SettingsManager(plugin)

        const defaults = {} as Record<string, SchemaSettings>
        const displays = {} as Record<
            string,
            (settings: SchemaSettings, containerEl: HTMLElement, save: () => Promise<void>) => void
        >

        const settingProviders = [new SchemaSettingProvider()]

        for (const provider of settingProviders) {
            const namespace = provider.getNamespace()

            defaults[namespace] = provider.getDefaults()
            displays[namespace] = (
                settings: SchemaSettings,
                containerEl: HTMLElement,
                save: () => Promise<void>,
            ) => provider.getSettings(settings, containerEl, save)
        }

        manager.displays = displays
        manager.settings = Object.assign({}, defaults, (await plugin.loadData()) ?? {})

        plugin.addSettingTab(manager)

        return manager
    }

    getSettings<T>(namespace: string): T {
        const settings = this.settings[namespace] as T

        if (!settings) {
            throw new Error(`Settings for namespace ${namespace} not found`)
        }

        return settings
    }

    private async save() {
        await this.plugin.saveData(this.settings)
    }

    override display() {
        this.containerEl.empty()

        for (const [namespace, display] of Object.entries(this.displays)) {
            this.containerEl.createEl("h2", { text: namespace })

            const currentSettings = this.settings[namespace]

            if (currentSettings) {
                display(currentSettings, this.containerEl, this.save.bind(this))
            }
        }
    }
}
