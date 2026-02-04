import { TFile } from "obsidian"
import pluralize from "pluralize"

export const getFile = (file: TFile | string): TFile | null => {
    if (file instanceof TFile) {
        return file
    }

    const maybeFile = app.vault.getAbstractFileByPath(file)

    if (maybeFile instanceof TFile) {
        return maybeFile
    }

    return null
}

export const getPrettyTagName = (() => {
    const cache = new Map<string, string>()

    return (name: string): string => {
        if (cache.has(name)) {
            return cache.get(name) as string
        }

        const words = name.slice(1).split(/[-_]/g)

        const result = words
            .map((it, index) => {
                let word = it[0]?.toUpperCase() + it.slice(1)

                if (index === words.length - 1) {
                    word = pluralize(word)
                }

                return word.trim()
            })
            .join(" ")
            .trim()

        cache.set(name, result)

        return result
    }
})()
