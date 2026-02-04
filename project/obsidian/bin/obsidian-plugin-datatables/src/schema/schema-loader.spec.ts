import { describe, vi } from "vitest"

import { it } from "../../test/context"
import { SchemaLoader } from "./schema-loader"

describe("Schema Loader", () => {
    describe("Parsing", () => {
        it("Loads a valid schema", async ({ expect, loadFixture }) => {
            const fixture = await loadFixture("schema-loader/common/schema_valid")

            const loader = await SchemaLoader.create(fixture.index, fixture.vault, fixture.settings)

            expect(loader).toBeTruthy()
        })

        it("Creates a schema file when one does not exist", async ({ expect, loadFixture }) => {
            const fixture = await loadFixture("schema-loader/parsing/schema_file_nonexistent")

            const schemaPath = `${fixture.settings.schema.folder}/${fixture.settings.schema.datatablesFile}`
            const schemaFile = () => fixture.vault.getFile(schemaPath)

            expect(await schemaFile()).toBeNull()

            await SchemaLoader.create(fixture.index, fixture.vault, fixture.settings)

            expect(await schemaFile()).not.toBeNull()
        })

        it("Errors when creating a schema if the schema directory does not exist", async ({
            expect,
            loadFixture,
        }) => {
            const fixture = await loadFixture("schema-loader/parsing/schema_dir_nonexistent")

            expect(async () => {
                await SchemaLoader.create(fixture.index, fixture.vault, fixture.settings)
            }).rejects.toThrowError(
                "The schema folder must exist before the schema file can be created",
            )
        })

        it("Errors if the schema file is a directory", async ({ expect, loadFixture }) => {
            const fixture = await loadFixture("schema-loader/parsing/schema_file_is_directory")

            expect(async () => {
                await SchemaLoader.create(fixture.index, fixture.vault, fixture.settings)
            }).rejects.toThrowError(
                "The provided schema path is a folder. The path must either point to an existing schema file, or not exist. If the path does not exist, a new schema file will be created.",
            )
        })

        it("Errors if the schema contains invalid JSON", async ({ expect, loadFixture }) => {
            const fixture = await loadFixture("schema-loader/parsing/schema_invalid_json")

            expect(async () => {
                await SchemaLoader.create(fixture.index, fixture.vault, fixture.settings)
            }).rejects.toThrowError("Failed to parse schema content: JSON parsing failed:")
        })

        it("Errors if the schema failed schema validation", async ({ expect, loadFixture }) => {
            const fixture = await loadFixture("schema-loader/parsing/schema_invalid_data")

            expect(async () => {
                await SchemaLoader.create(fixture.index, fixture.vault, fixture.settings)
            }).rejects.toThrowError("Failed to parse schema content: Validation failed:")
        })
    })

    describe("Persisting", () => {
        it("Persists a schema change", async ({ expect, loadFixture }) => {
            const fixture = await loadFixture("schema-loader/common/schema_valid")

            const loaderA = await SchemaLoader.create(
                fixture.index,
                fixture.vault,
                fixture.settings,
            )

            expect(loaderA.internalGetSchema().version).toEqual(1)
            // @ts-expect-error Accessing a private property
            loaderA.schema.version = 2
            await loaderA.persist()
            expect(loaderA.internalGetSchema().version).toEqual(2)

            // Load a new schema to ensure that the change was persisted to disk
            const loaderB = await SchemaLoader.create(
                fixture.index,
                fixture.vault,
                fixture.settings,
            )
            expect(loaderB.internalGetSchema().version).toEqual(2)
        })

        it("Errors when the schema is un-parsable after serialization", async ({
            expect,
            loadFixture,
        }) => {
            const fixture = await loadFixture("schema-loader/common/schema_valid")

            //
            // TEST CASE A
            //
            const loaderA = await SchemaLoader.create(
                fixture.index,
                fixture.vault,
                fixture.settings,
            )

            expect(loaderA.internalGetSchema().version).toEqual(1)

            await expect(async () => {
                // @ts-expect-error Accessing a private property
                loaderA.schema.version = "invalid"

                await loaderA.persist()
            }).rejects.toThrowError(
                "Unable to persist schema. Serialization produced a result which cannot be parsed",
            )

            //
            // TEST CASE B
            //

            // Load a new schema to ensure that the change was persisted to disk
            const loaderB = await SchemaLoader.create(
                fixture.index,
                fixture.vault,
                fixture.settings,
            )
            expect(loaderB.internalGetSchema().version).toEqual(1)
        })

        it("Debounces persist calls", async ({ expect, loadFixture }) => {
            const fixture = await loadFixture("schema-loader/common/schema_valid")

            const loader = await SchemaLoader.create(fixture.index, fixture.vault, fixture.settings)

            const promiseSpy = vi.fn()
            // @ts-expect-error Private method
            const dirtySpy = vi.spyOn(loader, "markDirty")

            loader.persist().then(promiseSpy)

            const startTime = new Date().getTime()

            for (const index of [...new Array(10)]) {
                loader.persist()

                await new Promise((resolve) => {
                    setTimeout(resolve, 25)
                })
            }

            const endTime = new Date().getTime()

            expect(endTime - startTime).toBeGreaterThan(220)
            expect(promiseSpy).not.toHaveBeenCalled()

            await new Promise((resolve) => {
                setTimeout(resolve, 110)
            })

            expect(promiseSpy).toHaveBeenCalledOnce()
            expect(dirtySpy).toHaveBeenCalledOnce()

            // Run again to make sure that debouncing resets state
            loader.persist().then(promiseSpy)

            await new Promise((resolve) => {
                setTimeout(resolve, 110)
            })

            expect(promiseSpy).toHaveBeenCalledTimes(2)
        })

        it("Updates the revision after persisting", async ({ expect, loadFixture }) => {
            const fixture = await loadFixture("schema-loader/common/schema_valid")

            const loader = await SchemaLoader.create(fixture.index, fixture.vault, fixture.settings)

            // @ts-expect-error Revision is private
            expect(loader.revision).toEqual(0)

            await loader.persist()

            // @ts-expect-error Revision is private
            expect(loader.revision).toEqual(1)
        })

        it("Emits an event when the schema is changed", async ({ expect, loadFixture }) => {
            const fixture = await loadFixture("schema-loader/common/schema_valid")

            const loader = await SchemaLoader.create(fixture.index, fixture.vault, fixture.settings)

            const spy = vi.spyOn(fixture.vault, "emitWithoutLock")

            await loader.persist()

            expect(spy).toHaveBeenCalledOnce()
            expect(spy).toHaveBeenCalledWith("datatables:schema:changed", { revision: 1 })
        })

        it("Waits to emit until after persisting when locked", async ({ expect, loadFixture }) => {
            const fixture = await loadFixture("schema-loader/common/schema_valid")

            const loader = await SchemaLoader.create(fixture.index, fixture.vault, fixture.settings)

            const spy = vi.spyOn(fixture.vault, "emitWithoutLock")

            fixture.vault.lock()
            await loader.persist()

            expect(spy).not.toHaveBeenCalled()
            fixture.vault.unlock()
            expect(spy).toHaveBeenCalledOnce()
        })
    })
})
