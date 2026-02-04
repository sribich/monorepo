import { Logger } from "@basalt/obsidian-logger"
import type { Immutable } from "@sribich/ts-utils"

import type { DtSchema } from "../schema/schema-definition"
import type { SchemaLoader } from "../schema/schema-loader"
import { ExtendedSet } from "../util/set"
import type { File, Vault } from "../vault/vault"
import { type Document } from "./document"
import { TagIndex } from "./indexes/tag"
import { documentFilters } from "./loader/filters"
import { Loader } from "./loader/loader"

/**
 * The index is responsible for loading and indexing all files within
 * the vault.
 *
 * All inputs are normalized into a common `Document` format, which is
 * what we process to generate the index.
 *
 * When constructed, the index will load all files in the vault and
 * index them. It will also listen for changes to the vault and update
 * the index accordingly.
 */
export class Index {
    private logger = new Logger(Index.name)

    /**
     * The loader is responsible for loading generic file types and
     * normalizing them into the common `Document` format.
     */
    private loader: Loader

    /**
     * The current revision of the index. This is incremented for every
     * change to the index, and is used to determine whether or not any
     * component that depends on the index needs to be updated.
     */
    private _revision = 0

    /**
     * The set of all documents that have been loaded and indexed.
     *
     * Documents are tracked by their filesystem path and contains the
     * normalized document data.
     */
    private _documents = new Map<string, Document>()

    /**
     * A reversed index of all tags to the document paths that contain
     * them.
     */
    private _tags = new TagIndex()

    public get documents(): Immutable<Map<string, Document>> {
        return this._documents as Immutable<Map<string, Document>>
    }

    public get revision(): number {
        return this._revision
    }

    public get tags(): Immutable<TagIndex> {
        return this._tags
    }

    private constructor(
        private readonly vault: Vault,
        schema: Immutable<DtSchema>,
    ) {
        this.loader = new Loader(this.vault, schema)
    }

    static async create(vault: Vault, schema: Immutable<DtSchema>): Promise<Index> {
        const index = new Index(vault, schema)

        await index.initialise()

        return index
    }

    /**
     * Returns the associated Page object for the given path.
     */
    page(path: string, sourcePath?: string): Immutable<Document> | undefined {
        const realPath = app.metadataCache.getFirstLinkpathDest(path, sourcePath ?? "")

        if (!realPath) {
            return undefined
        }

        return this.documents.get(realPath.path)
    }

    getDocuments(tag: string) {
        const pages = this.tags.getPages(tag)

        return [...pages].map((page) => this.page(page)).filter(Boolean)
    }

    private async initialise() {
        this._documents = new Map()
        this._tags = new TagIndex()

        const loadStart = Date.now()

        const vaultFiles = await this.vault.getMarkdownFiles()
        const loadResults = await Promise.all(vaultFiles.map((file) => this.load(file)))

        const { skipped } = loadResults.reduce(
            (acc, cur) => ({
                skipped: acc.skipped + +!!cur.skipped,
            }),
            { skipped: 0 },
        )

        const deltaTime = (Date.now() - loadStart) / 1000.0

        this.logger.log(
            `Indexed ${loadResults.length} files in ${deltaTime} seconds (${skipped} skipped).`,
        )
    }

    /**
     * @internal
     */
    async load(file: File): Promise<{ skipped: boolean }> {
        if (!documentFilters.isMarkdown(file.path)) {
            return { skipped: true }
        }

        // ADD A LOCK WRAPPER

        const document = await this.loader.load({
            kind: "markdown",
            path: file.path,
            file,
        })

        this._documents.set(file.path, document)
        this._tags.set(file.path, new ExtendedSet(document.data.tags))

        // this.markDirty({ type: "load", file, revision: this.revision })
        this.markDirty()

        return { skipped: false }
    }

    private markDirty(/*event: IndexEvent*/): void {
        this._revision += 1

        app.workspace.trigger("datatables:index:changed", {
            // ...event,
            revision: this._revision,
        })
    }

    public async rename(_file: File, _oldPath: string): Promise<void> {
        throw new Error("Not implemented")
    }

    public async delete(_file: File): Promise<void> {
        throw new Error("Not implemented")
    }

    /*
    private async rename(file: TAbstractFile, oldPath: string): Promise<void> {
        if (!(file instanceof TFile) || !pageFilters.isMarkdown(file)) {
            return
        }

        if (this._pages[oldPath]) {
            const oldPage = this._pages[oldPath]

            if (oldPage) {
                oldPage.path = file.path

                this._pages[file.path] = oldPage
            }

            delete this._pages[oldPath]
        }

        this._tags.rename(oldPath, file.path)

        this.markDirty({ type: "rename", file, oldPath, revision: this.revision })
    }

    private async delete(file: TAbstractFile): Promise<void> {
        if (!(file instanceof TFile) || !pageFilters.isMarkdown(file)) {
            return
        }

        delete this._pages[file.path]

        this._tags.delete(file.path)

        this.markDirty({ type: "delete", file, revision: this.revision })
    }



    

    

    /**
     * Returns a list of Page objects that match the given query.
     *
    queryPages(query: string, sourcePath?: string): Immutable<Page>[] {
        return this.pagePaths(query)
            .map((it) => this.page(it, sourcePath))
            .filter(Boolean)
    }

    /**
     * Returns a list of filesystem paths to pages that match the given query.
     *
    pagePaths(query: string, sourcePath?: string): string[] {
        let source: Source

        try {
            source = parseSource(query)
        } catch (error) {
            throw new Error(`Unable to parse query: ${error}`)
        }

        return Array.from(this.match(source, sourcePath).unwrap())
    }

    match(source: Source, _sourcePath?: string): Result<Immutable<string[]>, Error> {
        if (source.type !== "tag") {
            return new Err(new Error(`Unsupported source type: ${source.type}`))
        }

        return new Ok(this.tags.getPages(source.tag))
    }
    */
}
