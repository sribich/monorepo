export const documentFilters = {
    isMarkdown(path: string): boolean {
        const lowercasePath = path.toLocaleLowerCase()

        return lowercasePath.endsWith(".md") || lowercasePath.endsWith(".markdown")
    },
}
