import type { Immutable } from "@sribich/ts-utils"
import type { CachedMetadata } from "obsidian"

import { getProperty } from "../../../schema/property/property"
import type { DtSchema } from "../../../schema/schema-definition"
import { type Document, type ReadMarkdownDocument } from "../../document"

const FRONTMATTER_EXCLUDE_FIELDS = [
    "tags",
    "tag",
    "aliases",
    "alias",
    "banner",
    "banner_y",
    "position",
]

export const parseObsidianMarkdown = async (
    path: string,
    readDocument: ReadMarkdownDocument,
    schema: Immutable<DtSchema>,
): Promise<Document> => {
    const { metadata } = readDocument

    if (!metadata) {
        throw new Error(`Missing metadata when parsing obsidian markdown file: ${path}`)
    }

    const tags = parseTags(metadata)
    const { fields, renderedFields } = parseFields(metadata, schema)

    // const aliases = MarkdownPage.parseAliases(metadata)
    // const links = MarkdownPage.parseLinks(metadata)

    // const aliases = new Set<string>()
    // const fields = new Map<string, string>()
    // let links = []

    // (metadata.tags || []).forEach((it) => tags.add(it))

    // console.log(renderedTables)

    return {
        kind: readDocument.kind,
        path,
        data: {
            tags,
            fields,
            renderedFields,
            // aliases,
            // links,
        },
    }
}

const parseTags = (metadata: CachedMetadata): string[] => {
    const tags = new Set<string>()

    const addTag = (tag: string) => {
        tag = tag.toLocaleLowerCase()
        tags.add(tag.startsWith("#") ? tag : `#${tag}`)
    }

    for (const tag of metadata.tags || []) {
        addTag(tag.tag)
    }

    for (const [key, value] of Object.entries(metadata.frontmatter || [])) {
        if (["tag", "tags"].includes(key.toLocaleLowerCase())) {
            getArrayFromListOrString(value).map(addTag)
        }
    }

    return Array.from(tags)
}

const parseFields = (
    metadata: CachedMetadata,
    schema: Immutable<DtSchema>,
): { fields: Record<string, string[]>; renderedFields: Record<string, unknown> } => {
    const fields = new Map<string, string[]>()
    const renderedFields = {} as Record<string, unknown>

    for (const [key, value] of Object.entries(metadata.frontmatter || {})) {
        if (FRONTMATTER_EXCLUDE_FIELDS.includes(key.toLocaleLowerCase())) {
            continue
        }

        // CHECK IF KEY IS IN SCHEMA
        if (key in schema.tables) {
            const table = {} as Record<string, unknown>

            for (const property of schema.tables[key]?.properties || []) {
                if (!(property.name in value)) {
                    continue
                }

                const propertySchema = getProperty(property.kind)
                const { data, problems } = propertySchema.field.type(value[property.name])

                if (problems) {
                    if (property.name === "Do Date") {
                        console.log(problems.summary)
                    }
                    // console.warn("Error parsing value")
                    continue
                }

                table[property.uuid] = data
            }

            renderedFields[key] = table
        }

        fields.set(key.toLocaleLowerCase(), value)
    }

    // We no longer care about content based fields, but we're keeping this
    // for the time being since it's a useful reference.
    /*
    const regex =
        /[[(]?([^\n\r([]*)::[ ]*((?:[^\r\n)\]]*(?:(?<=\[.*::.*)(?<=\[\[[^\]]*)(?:(?:\]\])(?=.*\])))?(?:(?<!\[.*::.*)(?<=\[\[[^\]]*)(?:(?:\]\])))?)*)/g

    let iter: RegExpExecArray | null

    while ((iter = regex.exec(content)) !== null) {
        fields.set(iter[1].trim(), iter[2].trim())
    }
    */

    return {
        fields: Object.fromEntries(fields),
        renderedFields,
    }
}

const getArrayFromListOrString = (list: string | string[]): string[] => {
    if (!list) {
        return []
    }

    if (Array.isArray(list)) {
        return list
            .filter(Boolean)
            .map(getArrayFromListOrString)
            .reduce((acc, cur) => acc.concat(cur), [])
    }

    return list
        .toString()
        .split(/[,\s]+/)
        .filter(Boolean)
        .map((it) => it.trim())
        .filter((it) => it.length > 0)
}

