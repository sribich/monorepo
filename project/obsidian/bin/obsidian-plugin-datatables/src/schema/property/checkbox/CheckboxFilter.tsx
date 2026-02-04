import { Menu, MenuItem } from "@sribich/fude"

import { useSchema } from "../../../ui/hooks/useSchema"
import { useViewScopeContext } from "../../../ui/hooks/useViewScope"
import type { PropertyFilterProps } from "../PropertyComponent"
import { type PropertySchemaRepr } from "../property"
import { type CheckboxFilter as CheckboxFilterType } from "./checkbox"

export interface CheckboxFilterProps {
    property: PropertySchemaRepr<"checkbox">
    filter: CheckboxFilterType & { property: string; uuid: string }
}

export const CheckboxFilter = (props: PropertyFilterProps) => {
    const schema = useSchema()
    const viewScope = useViewScopeContext()

    const onAction = (value: string | number) => {
        schema.view.updateFilter(viewScope.schema.uuid, props.filter.uuid, value === "Checked" ? true : false)
    }

    return (
        <Menu onAction={onAction}>
            <MenuItem id="Unchecked">Unchecked</MenuItem>
            <MenuItem id="Checked">Checked</MenuItem>
        </Menu>
    )
}

const contentComponents = {
    IS: (props: PropertyFilterProps) => {
        return props.filter.data ? "Checked" : "Unchecked"
    },
    IS_NOT: (props: PropertyFilterProps) => {
        return props.filter.data ? "Not Checked" : "Not Unchecked"
    },
} as const

export const CheckboxFilterContent = (props: PropertyFilterProps) => {
    const Component = contentComponents[props.filter.kind]

    console.log("in content")

    if (!Component) {
        return "-"
    }

    console.log()

    return <Component {...props} />
}
