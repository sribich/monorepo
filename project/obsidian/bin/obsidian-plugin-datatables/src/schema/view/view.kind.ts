export const viewKindIterator = [
    "table",
    "board",
    "timeline",
    "calendar",
    "list",
    "gallery",
] as const

export type ViewKind = (typeof viewKindIterator)[number]
