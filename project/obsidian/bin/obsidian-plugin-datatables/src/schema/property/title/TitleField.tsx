import { TextField } from "@sribich/fude"
import { type KeyboardEvent, type MouseEvent } from "react"

import {
    Overlay,
    OverlayContent,
    OverlayTrigger,
    useOverlayContext,
} from "../../../ui/components/Overlay/Overlay"
import { assertProperty } from "../../../ui/hooks/useProperty"
import { useSchema } from "../../../ui/hooks/useSchema"
import type { PropertyFieldProps } from "../PropertyComponent"

////////////////////////////////////////////////////////////////////////////////
/// TitleField
////////////////////////////////////////////////////////////////////////////////
export const TitleField = (props: PropertyFieldProps) => {
    const property = assertProperty(props.property, "title")

    const schema = useSchema()
    const value =
        schema.property.getValue(property, props.document) ||
        props.document.path.split("/").at(-1)?.split(".").at(0)

    if (!value) {
        return "Error"
    }

    const onClick = (event: MouseEvent) => {
        if (event.altKey || event.shiftKey || event.ctrlKey) {
            schema.document.navigateTo(props.document)

            return true
        }

        return false
    }

    return (
        <Overlay>
            <OverlayTrigger className="px-0" onClick={onClick}>
                <TextField
                    aria-label={`${property.name} title`}
                    className="flex-auto !p-0"
                    placeholder="Empty"
                    value={value}
                />
            </OverlayTrigger>
            <OverlayContent>
                <TitleFieldEdit {...props} initialValue={value} />
            </OverlayContent>
        </Overlay>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// TitleFieldEdit
////////////////////////////////////////////////////////////////////////////////
interface TitleFieldEditProps extends PropertyFieldProps {
    initialValue: string
}

const TitleFieldEdit = (props: TitleFieldEditProps) => {
    const schema = useSchema()
    const context = useOverlayContext()

    const property = assertProperty(props.property, "title")

    const onKeyUp = ({ key, target }: KeyboardEvent) => {
        if (key === "Enter" && target instanceof HTMLInputElement) {
            schema.property.updateValue(property, props.document, () => target.value)
            context.toggleOpen()
        }
    }

    return (
        <TextField
            className="flex-auto !p-0"
            placeholder="Empty"
            defaultValue={props.initialValue}
            autoFocus
            onKeyUp={onKeyUp}
        />
    )
}
