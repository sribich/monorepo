import { Tab, Tabs } from "@sribich/fude"
import { createElement } from "react"

import { View } from "../../../schema/view/ViewC"
import { viewComponents } from "../../../schema/view/components"
import { useSchema } from "../../hooks/useSchema"
import { useView } from "../../hooks/useView"
import { DatatableAddons } from "./addons/DatatableAddons"
import { DatatableStateProvider, useDatatableState } from "./hooks/useDatatableState"

////////////////////////////////////////////////////////////////////////////////
/// Datatable
////////////////////////////////////////////////////////////////////////////////
export const Datatable = () => {
    const { view, setView } = useView()

    const schema = useSchema()
    const views = schema.view.getAll()

    const datatableState = useDatatableState()

    return (
        <DatatableStateProvider value={datatableState}>
            <Tabs
                items={views}
                variant="underline"
                defaultSelectedKey={view}
                onSelectionChange={setView}
                addons={view && <DatatableAddons />}
            >
                {(item) => (
                    <Tab
                        id={item.uuid}
                        title={item.name}
                        icon={createElement(viewComponents[item.kind].icon, { size: 16, className: "mr-1" })}
                    >
                        <View view={item} />
                    </Tab>
                )}
            </Tabs>
        </DatatableStateProvider>
    )
}
