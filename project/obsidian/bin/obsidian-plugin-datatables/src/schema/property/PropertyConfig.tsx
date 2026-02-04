import type { Immutable } from "@sribich/ts-utils"
import { Button, Dialog, DialogTrigger, Divider, Input, Popover, TextField } from "@sribich/fude"
import { type KeyboardEvent, createElement } from "react"

import { useSchema } from "../../ui/hooks/useSchema"
import type { PropertyComponent } from "./PropertyComponent"
import { propertyComponents } from "./property-components"
import type { PropertySchema } from "./property-schema"

////////////////////////////////////////////////////////////////////////////////
/// PropertyConfig
////////////////////////////////////////////////////////////////////////////////
interface PropertyConfigProps {
    property: Immutable<PropertySchema>
}

export const PropertyConfig = (props: PropertyConfigProps) => {
    const component = propertyComponents[props.property.kind]

    return (
        <DialogTrigger>
            <Button
                variant="light" /*className="mr-2 flex h-9 w-40 flex-initial cursor-pointer items-center rounded hover:bg-white hover:bg-opacity-5"*/
            >
                <div className="flex w-40 items-center px-2">
                    <div className="mr-2">{createElement(component.icon, {})}</div>
                    <div className="overflow-hidden text-ellipsis whitespace-nowrap">{props.property.name}</div>
                </div>
            </Button>
            <Popover>
                <Dialog>
                    <PropertyConfigView component={component} property={props.property} />
                </Dialog>
            </Popover>
        </DialogTrigger>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// PropertyConfigView
////////////////////////////////////////////////////////////////////////////////
interface PropertyConfigViewProps {
    component: PropertyComponent
    property: Immutable<PropertySchema>
}

const PropertyConfigView = (props: PropertyConfigViewProps) => {
    const schema = useSchema()

    const onKeyUp = ({ key, target }: KeyboardEvent) => {
        if (key === "Enter" && target instanceof HTMLInputElement) {
            schema.property.rename(props.property, target.value)
        }
    }

    const confirmDelete = () => {}

    return (
        <div className="flex max-h-[50vh] w-72 flex-col rounded-md bg-neutral-700 p-2">
            <div className="flex w-full flex-initial flex-col overflow-hidden">
                <TextField defaultValue={props.property.name} onKeyUp={onKeyUp}>
                    <Input />
                </TextField>
            </div>
            <Divider />
            {/*<GridList className="mt-2"></GridList>*/}
            {createElement(props.component.config, { property: props.property })}

            <Divider />
            <Button variant="light" color="danger" onPress={confirmDelete}>
                Delete Field
            </Button>
        </div>
    )
}

/*
<Input /*size="sm"* defaultValue={property.name} /*onBlur={persistPropertyName}* />

<Menu className="mt-2">
                <Popover>
                    <Popover.Trigger asChild>
                        <Menu.Item>
                            <Menu.Item.Text>Type</Menu.Item.Text>
                            <Menu.Item.Extra>
                                {createElement(component.icon, {})}
                                <span className="ml-1">{component.name}</span>
                            </Menu.Item.Extra>
                        </Menu.Item>
                    </Popover.Trigger>
                    <Popover.Content>
                        <PropertyConfigTypePop property={property} />
                    </Popover.Content>
                </Popover>
                {createElement(component.config, { property })}
            </Menu>
            */

/*

import { createElement } from "react"

import { Check } from "lucide-react"

import { PropertyKind } from "./property-kind"
import { PropertySchema } from "../schema-definition"
import { Button } from "../../components/button/Button"
import { Menu } from "../../components/menu/Menu"
import { usePopoverContext } from "../../components/popover/hooks/usePopoverContext"
import { Typography } from "../../components/typography/Typography"
import { assertProperty } from "../../ui/hooks/useProperty"
import { propertyComponents } from "./property-components"

type Props = {
    property: PropertySchema
}

export const PropertyConfigTypePop = (props: Props) => {
    const { close } = usePopoverContext()

    const changeProperty = (kind: PropertyKind) => {
        // await props.schema.changeFieldType(props.field, kind)
        close()
    }

    const entries = Object.entries(propertyComponents).map(([kind, component]) => (
        <Menu.Item key={kind} onClick={() => changeProperty(kind as PropertyKind)}>
            <Menu.Item.Icon>{createElement(component.icon, {})}</Menu.Item.Icon>
            <Menu.Item.Text>{component.name}</Menu.Item.Text>
            <Menu.Item.Extra>{kind === props.property.kind && <Check />}</Menu.Item.Extra>
        </Menu.Item>
    ))

    return (
        <div className="max-h-[50vh] w-72">
            <Typography.Muted>Type</Typography.Muted>
            <div className="mt-1 flex flex-col">{entries}</div>
        </div>
    )
}
*/
