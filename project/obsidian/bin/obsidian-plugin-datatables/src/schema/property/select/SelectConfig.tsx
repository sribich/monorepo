import type { Immutable } from "@sribich/ts-utils"
import {
    Box,
    Button,
    Chip,
    DialogTrigger,
    Divider,
    GridList,
    GridListItem,
    Popover,
    Sketch,
    TextField,
} from "@sribich/fude"
import { MoreHorizontal } from "lucide-react"
import { type KeyboardEvent, useState } from "react"

import { assertProperty } from "../../../ui/hooks/useProperty"
import { useSchema } from "../../../ui/hooks/useSchema"
import type { PropertyConfigProps } from "../PropertyComponent"
import type { PropertySchemaRepr } from "../property"
import type { SelectPropertyOption } from "./select"

export const SelectConfig = (props: PropertyConfigProps) => {
    const schema = useSchema()
    const property = assertProperty(props.property, "select")

    const [isAddingOption, setAddingOption] = useState(false)

    const addOption = (e: KeyboardEvent) => {
        const { key, target } = e

        if (key === "Enter" && target instanceof HTMLInputElement) {
            schema.property.updateConfig(property, (config, morph) => {
                morph.addOption(target.value)(config)
            })

            setAddingOption(false)
        }
    }

    const onAction = (key: string | number) => {}

    return (
        <>
            <div>
                <span>Options</span>
            </div>
            {/*
            <Menu.Item>
                <Menu.Item.Text>Sort</Menu.Item.Text>
                <Menu.Item.Extra arrow>Manual</Menu.Item.Extra>
            </Menu.Item>
            <Menu.Section />
            */}
            <GridList aria-label="" items={property.config.options} onAction={onAction} className="flex flex-col gap-2">
                {(item) => {
                    return (
                        <GridListItem id={item.id}>
                            <DialogTrigger>
                                <Button variant="light" size="sm" radius="sm" className="w-full">
                                    <div className="flex w-full">
                                        <div className="flex flex-1 items-center justify-start">
                                            <Chip rawColor={item.color}>{item.name}</Chip>
                                        </div>
                                        <Button variant="light" size="sm" radius="sm" className="flex-0">
                                            <MoreHorizontal />
                                        </Button>
                                    </div>
                                </Button>
                                <Popover>
                                    <SelectConfigPopover property={property} option={item} />
                                </Popover>
                            </DialogTrigger>
                        </GridListItem>
                    )
                }}
            </GridList>
            {isAddingOption ? (
                <TextField autoFocus label="" onKeyUp={addOption} />
            ) : (
                <Button variant="light" size="sm" color="primary" onPress={() => setAddingOption(true)}>
                    Add option
                </Button>
            )}
        </>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// SelectConfigPopover
////////////////////////////////////////////////////////////////////////////////
interface SelectConfigPopoverProps {
    option: Immutable<SelectPropertyOption>
    property: PropertySchemaRepr<"select">
}

const SelectConfigPopover = ({ option, property }: SelectConfigPopoverProps) => {
    const schema = useSchema()

    const updateOptionName = ({ key, target }: KeyboardEvent) => {
        if (key === "Enter" && target instanceof HTMLInputElement) {
            schema.property.updateConfig(property, (config, morph) => {
                morph.renameOption(option.id, target.value)(config)
            })
        }
    }

    const updateColor = (color: string) => {
        schema.property.updateConfig(property, (config, morph) => {
            morph.color(option.id, color)(config)
        })
    }

    return (
        <Box className="flex max-h-[50vh] w-72 flex-col" padding="2" shadow="md" rounded="md">
            <TextField className="w-full" label="" defaultValue={option.name} onKeyUp={updateOptionName} />
            <Divider />
            <DialogTrigger>
                <Button>Change Color</Button>
                <Popover>
                    <Box>
                        <Sketch color={option.color} onCommit={updateColor} />
                    </Box>
                </Popover>
            </DialogTrigger>
            <Divider />
            <Button color="danger" variant="light">
                Delete
            </Button>
        </Box>
    )
}
