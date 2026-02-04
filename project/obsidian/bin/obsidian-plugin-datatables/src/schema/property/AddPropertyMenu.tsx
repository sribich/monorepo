import { Button, Heading, Menu, MenuItem, MenuTrigger, MenuSection } from "@sribich/fude"
import { Plus } from "lucide-react"
import { type Key, createElement, useState } from "react"

import { useSchema } from "../../ui/hooks/useSchema"
import { propertyComponents } from "./property-components"
import { isValidPropertyKind } from "./property-kind"

////////////////////////////////////////////////////////////////////////////////
/// AddPropertyMenu
////////////////////////////////////////////////////////////////////////////////
export const AddPropertyMenu = () => {
    const schema = useSchema()

    const [isOpen, setOpen] = useState(false)

    /*
    const entries = Object.entries(propertyComponents)
        // .filter(([_, component]) => component.creatable ?? true)
        .map(([kind, component]) => (
            <Menu key={kind}>
                <Menu.Item onClick={() => addProperty(kind as PropertyKind)}>
                    <Menu.Item.Icon>{}</Menu.Item.Icon>
                    <Menu.Item.Text>{component.name}</Menu.Item.Text>
                </Menu.Item>
            </Menu>
        ))

    return (
        <div>
            <div className="flex flex-col mb-1 px-1.5">
                <Typography.Muted>Type</Typography.Muted>
            </div>
            <div className="flex flex-col">{entries}</div>
        </div>
    )
    */

    const onAction = (key: Key) => {
        const entry = Object.entries(propertyComponents).find(([kind]) => kind === key)

        if (!entry) {
            throw new Error(`Attempted to create property of unknown type using menu key '${key}'.`)
        }

        if (!isValidPropertyKind(entry[0])) {
            throw new Error(`Attempted to create property of unknown type '${entry[0]}'.`)
        }

        schema.property.create(entry[0])

        setOpen(false)
    }

    const entries = Object.entries(propertyComponents).map(([kind, component]) => (
        <MenuItem key={kind} id={kind}>
            {createElement(component.icon, { className: "h-4 w-4 mr-2" })}
            {component.name}
        </MenuItem>
    ))

    return (
        <MenuTrigger isOpen={isOpen} onOpenChange={setOpen}>
            <Button variant="light">
                <Plus className="mr-1" />
                Add a property
            </Button>
            <Menu onAction={onAction} className="p-2" size="sm">
                <MenuSection>
                    <Heading>Type</Heading>
                    {entries}
                </MenuSection>
            </Menu>
        </MenuTrigger>
    )
}
