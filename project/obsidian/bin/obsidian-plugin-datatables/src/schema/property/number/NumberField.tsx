export const NumberField = () => {
    return null
}

/*
import { PropertyKind } from "../../../../schema/property/property-kind"
import { Input } from "../../../components/input/Input"
import { Overlay } from "../../../components/overlay/Overlay"
import { useSchema } from "../../../context/editor"
import { assertProperty } from "../../../hooks/useProperty"
import type { PropertyFieldProps } from "../PropertyComponent"

export const NumberField = (props: PropertyFieldProps) => {
    const schema = useSchema()

    const property = assertProperty(props.property, PropertyKind.Number)

    const value = schema.property.getValue(property, props.document)

    return (
        <Overlay>
            <Overlay.Trigger asChild>
                <Input value={value} />
            </Overlay.Trigger>
            <Overlay.Content>...</Overlay.Content>
        </Overlay>
    )
}
*/

/*
// TODO: Number formatting option
// TODO: Allow custom options to be injected into the edit popover
// TODO: Require the value to only be a number
export const NumberValue = (props: FieldProps) => {
    const field = useFieldKind(props.field, FieldKind.Number)
    const value = props.schema.field.getValue(field, props.page)

    return (
        <Input
            className="flex-auto !p-0"
            style={{ background: "none", border: 0, boxShadow: "none" }}
            placeholder="Empty"
            bordered={false}
            value={value}
        />`
    )`
}

export const NumberValueEdit = (props: FieldProps) => {
    const { editor, page } = props

    const field = useFieldKind(props.field, FieldKind.Number)

    const onBlur = async (event: ChangeEvent<HTMLInputElement>) => {
        const number = Number.parseInt(event.target.value || "0") || 0

        props.schema.field.updateValue(field, page, () => number)
    }

    return (
        <div className="flex items-center flex-auto p-2 rounded h-9 bg-neutral-800">
            <InputNumber
                className="flex-auto !p-0"
                style={{ background: "none", border: 0, boxShadow: "none" }}
                placeholder=""
                controls={false}
                bordered={false}
                defaultValue={schema.field.getValue(field, page)}
                onBlur={onBlur}
                onPressEnter={maybeBlur}
            />
        </div>
    )
}

import { SyntheticEvent } from "react"

export const maybeBlur = (event: SyntheticEvent) => {
    if (event.target instanceof HTMLElement) {
        event.target.blur()
    }
}

*/
