import { Button, Dialog, DialogTrigger, Popover } from "@sribich/fude"
import { useState } from "react"

import { assertProperty } from "../../../ui/hooks/useProperty"
import type { PropertyConfigProps } from "../PropertyComponent"

////////////////////////////////////////////////////////////////////////////////
/// ReferenceConfig
////////////////////////////////////////////////////////////////////////////////
export const ReferenceConfig = (props: PropertyConfigProps) => {
    const property = assertProperty(props.property, "reference")
    const isConfigurable = property.config.target === ""

    const [isOpen, setOpen] = useState(false)

    const onOpenChange = (value: boolean) => {
        if (isConfigurable) {
            setOpen(value)
        }
    }

    return (
        <DialogTrigger isOpen={isOpen} onOpenChange={onOpenChange}>
            <Button variant="light" size="sm" fullWidth>
                <div className="flex w-full">
                    <span className="flex-1 text-start">Related to</span>
                    <span className="flex-0">{isConfigurable ? "Not yet set" : property.config.target}</span>
                </div>
            </Button>
            <Popover>
                <ReferenceConfigDialog />
            </Popover>
        </DialogTrigger>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// ReferenceConfigDialog
////////////////////////////////////////////////////////////////////////////////
const ReferenceConfigDialog = () => {
    return <Dialog>...</Dialog>
}

/*
import { useMemo, useState } from "react"

import { Input, Typography } from "antd"

import { FieldKind } from "../../../../schema/field/field.definition"
import { ListItem } from "../../../components/_/list/ListItem"
import { PopEditProps } from "../../../components/_/PopEdit/PopEdit"
import { useFieldKind } from "../../../hooks/useFieldKind"
import { FieldProps } from "../Field"

export const EditReferenceTarget = ({ editor, ...props }: PopEditProps<FieldProps>) => {
    const field = useFieldKind(props.field, FieldKind.Reference)

    const [search, setSearch] = useState("")

    const tags = useMemo(() => {
        const setTarget = (name: string, prettyName: string) => {
            schema.field.updateConfig(field, (config) => {
                config.target = name
                props.close()
            })
            schema.field.changeName(field, prettyName)
        }
        return schema.table.getNames().map((it) => ({
            data: {
                name: it.name.toLocaleLowerCase(),
                prettyName: it.prettyName.toLocaleLowerCase(),
            },
            component: (
                <ListItem key={it.name} onClick={() => setTarget(it.name, it.prettyName)}>
                    <ListItem.Left>
                        <div className="flex flex-col">
                            <b>{it.prettyName}</b>
                            <Typography.Text type="secondary">{it.name}</Typography.Text>
                        </div>
                    </ListItem.Left>
                </ListItem>
            ),
        }))
    }, [editor, field, props])

    const lowercaseSearch = search.toLocaleLowerCase()
    const filteredTags = tags
        .filter((it) => it.data.name.includes(lowercaseSearch) || it.data.prettyName.includes(lowercaseSearch))
        .map((it) => it.component)

    return (
        <div className="flex max-h-[50vh] w-64 flex-col">
            <Input placeholder="Search for a tag" value={search} onChange={(event) => setSearch(event.target.value)} />
            {filteredTags}
        </div>
    )
}
*/
