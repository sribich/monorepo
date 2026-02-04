export const loadReferences = (content: string): string[] => {
    const regex = /\[\[([^\]]*)\]\]/g

    let page
    const pages = []

    while ((page = regex.exec(content)) !== null) {
        if (typeof page[1] === "string") {
            pages.push(page[1])
        }
    }

    return pages
}

export const saveReferences = (values: string[]): string => {
    return values.map((it) => `[[${it}]]`).join(" ")
}
