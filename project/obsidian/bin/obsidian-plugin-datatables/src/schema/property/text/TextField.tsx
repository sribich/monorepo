import { TextField as UiTextField } from "@sribich/fude"
import { type KeyboardEvent, useRef } from "react"

import {
    Overlay,
    OverlayContent,
    OverlayTrigger,
    useOverlayContext,
} from "../../../ui/components/Overlay/Overlay"
import { assertProperty } from "../../../ui/hooks/useProperty"
import { useSchema } from "../../../ui/hooks/useSchema"
import type { PropertyFieldProps } from "../PropertyComponent"

export const TextField = (props: PropertyFieldProps) => {
    const property = assertProperty(props.property, "text")

    const schema = useSchema()
    const value = schema.property.getValue(property, props.document)

    return (
        <Overlay>
            <OverlayTrigger className="px-0">
                <UiTextField
                    aria-label={`${property.name} text input`}
                    className="flex-auto !p-0"
                    placeholder="Empty"
                    value={value}
                />
            </OverlayTrigger>
            <OverlayContent>
                <TextFieldEdit {...props} initialValue={value} />
            </OverlayContent>
        </Overlay>
    )
}
// asChild
// variant="ghost"

////////////////////////////////////////////////////////////////////////////////
/// TextFieldEdit
////////////////////////////////////////////////////////////////////////////////
interface TextFieldEditProps extends PropertyFieldProps {
    initialValue: string
}

const TextFieldEdit = (props: TextFieldEditProps) => {
    const schema = useSchema()
    const context = useOverlayContext()

    const property = assertProperty(props.property, "text")

    const ref = useRef<HTMLInputElement>(null)

    const onKeyUp = ({ key, target }: KeyboardEvent) => {
        if (key === "Enter" && target instanceof HTMLInputElement) {
            schema.property.updateValue(property, props.document, () => target.value)
            context.toggleOpen()
        }
    }

    return (
        <UiTextField
            ref={ref}
            aria-label={`${property.name} text input`}
            className="flex-auto !p-0"
            placeholder="Empty"
            defaultValue={props.initialValue}
            autoFocus
            onKeyUp={onKeyUp}
        />
    )
}
