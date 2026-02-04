import type { Immutable } from "@sribich/ts-utils"

import type { Index } from "../index/index"
import type { SettingsContainer } from "../settings/settings-container"
import { objectToPrettyJson } from "../util/json"
import type { File, Vault } from "../vault/vault"
import { Schema } from "./schema"
import { type DtSchema, defaultSchema, schema } from "./schema-definition"

export class SchemaLoader {
    protected schema!: DtSchema
    protected schemaFile!: File

    private persistDebouncer: NodeJS.Timeout | undefined
    private persistPromise: Promise<void> | undefined
    private persistRejector: ((error: Error) => void) | undefined
    private persistResolver: (() => void) | undefined
    private revision = 0

    private constructor(
        public readonly index: Index,
        public readonly vault: Vault,
        public readonly settings: SettingsContainer,
    ) {}

    /**
     * TODO: Docs
     */
    static async create(
        index: Index,
        vault: Vault,
        settings: SettingsContainer,
    ): Promise<SchemaLoader> {
        const loader = new SchemaLoader(index, vault, settings)

        const schemaPath = `Schemas/${settings.schema.datatablesFile}`
        const schemaFile = await loader.getSchemaFile(schemaPath)

        const schemaData = await vault.read(schemaFile)

        loader.schemaFile = schemaFile
        loader.schema = await loader.parseSchema(schemaData)

        return loader
    }

    /**
     * @internal
     */
    internalGetSchema(): Immutable<DtSchema> {
        return this.schema
    }

    getSchema(tableName: string) {
        if (!tableName.startsWith("#")) {
            throw new Error(`Expected table to be a tag starting with '#'. Got '${tableName}'.`)
        }

        tableName = tableName.toLocaleLowerCase()
        const table = this.schema.tables[tableName]

        if (!table) {
            throw new Error(`Table does not exist: ${tableName}`)
        }

        // const table = (this.schema.tables[tableName] ??= {
        //     properties: [],
        //     views: [],
        // })

        return new Schema(tableName, table, this, this.vault, this.settings)
    }

    async persist() {
        if (this.persistDebouncer) {
            clearTimeout(this.persistDebouncer)
        }

        if (!this.persistPromise) {
            this.persistPromise = new Promise((resolve, reject) => {
                this.persistResolver = resolve
                this.persistRejector = reject
            })
        }

        this.persistDebouncer = setTimeout(async () => {
            const serializedSchema = objectToPrettyJson(this.schema)

            try {
                await this.parseSchema(serializedSchema)
            } catch (e) {
                const error = new Error(
                    `Unable to persist schema. Serialization produced a result which cannot be parsed: ${e}`,
                )

                // We do not want to exit when testing. Instead, reject the persist
                // call so we can verify test cases.
                if (process.env["IN_TEST_RUNNER"] === "true") {
                    return this.persistRejector?.(error)
                }

                process.exit(1)

                // @ts-expect-error See startExitLoop documentation for reasoning
                this.startExitLoop(error)
            }

            await this.vault.modifyContent(this.schemaFile, serializedSchema)

            this.markDirty()

            this.persistResolver?.()

            this.persistDebouncer = undefined
            this.persistPromise = undefined
            this.persistRejector = undefined
            this.persistResolver = undefined
        }, 100)

        return this.persistPromise
    }

    private markDirty() {
        this.revision += 1

        this.vault.emit("datatables:schema:changed", {
            revision: this.revision,
        })
    }

    /**
     * Returns the path to the persisted schema file, creating an empty schema
     * in its place if it does not exist.
     */
    private async getSchemaFile(path: string): Promise<File> {
        const datatablesFile = await this.vault.getFile(path)

        if (datatablesFile?.isFile()) {
            return datatablesFile
        }

        if (datatablesFile?.isDirectory()) {
            throw new Error(
                `The provided schema path is a folder. The path must either point to an existing schema file, or not exist. If the path does not exist, a new schema file will be created.`,
            )
        }

        return await this.vault.create(path, JSON.stringify(defaultSchema, undefined, 2))
    }

    /**
     * Parses the content of the schema file.
     */
    private async parseSchema(schemaData: string): Promise<DtSchema> {
        let jsonData

        try {
            jsonData = JSON.parse(schemaData)
        } catch (e) {
            throw new Error(`Failed to parse schema content: JSON parsing failed: ${e}`)
        }

        const { data, problems } = schema(jsonData)

        if (problems && problems.length > 0) {
            throw new Error(
                `Failed to parse schema content: Validation failed: ${problems.summary}`,
            )
        }

        if (!data) {
            throw new Error(`Failed to parse schema content: Data is null`)
        }

        return data
    }

    /**
     * Infinitely loop, preventing the application from continuing execution.
     *
     * There are cases where electron will fail to close when calling
     * process.exit. This is a final stop-gap to prevent us from potentially
     * making a change which will cause the schema to become corrupted.
     *
     * TODO: Force electron to close ala our privacy plugin.
     */
    private startExitLoop(cause: Error) {
        while (true) {
            console.log(cause)
        }
    }
}
