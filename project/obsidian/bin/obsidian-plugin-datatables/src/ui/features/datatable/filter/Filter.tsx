import type { Immutable } from "@sribich/ts-utils"
import {
    Box,
    Button,
    Chip,
    DelegateButton,
    Dialog,
    DialogTrigger,
    Flex,
    Menu,
    MenuItem,
    Popover,
    Select,
    SelectItem,
    TypographyText,
} from "@sribich/fude"
import { ChevronDown, MoreHorizontal } from "lucide-react"
import { createElement } from "react"

import { getProperty } from "../../../../schema/property/property"
import { propertyComponents } from "../../../../schema/property/property-components"
import type { PropertySchema } from "../../../../schema/property/property-schema"
import { useSchema } from "../../../hooks/useSchema"
import { useViewScopeContext } from "../../../hooks/useViewScope"

export interface FilterProps {
    filter: any
    property: Immutable<PropertySchema>
}

export const Filter = (props: FilterProps) => {
    const schema = useSchema()
    const viewScope = useViewScopeContext()

    const component = propertyComponents[props.property.kind]

    const property = getProperty(props.property.kind)
    const filterKeys = Object.keys(property.filter.filters).map((it) => ({ key: it }))

    const onSelectionChange = (kind: string | number) => {
        console.log(kind)
        if (kind !== props.filter.kind) {
            console.log("here?")
            schema.view.updateFilterKind(viewScope.schema.uuid, props.filter.uuid, kind)
            console.log("woah")
        }
    }

    const onMoreAction = (action: string | number) => {
        switch (action) {
            case "delete":
                schema.view.deleteFilter(viewScope.schema.uuid, props.filter.uuid)
                return
            default:
                throw new Error(`Unknown action ${action}`)
        }
    }

    return (
        <DialogTrigger>
            <DelegateButton>
                <Chip size="md" className="flex items-center">
                    {createElement(component.icon, { size: 16, className: "mr-1" })}
                    <span>
                        {props.property.name}: {<component.filterContent {...props} />}
                    </span>
                    <ChevronDown size="16" />
                </Chip>
            </DelegateButton>
            <Popover>
                <Dialog>
                    <Box padding="2" rounded="md">
                        <Flex className="mb-2">
                            <Select
                                className="flex-1"
                                label={
                                    <TypographyText size="xs" color="secondary">
                                        {component.name}
                                    </TypographyText>
                                }
                                labelPlacement="side"
                                size="xs"
                                variant="light"
                                defaultSelectedKey={props.filter.kind}
                                items={filterKeys}
                                onSelectionChange={onSelectionChange}
                            >
                                {(item) => (
                                    <SelectItem id={item.key}>
                                        <Box color="secondary">{item.key}</Box>
                                    </SelectItem>
                                )}
                            </Select>
                            <DialogTrigger>
                                <Button size="xs" variant="light" className="flex-0">
                                    <MoreHorizontal size="16" />
                                </Button>
                                <Popover>
                                    <Dialog>
                                        <Menu onAction={onMoreAction}>
                                            <MenuItem id="delete">Delete</MenuItem>
                                            <MenuItem id="convert">
                                                Convert to compound filter
                                            </MenuItem>
                                        </Menu>
                                    </Dialog>
                                </Popover>
                            </DialogTrigger>
                        </Flex>
                        {<component.filter {...props} />}
                    </Box>
                </Dialog>
            </Popover>
        </DialogTrigger>
    )
}
