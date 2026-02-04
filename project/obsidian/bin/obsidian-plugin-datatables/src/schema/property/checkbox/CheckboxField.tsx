import { Checkbox } from "@sribich/fude"

import { assertProperty } from "../../../ui/hooks/useProperty"
import { useSchema } from "../../../ui/hooks/useSchema"
import type { PropertyFieldProps } from "../PropertyComponent"

/**
 * TODO: Documentation
 */
export const CheckboxField = (props: PropertyFieldProps) => {
    const schema = useSchema()

    const property = assertProperty(props.property, "checkbox")
    const value = schema.property.getValue(property, props.document)

    const toggle = () => {
        schema.property.updateValue(property, props.document, () => !value)
    }

    return <Checkbox aria-label={`${property.name} checkbox`} isSelected={value} onChange={toggle} />
}
