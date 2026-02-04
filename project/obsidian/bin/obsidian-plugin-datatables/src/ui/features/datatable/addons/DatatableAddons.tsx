import { Button, Flex } from "@sribich/fude"

import { useDatatableStateContext } from "../hooks/useDatatableState"
import { SearchAddon } from "./SearchAddon"
import { SettingsAddon } from "./SettingsAddon"
import { TemplateAddon } from "./TemplateAddon"

////////////////////////////////////////////////////////////////////////////////
/// DatatableAddons
////////////////////////////////////////////////////////////////////////////////
export const DatatableAddons = () => {
    const { toggleFilterBar, toggleSortBar } = useDatatableStateContext()

    // TODO: We should probably convert the filter/sort buttons to ToggleButtons
    //       so that they can be properly represented as open/closed.
    return (
        <Flex alignItems="center" gap="2">
            <Button size="sm" variant="light" onClick={toggleFilterBar}>
                Filter
            </Button>

            <Button size="sm" variant="light" onClick={toggleSortBar}>
                Sort
            </Button>

            <SearchAddon />

            <SettingsAddon />
            <TemplateAddon />
        </Flex>
    )
}
