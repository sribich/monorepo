import type { DatatableContext } from "../../../processor/processors/datatable"
import { useMountContext } from "../../hooks/useMountContext"
import { SchemaProvider } from "../../hooks/useSchema"
import { Datatable } from "../datatable/Datatable"
import { DatatableSourceConfigurator } from "./DatatableSourceConfigurator"

////////////////////////////////////////////////////////////////////////////////
/// DatatableRoot
////////////////////////////////////////////////////////////////////////////////
export const DatatableRoot = () => {
    const { proxy } = useMountContext<DatatableContext>()

    proxy.indexRevision
    proxy.schemaRevision

    if (!proxy.codeBlock.source) {
        return <DatatableSourceConfigurator />
    }

    return <DatatableRootView source={proxy.codeBlock.source} />
}

////////////////////////////////////////////////////////////////////////////////
/// DatatableRootView
////////////////////////////////////////////////////////////////////////////////
interface DatatableRootViewProps {
    source: string
}

const DatatableRootView = (props: DatatableRootViewProps) => {
    const { loader } = useMountContext<DatatableContext>()

    const schema = loader.getSchema(props.source)

    return (
        <SchemaProvider value={schema}>
            <Datatable />
        </SchemaProvider>
    )
}
