import type { Immutable } from "@sribich/ts-utils"

import type { DtSchema } from "../../schema/schema-definition"
import type { Document, DocumentKind, ReadDocument } from "../document"
import { parseObsidianMarkdown } from "./documents/markdown-obsidian"

export const documentParsers = {
    markdown: parseObsidianMarkdown,
} as const satisfies Record<
    DocumentKind,
    (path: string, readDocument: ReadDocument, schema: Immutable<DtSchema>) => Promise<Document>
>
