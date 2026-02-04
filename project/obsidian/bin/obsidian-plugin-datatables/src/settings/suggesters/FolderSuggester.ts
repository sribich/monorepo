import { TAbstractFile, TFolder } from "obsidian"

import { InputSuggester } from "./suggester"

export class FolderSuggest extends InputSuggester<TFolder> {
    private possibilities: TFolder[] = []

    getPossibilities() {
        if (!this.possibilities) {
            this.possibilities = app.vault
                .getAllLoadedFiles()
                .reduce((folders: TFolder[], file: TAbstractFile) => {
                    return file instanceof TFolder ? [...folders, file] : folders
                }, [])
        }

        return this.possibilities
    }

    getSuggestions(input: string): TFolder[] {
        const lowercaseInput = input.toLowerCase()

        return this.getPossibilities().filter((it) =>
            it.path.toLowerCase().contains(lowercaseInput),
        )
    }

    renderSuggestion(file: TFolder, el: HTMLElement): void {
        el.setText(file.path)
    }

    selectSuggestion(file: TFolder): void {
        this.inputEl.value = file.path
        this.inputEl.trigger("input")

        this.close()
    }
}
