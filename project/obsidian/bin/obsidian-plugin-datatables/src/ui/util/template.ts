/*
import { App, TFile, TFolder } from "obsidian"

export type TemplateButtonProps = {
    text: string
    template: string
    folder: string

    right?: boolean
}

export const TemplateButton = (props: TemplateButtonProps) => {
    const templaterPlugin = getPlugin("templater-obsidian")

    const onClick = () => {
        templaterPlugin.templater.create_new_note_from_template(
            app.vault.getAbstractFileByPath(`Templates/${props.template}.md`) as TFile,
            app.vault.getAbstractFileByPath(props.folder) as TFolder,
        )
    }

    return (
        <div className="flow-root">
            <button className={props.right ? "float-right" : ""} onClick={onClick}>
                {props.text}
            </button>
        </div>
    )
}

export const getPlugin = <TPlugin extends keyof App["plugins"]["plugins"]>(
    pluginName: TPlugin,
): App["plugins"]["plugins"][TPlugin] => {
    const app = window.app

    const plugins = app.plugins.plugins as App["plugins"]["plugins"]

    if (pluginName in plugins) {
        return plugins[pluginName]
    }

    throw new Error(`Plugin "${pluginName}" not found.`)
}
*/
