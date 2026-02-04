import type { Immutable } from "@sribich/ts-utils"
import { Divider, Flex, MultiProvider } from "@sribich/fude"

import { FilterBar } from "../../ui/features/datatable/filter/FilterBar"
import { useDatatableStateContext } from "../../ui/features/datatable/hooks/useDatatableState"
import { SortBar } from "../../ui/features/datatable/sort/SortBar"
import { ViewProvider } from "../../ui/hooks/useView"
import { ViewScopeProvider, useViewScope } from "../../ui/hooks/useViewScope"
import { viewComponents } from "./components"
import type { ViewSchema } from "./view-schema"

////////////////////////////////////////////////////////////////////////////////
/// View
////////////////////////////////////////////////////////////////////////////////
interface ViewProps {
    view: Immutable<ViewSchema>
}

export const View = ({ view }: ViewProps) => {
    const viewScope = useViewScope(view.uuid)

    const Component = viewComponents[view.kind].component

    return (
        <MultiProvider
            values={[
                [ViewProvider, view.uuid],
                [ViewScopeProvider, viewScope],
            ]}
        >
            <DatatableCollationBar />
            <Component />
        </MultiProvider>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// DatatableCollationBar
////////////////////////////////////////////////////////////////////////////////
const DatatableCollationBar = () => {
    const { filterBarVisible, sortBarVisible } = useDatatableStateContext()

    if (!filterBarVisible && !sortBarVisible) {
        return null
    }

    return (
        <Flex>
            {sortBarVisible && <SortBar />}
            {sortBarVisible && filterBarVisible && <Divider orientation="vertical" className="mx-2" />}
            {filterBarVisible && <FilterBar />}
        </Flex>
    )
}
