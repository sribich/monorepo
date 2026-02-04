import { stat } from "fs/promises"
import { describe } from "vitest"

import { it } from "../../test/context"
import { SchemaLoader } from "./schema-loader"

describe("Schema", () => {
    describe("Templates", () => {
        ////////////////////////////////////////////////////////////////////////
        /// createTemplate
        ////////////////////////////////////////////////////////////////////////
        it("Creates a new template", async ({ expect, loadFixture }) => {
            const fixture = await loadFixture("schema/templates/empty_templates")

            const loader = await SchemaLoader.create(fixture.index, fixture.vault, fixture.settings)
            const schema = loader.getSchema("#test")

            const initialTemplates = schema.getTemplates()
            await schema.createTemplate()

            expect(initialTemplates?.options.length).toStrictEqual(0)

            const newTemplates = schema.getTemplates()
            expect(newTemplates?.options.length).toStrictEqual(1)

            // @ts-expect-error We know this exists from test setup
            const templatePath = newTemplates.options[0].path
            const templateFile = await fixture.vault.getFile(templatePath)

            expect(templateFile).not.toBeNull()
        })

        it("Errors when the template directory does not exist", async ({ expect, loadFixture }) => {
            const fixture = await loadFixture("schema/templates/no_template_dir")

            const loader = await SchemaLoader.create(fixture.index, fixture.vault, fixture.settings)
            const schema = loader.getSchema("#test")

            await expect(async () => await schema.createTemplate()).rejects.toThrowError(
                `Unable to create template. The template directory (Templates) does not exist`,
            )
        })

        ////////////////////////////////////////////////////////////////////////
        /// getTemplates
        ////////////////////////////////////////////////////////////////////////
        it("Fetches a list of existing templates when no options exist", async ({
            expect,
            loadFixture,
        }) => {
            const fixture = await loadFixture("schema/templates/empty_templates")

            const loader = await SchemaLoader.create(fixture.index, fixture.vault, fixture.settings)
            const schema = loader.getSchema("#test")

            const templates = schema.getTemplates()

            expect(templates).not.toBeUndefined()
            expect(templates?.options.length).toStrictEqual(0)
        })

        it("Fetches a list of existing templates the template key does not exist at all", async ({
            expect,
            loadFixture,
        }) => {
            const fixture = await loadFixture("schema/templates/no_template_key")

            const loader = await SchemaLoader.create(fixture.index, fixture.vault, fixture.settings)
            const schema = loader.getSchema("#test")

            const templates = schema.getTemplates()

            expect(templates).not.toBeUndefined()
            expect(templates?.options.length).toStrictEqual(0)
        })
    })

    it("works", () => {})
})

describe("Property Schema", () => {})

/*
describe("Schema", () => {
    describe("rename", () => {
        it("Renames a property and persists the changes", async ({ expect, loadFixture }) => {
            const fixture = await loadFixture("schema/properties/property_to_rename")

            const loader = await SchemaLoader.create(fixture.index, fixture.vault, fixture.settings)
            const schema = loader.getSchema("#test")
            const propertyToRename = schema.table.properties[0]

            await schema.rename(propertyToRename, "newName")

            const renamedProperty = schema.table.properties.find(property => property.uuid === propertyToRename.uuid)

            expect(renamedProperty).not.toBeNull()
            expect(renamedProperty?.name).toStrictEqual("newName")
        })

        it("Does nothing when the property to rename does not exist", async ({ expect, loadFixture }) => {
            const fixture = await loadFixture("schema/properties/no_such_property")

            const loader = await SchemaLoader.create(fixture.index, fixture.vault, fixture.settings)
            const schema = loader.getSchema("#test")
            const nonExistentProperty = { uuid: "non-existent-uuid", name: "nonExistent" }

            await schema.rename(nonExistentProperty, "newName")

            const renamedProperty = schema.table.properties.find(property => property.uuid === nonExistentProperty.uuid)

            expect(renamedProperty).toBeUndefined()
        })
    })
})
*/
