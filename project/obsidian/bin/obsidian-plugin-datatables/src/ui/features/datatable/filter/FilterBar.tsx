import { Box, Button, Dialog, DialogTrigger, Menu, MenuItem, Popover } from "@sribich/fude"
import { Plus } from "lucide-react"
import { createElement } from "react"

import { propertyComponents } from "../../../../schema/property/property-components"
import { useSchema } from "../../../hooks/useSchema"
import { useViewScopeContext } from "../../../hooks/useViewScope"
import { Filter } from "./Filter"

////////////////////////////////////////////////////////////////////////////////
/// FilterBar
////////////////////////////////////////////////////////////////////////////////
export const FilterBar = () => {
    const schema = useSchema()
    const viewScope = useViewScopeContext()

    const filters = viewScope.filters.map((filter) => {
        const property = schema.property.find(filter.property)

        if (!property) {
            // TODO: What should we do here? Filter on nonexitent property.
            return null
        }

        return <Filter key={filter.uuid} filter={filter} property={property} />
    })

    return (
        <Box className="mb-2">
            {filters}
            <AddFilterButton />
        </Box>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// AddFilterButton
////////////////////////////////////////////////////////////////////////////////
const AddFilterButton = () => {
    const schema = useSchema()
    const viewScope = useViewScopeContext()

    const onAction = (uuid: string | number) => {
        if (typeof uuid !== "string") {
            throw new Error(`TODO`)
        }

        schema.view.addFilter(viewScope.view, uuid)
    }

    return (
        <DialogTrigger>
            <Button variant="light" radius="none" size="sm">
                <Plus size="20" />
                Add filter
            </Button>
            <Popover placement="bottom start">
                <Dialog>
                    <Box shadow="md" padding="2" rounded="md">
                        <div className="max-h-[50vh] w-72">
                            Add filter
                            <Menu items={viewScope.properties} onAction={onAction}>
                                {(item) => (
                                    <MenuItem id={item.uuid}>
                                        {createElement(propertyComponents[item.kind].icon, {})}
                                        {item.name}
                                    </MenuItem>
                                )}
                            </Menu>
                        </div>
                    </Box>
                </Dialog>
            </Popover>
        </DialogTrigger>
    )
}
