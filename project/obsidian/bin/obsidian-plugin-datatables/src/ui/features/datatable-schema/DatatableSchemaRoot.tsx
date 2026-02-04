import { Tab, Tabs } from "@sribich/fude"

import type { DatatableSchemaContext } from "../../../processor/processors/datatable-schema"
import { useMountContext } from "../../hooks/useMountContext"
import { SchemaProvider } from "../../hooks/useSchema"
import { PropertyList } from "./PropertyList"

////////////////////////////////////////////////////////////////////////////////
/// DatatableSchemaRoot
////////////////////////////////////////////////////////////////////////////////
export const DatatableSchemaRoot = () => {
    const { proxy } = useMountContext<DatatableSchemaContext>()

    const tags = proxy.document.data.tags.map((tag) => ({ tag }))

    proxy.indexRevision
    proxy.schemaRevision

    return (
        <Tabs items={tags} variant="pill" radius="sm">
            {({ tag }) => (
                <Tab id={tag} title={tag}>
                    <DatatableSchema tableName={tag} />
                </Tab>
            )}
        </Tabs>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// DatatableSchema
////////////////////////////////////////////////////////////////////////////////
interface DatatableSchemaProps {
    tableName: string
}

const DatatableSchema = (props: DatatableSchemaProps) => {
    const { loader } = useMountContext<DatatableSchemaContext>()

    const schema = loader.getSchema(props.tableName)

    return (
        <SchemaProvider value={schema}>
            <PropertyList />
        </SchemaProvider>
    )
}
