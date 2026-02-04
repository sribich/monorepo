import type { ForceMutable, Immutable } from "@sribich/ts-utils"
import { randomUUID } from "crypto"

import type { Document } from "../index/document"
import type { SettingsContainer } from "../settings/settings-container"
import { isObject } from "../util/primitive"
import type { File, Vault } from "../vault/vault"
import type { ObsidianFile } from "../vault/vaults/obsidian"
import { type PropertyFieldRepr, type PropertyRepr, getProperty } from "./property/property"
import type { PropertyKind } from "./property/property-kind"
import type { PropertySchema } from "./property/property-schema"
import type { SchemaLoader } from "./schema-loader"
import type { TableSchema } from "./table/table-schema"
import type { TemplateSchema } from "./template/template-schema"
import { getView } from "./view/view"
import type { ViewSchema } from "./view/view-schema"
import type { ViewKind } from "./view/view.kind"

/**
 * TODO: Basically everything here needs to create _NEW_ objects when making
 *       modifications to the schema so that things are not inadvertently kept
 *       and modified incorrectly.
 */
export class Schema {
    public readonly document: DocumentSchema
    public readonly property: PropertySchema2
    public readonly view: ViewSchema2

    constructor(
        public readonly tableName: string,
        private _table: TableSchema,

        private loader: SchemaLoader,
        private vault: Vault,
        private settings: SettingsContainer,
    ) {
        this.document = new DocumentSchema(tableName, _table, loader, this)
        this.property = new PropertySchema2(tableName, _table, loader, this)
        this.view = new ViewSchema2(tableName, _table, loader, this)

        // console.log(this)
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Create
    ////////////////////////////////////////////////////////////////////////////
    async createTemplate(): Promise<void> {
        const templatePath = await this.vault.getFile(this.settings.schema.templateDir)

        if (!templatePath || !templatePath?.isDirectory()) {
            throw new Error(
                `Unable to create template. The template directory (${this.settings.schema.templateDir}) does not exist in the vault. This directory must exist before templates can be created.`,
            )
        }

        if (!this._table.templates) {
            this._table.templates = {
                options: [],
            }
        }

        let templateFile: File | undefined

        await this.loader.vault.withLock(async () => {
            const templateUuid = randomUUID()

            templateFile = await this.vault.createEmptyMarkdownFile(
                templatePath,
                `${templateUuid}.md`,
            )

            this._table.templates = {
                ...this._table.templates,
                options: [
                    ...(this._table.templates?.options || []),
                    {
                        uuid: templateUuid,
                        name: templateUuid.split("-")[0] ?? "",
                        path: templateFile.path,
                    },
                ],
            }

            await this.loader.persist()
        })

        // We do not want to test editor-specific functionality
        if (!process.env["IN_TEST_RUNNER"]) {
            const activeLeaf = app.workspace.getLeaf(false)

            if (!activeLeaf || !templateFile) {
                return
            }

            await activeLeaf.openFile((templateFile as ObsidianFile).asFile(), {
                state: {
                    mode: "source",
                },
            })
        }
    }

    /**
     * TODO: Document
     * TODO: Migrate to use vault
     * TODO: Test
     */
    async instantiateTemplate(templateUuid: string): Promise<void> {
        const templates = this.getTemplates()
        const template = templates.options.find((template) => template.uuid === templateUuid)

        if (!template) {
            // TODO: Warning? Alert?
            return
        }

        const itemsDir = await this.vault.getFile(this.settings.schema.itemsDir)
        const templateFile = await this.vault.getFile(template.path)

        if (!itemsDir || !itemsDir.isDirectory()) {
            throw new Error(
                `Unable to instantiate template. The instantiation directory does not exist: ${this.settings.schema.itemsDir}`,
            )
        }

        if (!templateFile || !templateFile.isFile()) {
            throw new Error(
                `Unable to instantiate template. The requested template does not exist: ${template.path}`,
            )
        }

        const templateContent = await this.vault.read(templateFile)

        let instantiatedFile: File | undefined

        await this.loader.vault.withLock(async () => {
            const templateUuid = randomUUID()

            instantiatedFile = await this.vault.createEmptyMarkdownFile(
                itemsDir,
                `${templateUuid}.md`,
            )

            await this.vault.modifyContent(instantiatedFile, templateContent)
        })

        // We do not want to test editor-specific functionality
        if (!process.env["IN_TEST_RUNNER"]) {
            const activeLeaf = app.workspace.getLeaf(false)

            if (!activeLeaf || !instantiatedFile) {
                return
            }

            await activeLeaf.openFile((instantiatedFile as ObsidianFile).asFile(), {
                state: {
                    mode: "source",
                },
            })
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Read
    ////////////////////////////////////////////////////////////////////////////
    getTemplates(): Immutable<Exclude<TemplateSchema, undefined>> {
        return this._table.templates ?? { options: [] }
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Update
    ////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////
    /// Delete
    ////////////////////////////////////////////////////////////////////////////
}
export class DocumentSchema {
    constructor(
        private tableName: string,
        private table: TableSchema,
        private loader: SchemaLoader,
        private schema: Schema,
    ) {}

    ////////////////////////////////////////////////////////////////////////////
    /// Util
    ////////////////////////////////////////////////////////////////////////////
    async navigateTo(document: Immutable<Document>): Promise<void> {
        const file = await this.loader.vault.getFile(document.path)
        const activeLeaf = app.workspace.getLeaf(false)

        if (!activeLeaf || !file) {
            return
        }

        await activeLeaf.openFile((file as ObsidianFile).asFile(), {
            state: {
                mode: "source",
            },
        })
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Create
    ////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////
    /// Read
    ////////////////////////////////////////////////////////////////////////////
    /**
     * TODO: Migrate to use vault
     * TODO: Test
     */
    getAll(): Immutable<Document[]> {
        return this.loader.index.getDocuments(this.tableName)
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Update
    ////////////////////////////////////////////////////////////////////////////
    /**
     * Runs an update function over all documents that belong to the current
     * schema.
     *
     * TODO: Migrate to use vault
     * TODO: Test
     */
    async updateFrontmatter(update: (frontmatter: Record<string, unknown>) => void): Promise<void> {
        const pages = this.loader.index.tags.getPages(this.tableName)

        await this.loader.vault.withLock(async () => {
            const promises = []

            for (const page of pages) {
                const file = app.metadataCache.getFirstLinkpathDest(page, page)

                if (!file) {
                    continue
                }

                promises.push(app.fileManager.processFrontMatter(file, update))
            }

            await Promise.all(promises)
        })
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Delete
    ////////////////////////////////////////////////////////////////////////////
}
export class PropertySchema2 {
    constructor(
        private tableName: string,
        private table: TableSchema,
        private loader: SchemaLoader,
        private schema: Schema,
    ) {}

    ////////////////////////////////////////////////////////////////////////////
    /// Create
    ////////////////////////////////////////////////////////////////////////////
    /**
     * Creates a new property of a predefined `kind` on the table
     * with default values.
     */
    async create<TKind extends PropertyKind>(kind: TKind): Promise<void> {
        const property = getProperty(kind)
        const propertyName = this.getNextAvailablePropertyName(kind)

        const uuid = randomUUID()

        await this.loader.vault.withLock(async () => {
            // TODO: FIX
            // @ts-expect-error
            this.table.properties.push({
                name: propertyName,
                kind,
                uuid,
                config: {
                    ...structuredClone(property.config.default),
                },
            })

            this.table.views.forEach((view) => {
                // TODO: FIX
                // @ts-expect-error
                view.config.properties?.push({ uuid: randomUUID(), field: uuid })
            })

            await this.loader.persist()

            if (kind === "title") {
                return
            }

            await this.schema.document.updateFrontmatter((frontmatter) => {
                const tagData = (frontmatter[this.tableName] ??= {})

                if (isObject(tagData) && !(propertyName in tagData)) {
                    tagData[propertyName] = property.field.default
                }
            })
        })
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Read
    ////////////////////////////////////////////////////////////////////////////
    find(propertyUuid: string): Immutable<PropertySchema> | undefined {
        return this.table.properties.find((property) => property.uuid === propertyUuid)
    }

    /**
     * Returns a list of all properties that exist on the table.
     *
     * TODO: Migrate to use vault
     * TODO: Test
     */
    getAll(): Immutable<PropertySchema[]> {
        return this.table.properties
    }
    /**
     * Returns the value of a property on a document.
     *
     * TODO: Migrate to use vault
     * TODO: Test
     */
    getValue<TProperty extends PropertySchema>(
        property: TProperty | Immutable<TProperty>,
        refDocument: Immutable<Document>,
    ): PropertyFieldRepr<TProperty["kind"]> {
        const file = app.metadataCache.getFirstLinkpathDest(refDocument.path, refDocument.path)

        if (!file) {
            throw new Error(`TODO`)
        }

        const { field } = getProperty(property.kind as PropertyKind)

        const document = this.loader.index.documents.get(refDocument.path)

        if (!document) {
            throw new Error(`Unable to find document`)
        }

        const fields = document.data.fields as ForceMutable<Document["data"]["fields"]>
        // TODO: REMOVE CAST TO RECORD<ANY,ANY>
        const fieldValues = (fields[this.tableName] ??= {}) as Record<any, any>

        if (!(property.name in fieldValues)) {
            return structuredClone(field.default) as never

            /*
            app.fileManager.processFrontMatter(file, (frontmatter) => {
                const existing = (frontmatter[this.tableName] ??= {})

                existing[property.name] = structuredClone(field.default) as PropertyFieldRepr<
                    TProperty["kind"]
                >
            })
            */
        }

        // console.log(fieldValues)

        return fieldValues[property.name]
    }

    /**
     * Returns the next available field name for use within a table.
     *
     * Names are generated in the format of `Field Name 1`, `Field Name 2`, etc.
     * If no available name is found after 20 attempts, a random UUID is returned.
     */
    getNextAvailablePropertyName<TKind extends PropertyKind>(kind: TKind): string {
        const metadata = getProperty(kind)

        for (let i = 1; i < 20; i++) {
            const name = `${metadata.name} ${i}`

            if (!this.table.properties.find((it) => it.name === name)) {
                return name
            }
        }

        return randomUUID()
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Update
    ////////////////////////////////////////////////////////////////////////////
    /**
     * Renames a property and updates all associated documents.
     *
     * TODO: Migrate to use vault
     * TODO: Test
     */
    async rename<TProperty extends PropertySchema>(
        propertyToRename: Immutable<TProperty>,
        newName: string,
    ): Promise<void> {
        const existingName = propertyToRename.name

        const existingProperty = this.table.properties.find(
            (property) => property.uuid === propertyToRename.uuid,
        )

        if (!existingProperty) {
            return
        }

        const mutableExistingProperty = existingProperty as ForceMutable<typeof existingProperty>
        mutableExistingProperty.name = newName

        await this.loader.vault.withLock(async () => {
            await this.loader.persist()

            await this.schema.document.updateFrontmatter((frontmatter) => {
                const data = (frontmatter[this.tableName] ??= {})

                if (isObject(data) && existingName in data) {
                    const existingData = data[existingName]
                    data[newName] = existingData
                    delete data[existingName]
                }
            })
        })
    }

    /**
     * Updates the config of a property.
     *
     * TODO: Migrate to use vault
     * TODO: Test
     */
    async updateConfig<const TProperty extends PropertySchema>(
        property: TProperty,
        update: (
            config: TProperty["config"],
            morph: PropertyRepr<"select">["config"]["morphs"],
        ) => void | Promise<void>,
    ) {
        const incomingConfig = structuredClone(property.config)
        const morphs = getProperty(property.kind).config.morphs

        // TODO: FIX
        // @ts-expect-error
        await update(incomingConfig, morphs as PropertyRepr<TProperty["kind"]>["config"]["morphs"])

        property.config = incomingConfig

        await this.loader.persist()
    }

    /**
     * Updates the value of a property in a document.
     *
     * TODO: Migrate to use vault
     * TODO: Test
     */
    async updateValue<TProperty extends PropertySchema>(
        property: TProperty,
        document: Immutable<Document>,
        update: (
            oldConfig: PropertyRepr<TProperty["kind"]>["field"]["default"],
        ) => PropertyRepr<TProperty["kind"]>["field"]["default"],
    ): Promise<void> {
        const file = app.metadataCache.getFirstLinkpathDest(document.path, document.path)

        if (!file) {
            return
        }

        const { field } = getProperty(property.kind)

        // TODO: Add this functionality as a method in DocumentSchema
        await app.fileManager.processFrontMatter(file, (frontmatter) => {
            const existing = (frontmatter[this.tableName] ??= {})

            existing[property.name] = update(existing[property.name] ?? field.default)
        })
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Delete
    ////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////
    /// Create
    ////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////
    /// Read
    ////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////
    /// Update
    ////////////////////////////////////////////////////////////////////////////
    /**
     * Moves a property to a new position within the table.
     *
     * This will effect the order of properties shown in the
     * schema editor as well as the default order of properties
     * in new datatable views.
     *
    async move(fromPosition: number, toPosition: number): Promise<void> {
        if (fromPosition === toPosition || fromPosition < 0 || toPosition < 0) {
            return
        }

        this.table.properties = arrayMoveImmutable(this.table.properties, fromPosition, toPosition)

        await this.schema.persist()
    }
    
    
    

    ////////////////////////////////////////////////////////////////////////////
    /// Delete
    ////////////////////////////////////////////////////////////////////////////
    */
}
export class ViewSchema2 {
    constructor(
        private tableName: string,
        private table: TableSchema,
        private loader: SchemaLoader,
        private schema: Schema,
    ) {}

    ////////////////////////////////////////////////////////////////////////////
    /// Create
    ////////////////////////////////////////////////////////////////////////////
    /**
     * TODO: Migrate to use vault
     * TODO: Test
     * TODO: Document
     */
    async addFilter(viewUuid: string, propertyUuid: string): Promise<void> {
        const view = this.table.views.find((it) => it.uuid === viewUuid)
        const property = this.table.properties.find((it) => it.uuid === propertyUuid)

        if (!view || !property) {
            return
        }

        const definition = getProperty(property.kind)

        await this.loader.vault.withLock(async () => {
            view.scope.filters.push({
                uuid: randomUUID(),
                property: propertyUuid,
                ...structuredClone(definition.filter.default),
            } as never)

            await this.loader.persist()
        })
    }

    /**
     * Ensures that a table has a default view associated with it.
     *
     * This is intended to provide a better user experience, so that
     *
     * TODO: Migrate to use vault
     * TODO: Test
     * TODO: Document
     */
    async createView<TKind extends ViewKind>(kind: TKind): Promise<ViewSchema> {
        const viewDefinition = getView(kind)

        const viewSchema = {
            name: this.getNextAvailableViewName(kind),
            kind,
            uuid: randomUUID() as string,
            layout: {
                ...structuredClone(viewDefinition.layout.default),
            },
            scope: {
                ...structuredClone(viewDefinition.scope.default),
            },
            config: {
                ...structuredClone(viewDefinition.config.default),
            },
        } as ViewSchema

        await this.loader.vault.withLock(async () => {
            this.table.views.push(viewSchema)

            await this.loader.persist()
        })

        return viewSchema
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Read
    ////////////////////////////////////////////////////////////////////////////
    /**
     * Returns a single view of the table, throwing an error if it does not
     * exist.
     *
     * TODO: Migrate to use vault
     * TODO: Test
     * TODO: Convert to use an Option or Result instead of throwing
     */
    get(viewUuid: string): Immutable<ViewSchema> {
        const result = this.table.views.find((view) => view.uuid === viewUuid)

        if (!result) {
            throw new Error(`Unable to find view ${viewUuid} for table ${this.tableName}.`)
        }

        return result
    }

    getAll(): Immutable<ViewSchema[]> {
        return this.table.views
    }

    /**
     * Returns the next available view name for use within a table.
     *
     * Names are generated in the format of `Field Name 1`, `Field Name 2`, etc.
     * If no available name is found after 20 attempts, a random UUID is returned.
     */
    getNextAvailableViewName<TKind extends ViewKind>(kind: TKind): string {
        const metadata = getView(kind)

        for (let i = 1; i < 20; i++) {
            const name = `${metadata.name} ${i}`

            if (!this.table.views.find((it) => it.name === name)) {
                return name
            }
        }

        return randomUUID()
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Update
    ////////////////////////////////////////////////////////////////////////////
    /**
     * TODO: Migrate to use vault
     * TODO: Test
     * TODO: Document
     */
    async rename(viewUuid: string, newName: string): Promise<void> {
        const view = this.table.views.find((it) => it.uuid === viewUuid)

        if (view) {
            await this.loader.vault.withLock(async () => {
                view.name = newName

                await this.loader.persist()
            })
        }
    }

    /**
     * TODO: Migrate to use vault
     * TODO: Test
     * TODO: Document
     */
    async resizeColumns(viewUuid: string, widths: Map<number | string, number | string>) {
        const view = this.table.views.find((it) => it.uuid === viewUuid)

        if (!view) {
            return
        }

        await this.loader.vault.withLock(async () => {
            for (const [columnUuid, columnWidth] of widths.entries()) {
                if (typeof columnUuid === "number") {
                    continue
                }

                const property = view.config.properties.find((it) => it.id === columnUuid)

                if (!property && typeof columnWidth === "number") {
                    view.config.properties.push({ id: columnUuid, width: columnWidth })
                } else if (property && typeof columnWidth === "number") {
                    property.width = columnWidth
                } else if (property && typeof columnWidth === "string") {
                    delete property.width
                }
            }

            await this.loader.persist()
        })
    }

    /**
     * TODO: Migrate to use vault
     * TODO: Test
     * TODO: Document
     */
    async updateFilter(viewUuid: string, filterUuid: string, newValue: unknown): Promise<void> {
        const view = this.table.views.find((it) => it.uuid === viewUuid)
        const filter = view?.scope.filters.find((it) => it.uuid === filterUuid)

        if (!filter) {
            return
        }

        // TODO: FIX
        // @ts-expect-error
        filter.data = newValue

        await this.loader.persist()

        /*
        refDocument: Document,
        update: (
            oldConfig: PropertyRepr<TProperty["kind"]>["field"]["default"],
        ) => PropertyRepr<TProperty["kind"]>["field"]["default"],

        const file = app.metadataCache.getFirstLinkpathDest(refDocument.path, refDocument.path)

        if (!file) {
            return
        }

        const { field } = getProperty(property.kind)

        await app.fileManager.processFrontMatter(file, (frontmatter) => {
            const existing = (frontmatter[this.tableName] ??= {})

            existing[property.name] = update(existing[property.name] ?? field.default)
        })
        */
    }

    async updateFilterKind(viewUuid: string, filterUuid: string, kind: string): Promise<void> {
        const view = this.table.views.find((it) => it.uuid === viewUuid)
        const filter = view?.scope.filters.find((it) => it.uuid === filterUuid)

        if (!filter) {
            return
        }

        const property = this.table.properties.find((it) => it.uuid === filter.property)

        if (!view || !property) {
            return
        }

        filter.kind = kind
        await this.loader.persist()
        /*
        const definition = getProperty(property.kind)

        await this.loader.vault.withLock(async () => {
            view.scope.filters.push({
                uuid: randomUUID(),
                property: propertyUuid,
                ...structuredClone(definition.filter.default),
            } as never)

            await this.loader.persist()
        })
        */
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Delete
    ////////////////////////////////////////////////////////////////////////////
    async deleteFilter(viewUuid: string, filterUuid: string): Promise<void> {
        const view = this.table.views.find((it) => it.uuid === viewUuid)

        if (!view) {
            return
        }

        view.scope.filters = view.scope.filters.filter((filter) => filter.uuid !== filterUuid)

        await this.loader.persist()
    }

    /*
    

    ////////////////////////////////////////////////////////////////////////////
    /// Update
    ////////////////////////////////////////////////////////////////////////////
    async reorderViews(
        viewAUuid: string,
        moveKind: "before" | "after",
        viewBUuid: string,
    ): Promise<void> {
        const views = this.table.views

        const viewA = views.findIndex((view) => view.uuid === viewAUuid)
        const viewB = views.findIndex((view) => view.uuid === viewBUuid)

        if (viewA === -1 || viewB === -1) {
            // TODO: Emit warning here
            console.warn("")
            return
        }

        const newPosition = moveKind === "before" ? viewB : Math.min(views.length - 1, viewB + 1)

        this.table.views = arrayMoveImmutable(views, viewA, newPosition)

        await this.schema.persist()
    }

    
/*
    

    

    ////////////////////////////////////////////////////////////////////////////
    /// Delete
    ////////////////////////////////////////////////////////////////////////////
    

    /*
    
    
    // Read
    //=====
    
    
    // Update
    //=======
    async move(fromPosition: number, toPosition: number): Promise<void> {
        table.views = arrayMoveImmutable(table.views, fromPosition, toPosition)

        await schemaService.persist()
    }
    async toggleProperty(view: string, uuid: string): Promise<void> {
        const tableView = table.views.find((it) => it.name === view)

        if (!tableView) {
            return
        }

        const properties = tableView.scope.properties

        if (properties.some((it) => it === uuid)) {
            tableView.scope.properties = tableView.scope.properties.filter(
                (it) => it !== uuid,
            )
        } else {
            tableView.scope.properties.push(uuid)
        }

        await schemaService.persist()
        // const  = table.views
    }
    // Delete
    //=======
    */
}

/*
import { arrayMove } from "@dnd-kit/sortable"
import { randomUUID } from "crypto"
import pluralize from "pluralize"

import { Immutable } from "../../../types/immutable"
import { debounce } from "../../../util/debounce"
import { isObject } from "../../../util/primitive"
import { Page } from "../page/page.model"
import { FieldDefinitionRepr, FieldValueRepr, getFieldDefinition } from "./field/field"
import { FieldKind } from "./field/field.definition"
import { DtField, TableSchema, ViewSchema } from "./schema-definition"
import { SchemaService } from "./schema.service"
import { ViewKind } from "./view/view-definition"

export type Schema = ReturnType<typeof createEditor>

export const createEditor = (tableName: string, table: TableSchema, schemaService: SchemaService) => {
    const editor = {
        tableName,
        schemaService,
        get revision() {
            return schemaService.index().revision
        },
        field: {
            // Update
            //=======
            /**
             * Updates the field name of a table.
             *
             * ! This updates all tagged pages with the new field name.
             *
            async changeName(field: DtField, newName: string) {
                table.fields = table.fields.map((it) =>
                    it.name === field.name ? { ...it, name: newName } : it,
                )

                const originalName = field.name
                field.name = newName

                await schema.page.updateTableFrontmatter(tableName, (frontmatter) => {
                    const data = (frontmatter[tableName] ??= {})

                    if (isObject(data) && originalName in data) {
                        data[newName] = data[originalName]
                        delete data[originalName]
                    }
                })

                await schemaService.persist()
            },
            // Delete
            //=======
            async delete(field: DtField) {
                table.fields = table.fields.filter((it) => it.name !== field.name)

                await schemaService.persist()

                await schema.page.updateTableFrontmatter(tableName, (frontmatter) => {
                    const tagData = (frontmatter[tableName] ??= {})

                    if (isObject(tagData) && field.name in tagData) {
                        delete tagData[field.name]
                    }
                })
            },
        },
        view: {
            filter: {
                            async add(view: ViewSchema | Immutable<ViewSchema>, propertyUuid: string) {
                const property = view.config.properties.find((it) => it.uuid === propertyUuid)
                    ?.field
                const field = table.fields.find((it) => it.uuid === property)

                if (!field) {
                    return
                }

                const filter = getFieldDefinition(field.kind).filter.default

                view.config.filters?.push({
                    property: propertyUuid,
                    filter,
                })

                await schemaService.persist()
            },
                get: () => {},
                update: () => {},
                remove: () => {},
            },

            // Read
            //=====



            // Update
            //=======

            async moveColumn(
                view: ViewSchema | Immutable<ViewSchema>,
                fromUuid: string,
                toUuid: string,
            ): Promise<void> {
                const properties = view.config.properties

                const fromIndex = properties.findIndex((i) => i.field === fromUuid)
                const toIndex = properties.findIndex((i) => i.field === toUuid)

                if (fromIndex === -1 || toIndex === -1) {
                    return
                }

                view.config.properties = arrayMove(properties, fromIndex, toIndex)

                await schemaService.persist()
            },
            
            // Delete
            //=======
        },
        table: {
            // Create
            //=======
            // Read
            //=====
            getNames(): { name: string; prettyName: string }[] {
                const tables = schemaService.getSchema().tables
                const tableNames = Object.keys(tables)

                const getPrettyName = (name: string) => {
                    const sliced = name.slice(1)
                    const words = sliced.replace(/[-_]/g, " ").split(/(?=[A-Z])/)

                    return words
                        .map((it, index) => {
                            let word = it[0]?.toUpperCase() + it.slice(1)

                            if (index === words.length - 1) {
                                word = pluralize(word)
                            }

                            return word.trim()
                        })
                        .join(" ")
                        .trim()
                }

                return tableNames.map((it) => ({
                    name: it,
                    prettyName: getPrettyName(it),
                }))
            },

            // Update
            //=======
            // Delete
            //=======
        },
    }

    return editor
}

/*
    /**
     * Updates a datatable field to be a new type.
     *
     * The new type is populated with the default config, and all
     * pages are updated with the default value.
     *
     * ! This updates all tagged pages with the new field value.
     *
    async changeFieldType(field: DatatableField, newType: FieldKind) {
        if (field.kind === newType) {
            return
        }

        const table = this.schemaService.getTable(this.tag)

        const oldField = structuredClone(field)
        const fieldDefinition = getFieldDefinition(newType)

        table.fields = table.fields.map((it) =>
            it.kind === field.kind
                ? ({
                      ...it,
                      kind: newType,
                      config: structuredClone(fieldDefinition.config.default),
                  } as DatatableField)
                : it,
        )

        await this.updatePageFrontmatterForTag(this.tag, (frontmatter) => {
            const data = (frontmatter[this.tag] ??= {})

            if (isObject(data) && oldField.name in data) {
                data[oldField.name] = structuredClone(fieldDefinition.value.default)
            }
        })

        await this.schemaService.persist()
    }
}
*/
