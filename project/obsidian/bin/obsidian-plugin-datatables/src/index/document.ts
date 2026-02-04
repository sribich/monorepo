import type { CachedMetadata } from "obsidian"

import type { File } from "../vault/vault"

export type DocumentMetadata = MarkdownDocumentMetadata
export type ReadDocument = ReadMarkdownDocument
export type Document = MarkdownDocument

export type DocumentKind = Document["kind"]
export type DocumentMetadataRepr<T extends DocumentKind> = Extract<DocumentMetadata, { kind: T }>
export type ReadDocumentRepr<T extends DocumentKind> = Extract<ReadDocument, { kind: T }>
export type DocumentRepr<T extends DocumentKind> = Extract<Document, { kind: T }>

////////////////////////////////////////////////////////////////////////////////
/// Markdown
////////////////////////////////////////////////////////////////////////////////
export interface MarkdownDocumentMetadata {
    kind: "markdown"
    path: string
    file: File
}

export interface ReadMarkdownDocument {
    kind: "markdown"
    content: string
    metadata: CachedMetadata | null
}

export interface MarkdownDocument {
    kind: "markdown"
    path: string
    data: {
        tags: string[]
        fields: Record<string, unknown>
        // aliases: string[]
        // links: Link[]

        renderedFields: Record<string, unknown>
    }
}
