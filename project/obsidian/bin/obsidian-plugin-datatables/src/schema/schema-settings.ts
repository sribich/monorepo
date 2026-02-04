import { Setting } from "obsidian"

import { SettingProvider } from "../settings/providers"
import { FolderSuggest } from "../settings/suggesters/FolderSuggester"

export interface SchemaSettings {
    itemsDir: string
    templateDir: string
    folder: string
    datatablesFile: string
}

export class SchemaSettingProvider extends SettingProvider<SchemaSettings> {
    public override getNamespace(): string {
        return "schema"
    }

    public override getDefaults(): SchemaSettings {
        return {
            itemsDir: "Items",
            templateDir: "Templates",
            folder: "/",
            datatablesFile: "DATATABLES.json",
        }
    }

    getSettings(settings: SchemaSettings, containerEl: HTMLElement, save: () => Promise<void>) {
        new Setting(containerEl)
            .setName("Template folder location")
            .setDesc("Template files for tables will be stored in this location")
            .addSearch((component) => {
                const suggest = new FolderSuggest(component.inputEl)

                component
                    .setPlaceholder("Example: Templates")
                    .setValue(settings.templateDir)
                    .onChange(async (value) => {
                        if (suggest.getPossibilities().some((it) => it.path === value)) {
                            settings.templateDir = value
                            await save()
                        }
                    })
            })
        new Setting(containerEl)
            .setName("Instantiated templates folder location")
            .setDesc("Files created from templates will be initially stored in this location.")
            .addSearch((component) => {
                const suggest = new FolderSuggest(component.inputEl)

                component
                    .setPlaceholder("Example: Items")
                    .setValue(settings.itemsDir)
                    .onChange(async (value) => {
                        if (suggest.getPossibilities().some((it) => it.path === value)) {
                            settings.itemsDir = value
                            await save()
                        }
                    })
            })
        new Setting(containerEl)
            .setName("Schema folder location")
            .setDesc("Files will be created in this folder to save schema metadata")
            .addSearch((component) => {
                const suggest = new FolderSuggest(component.inputEl)

                component
                    .setPlaceholder("Example: Schema")
                    .setValue(settings.folder)
                    .onChange(async (value) => {
                        if (suggest.getPossibilities().some((it) => it.path === value)) {
                            settings.folder = value
                            await save()
                        }
                    })
            })
        const _component = new Setting(containerEl)
            .setName("Database schema file name")
            .setDesc(
                "The file containing the schemas for every database within the vault. You are responsible for moving the schema files if you change this.",
            )
            .addText((component) => {
                const originalValue = settings.datatablesFile

                component
                    .setPlaceholder("Example: DATABASES")
                    .setValue(settings.datatablesFile)
                    .onChange(async (value) => {
                        const control = _component.controlEl

                        const file = app.vault.getAbstractFileByPath(
                            `${settings.folder}/${value}.json`,
                        )

                        if (!file || value === originalValue) {
                            control.removeClass("setting-error")
                            if (control.children[1]) {
                                control.removeChild(control.children[1])
                            }

                            settings.datatablesFile = value
                            await save()
                            return
                        }

                        control.addClass("setting-error")
                        if (!control.children[1]) {
                            control.appendChild(
                                createDiv({
                                    text: "File already exists",
                                    cls: "setting-error-msg",
                                }),
                            )
                        }
                    })
            })
    }
}
