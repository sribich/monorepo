import type { Immutable } from "@sribich/ts-utils"
import { createGenericContext } from "@sribich/fude"

import type { Document } from "../../index/document"
import { getProperty } from "../../schema/property/property"
import type { PropertySchema } from "../../schema/property/property-schema"
import type { ViewSchema } from "../../schema/view/view-schema"
import type { ViewFilter } from "../../schema/view/view.scope"
import { useSchema } from "./useSchema"

export interface ViewScope {
    /**
     * The UUID of the currently active view.
     */
    view: string
    /**
     * The schema of the currently active view.
     */
    schema: Immutable<ViewSchema>
    /**
     * The list of every property that exists on the table that
     * the view represents.
     */
    properties: Immutable<PropertySchema[]>
    /**
     * The list of properties that are used for the current view.
     */
    viewProperties: Immutable<PropertySchema[]>
    /**
     * The filters for the current view.
     */
    filters: Immutable<ViewFilter[]>
    /**
     * The subset of documents that match the filters for the current view.
     */
    filteredDocuments: Immutable<Document[]>
    /**
     * The list of all documents in the vault.
     */
    documents: Immutable<Document[]>
}

export const [useViewScopeContext, ViewScopeProvider] = createGenericContext<ViewScope>()

/**
 * TODO: This is memoable using schema & index ids
 *
 * TODO(perf): We can heavily optimize the filtering here.
 * TODO(any): Figure out how to remove these anys.
 */
export const useViewScope = (view: string) => {
    if (useViewScopeContext.isProvided()) {
        throw new Error(
            `Called useViewScope within an existing scope. Did you mean to call useViewScopeContext?`,
        )
    }

    const schema = useSchema()

    const viewSchema = schema.view.get(view)
    const properties = schema.property.getAll()

    const viewProperties = viewSchema.scope.properties
        .map((uuid) => properties.find((property) => property.uuid === uuid))
        .filter(Boolean)

    const documents = schema.document.getAll()

    const filters = viewSchema.scope.filters
    const filteredDocuments = documents.filter((document) => {
        // TODO: Do not hardcode templates. Get the path from settings.
        if (document.path.startsWith("Templates")) {
            return false
        }

        return filters.every((filter) => {
            const property = properties.find((it) => it.uuid === filter.property)

            if (property) {
                const definition = getProperty(property.kind)
                const filterItem = definition.filter.filters[filter.kind as never] as any

                return filterItem?.fn(
                    property,
                    filter,
                    (document.data.fields[schema.tableName] as any)?.[property.name],
                )
            }
        })
    })

    return {
        view,
        schema: viewSchema,
        properties,
        viewProperties,
        filters,
        filteredDocuments,
        documents,
    } satisfies ViewScope
}

/*
export const useView = (view: string, editor: Schema): ViewData => {
    const [data, setData] = useState<ViewData | null>(null)

    const schema = schema.view.get(view)
    const fields = schema.field.getAll()

    useEffect(() => {
        const schema = schema.view.get(view)

        const schemaProperties = schema.config.properties ?? []
        const schemaFields = schema.field.getAll()

        if (!schemaFields.some((it) => it.kind === FieldKind.Title)) {
            schema.field.add(FieldKind.Title)
        }

        schema.config.properties = schemaProperties.filter(
            (it) => !!schemaFields.find((field) => field.uuid === it.field),
        )

        if (schemaProperties.length !== schema.config.properties.length) {
            schema.schemaService.persist()
        }
    }, [editor, view])

    const pages = () => schema.page.getAll()

    const properties = schema.config.properties ?? []

    const mappedProperties = properties
        .map((it) => ({
            property: it,
            field: fields.find((field) => field.uuid === it.field),
        }))
        .filter((it) => !!it.field)

    return {
        name: view,
        schema,
        fields,
        pages,
        properties,
        mappedProperties,
    }
}

export type ViewData = {
    name: string
    schema: ViewSchema

    fields: Immutable<DtField[]>
    pages: () => Immutable<Page[]>
    properties: Immutable<
        {
            field: string
            width?: number
        }[]
    >
    mappedProperties: Immutable<
        {
            property: {
                field: string
                width?: number
            }
            field: DtField
        }[]
    >
}
*/