/*
    private static parseAliases(metadata: CachedMetadata): Immutable<string[]> {
        const aliases = new Set<string>()

        for (const [key, value] of Object.entries(metadata.frontmatter || [])) {
            if (["alias", "aliases"].includes(key.toLocaleLowerCase())) {
                getArrayFromListOrString(value).map((it) => aliases.add(it))
            }
        }

        return Array.from(aliases)
    }

    private static parseLinks(metadata: CachedMetadata): Immutable<Link[]> {
        const links: Link[] = []

        for (const link of metadata.links || []) {
            links.push(Link.infer(link, false))
        }

        for (const link of metadata.embeds || []) {
            links.push(Link.infer(link, true))
        }

        return links
    }
*/

/*
import type { Immutable } from "@sribich/ts-utils"
import { type LinkCache } from "obsidian"

export abstract class ObjectSerializable {
    abstract toObject(): Record<string, unknown>

    static fromObject(_data: Record<string, unknown>) {
        throw new Error("Not implemented")
    }
}

export type PageInfo<TFields> = {
    tags: string[]
    fields: Record<string, TFields>
}

export class Page<TFields = Record<string, unknown>> extends ObjectSerializable {
    private _name: string

    constructor(
        public path: string,
        public info: Immutable<PageInfo<TFields>>,
    ) {
        super()
    }

    override toObject(): Record<string, unknown> {
        return {
            path: this.path,
            info: this.info,
        }
    }

    /**
     * TODO: Fix
     *
    static override fromObject(data: Record<string, unknown>): Page {
        return new Page(data["path"] as string, data["info"] as Immutable<PageInfo>)
    }

    /**
     * TODO: Write a proper validator for this, probably using arktype
     *
    static fromSerialized(data: unknown): Page {
        return new Page(data.path, data.info)
    }

    get name() {
        if (!this._name) {
            let path = this.path

            if (path.includes("/")) {
                path = path.substring(path.lastIndexOf("/") + 1)
            }

            if (path.endsWith(".md")) {
                path = path.substring(0, path.length - 3)
            }

            this._name = path
        }

        return this._name
    }
}

export class Link extends ObjectSerializable {
    private constructor(
        /** The file path of the link *
        public readonly path: string,
        /** The block or header that this link points to within the $path *
        public readonly anchor: string | undefined,
        /** The unique display name set using [[path|display]] *
        public readonly displayName: string | undefined,
        /** Whether the link is embedded ![[...]] *
        public readonly isEmbed: boolean,
        /** The type of this link *
        public readonly type: "file" | "header" | "block",
    ) {
        super()
    }

    override toObject(): Record<string, unknown> {
        return {
            path: this.path,
            anchor: this.anchor,
            displayName: this.displayName,
            isEmbed: this.isEmbed,
            type: this.type,
        }
    }

    static override fromObject(data: Record<string, unknown>): Link {
        return new Link(
            data["path"] as string,
            data["anchor"] as string | undefined,
            data["displayName"] as string | undefined,
            data["isEmbed"] as boolean,
            data["type"] as "file" | "header" | "block",
        )
    }

    public static infer(link: LinkCache, isEmbed: boolean): Link {
        if (link.link.includes("#^")) {
            return Link.block(link, isEmbed)
        } else if (link.link.includes("#")) {
            return Link.header(link, isEmbed)
        } else {
            return Link.file(link, isEmbed)
        }
    }

    private static block(link: LinkCache, isEmbed: boolean): Link {
        const [page, block] = link.link.split("#^")

        return new Link(page as string, block, link.displayText, isEmbed, "block")
    }

    private static file(link: LinkCache, isEmbed: boolean): Link {
        return new Link(link.link, undefined, link.displayText, isEmbed, "file")
    }

    private static header(link: LinkCache, isEmbed: boolean): Link {
        const [page, header] = link.link.split("#")

        return new Link(page as string, header, link.displayText, isEmbed, "header")
    }
}
*/
