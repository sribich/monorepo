import { Logger } from "@basalt/obsidian-logger"
import { darkColors } from "@sribich/fude-theme"
import "@sribich/fude/reset.css"
import "@total-typescript/ts-reset"

import { type } from "arktype"
import { Plugin } from "obsidian"

import { Index } from "./index/index"
import { loadProcessors } from "./processor/processor"
import { SchemaLoader } from "./schema/schema-loader"
import type { SchemaSettings } from "./schema/schema-settings"
import { SettingsManager } from "./settings/settings"
import { SettingsContainer } from "./settings/settings-container"
import "./styles.css"

import type { Vault } from "./vault/vault"
import { ObsidianFile, ObsidianVault } from "./vault/vaults/obsidian"

export default class BasaltDatatablePlugin extends Plugin {
    private logger = new Logger(BasaltDatatablePlugin.name)

    private _vault!: Vault

    private _index!: Index
    private _schema!: SchemaLoader
    private _settings!: SettingsManager

    public get index(): Index {
        return this._index
    }

    public get schema(): SchemaLoader {
        return this._schema
    }

    public get settings(): SettingsManager {
        return this._settings
    }

    public get vault(): Vault {
        return this._vault
    }

    /**
     * The plugin entrypoint.
     */
    override async onload() {
        Logger.setPluginName(this.manifest.id)

        const settingsManager = await SettingsManager.create(this)
        const settings = new SettingsContainer({
            schema: settingsManager.getSettings<SchemaSettings>("schema"),
        })

        this.app.workspace.onLayoutReady(async () => {
            this.configureUi()
            this.processCleanupHooks()

            this._vault = new ObsidianVault(this)

            this._schema = await SchemaLoader.create(this._index, this._vault, settings)
            this._index = await Index.create(this._vault, this._schema.internalGetSchema())

            // @ts-expect-error TODO: We should separate the loader from the schema representation
            //                        so that we can instantiate the loader without the need for
            //                        the index and instead pass the index to a schema manager or
            //                        something.
            this._schema.index = this._index

            loadProcessors({
                plugin: this,
                index: this.index,
                schema: this.schema,
            })

            this.registerEvent(
                this.app.metadataCache.on("resolve", (file) => {
                    this._index.load(new ObsidianFile(file))
                }),
            )
            this.registerEvent(
                this.app.vault.on("rename", (file, oldPath) => {
                    this._index.rename(new ObsidianFile(file), oldPath)
                }),
            )
            this.registerEvent(
                this.app.vault.on("delete", (file) => {
                    this._index.delete(new ObsidianFile(file))
                }),
            )
        })

        this.logger.log(`loaded version ${this.manifest.version}`)
    }

    private configureUi() {
        const currentAttributes = document.documentElement.getAttribute("class")

        if (!currentAttributes?.includes("dark")) {
            document.__dev__dt_dark_mode = currentAttributes ?? ""
        }

        document.documentElement.setAttribute(
            "class",
            `${document.__dev__dt_dark_mode} dark ${/*darkModeClasses.className*/ ""}`,
        )

        document.body.setAttribute(
            "style",
            Object.entries({
                ...Object.fromEntries(
                    document.body
                        .getAttribute("style")
                        ?.split(";")
                        .map((item) => item.split(":", 2)) ?? [],
                ),
                "--bg1": darkColors.background,
                "--bg2": darkColors.background,
                "--bg3": darkColors.backgroundHover,
            })
                .map(([key, value]) => `${key}: ${value}`)
                .join(";"),
        )
    }

    /**
     * Processes all outstanding cleanup hooks on plugin reload.
     *
     * I use an edited version of obsidian that allows force-reloading of
     * markdown code-block processors.
     *
     * This allows me to quickly make UI changes in development without
     * needing to manually cause the component to re-render.
     */
    private processCleanupHooks() {
        /// We need to clone the array because active hooks will re-add
        /// themselves every time they are run while the component is
        /// still loaded, causing an infinite loop.
        const hooks = [...(document["__dev__dt_cleanup_hooks"] ?? [])]

        for (const hook of hooks) {
            hook()
        }

        document["__dev__dt_cleanup_hooks"] = []
    }
}
