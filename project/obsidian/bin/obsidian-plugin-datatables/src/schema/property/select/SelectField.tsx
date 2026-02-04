import type { Immutable } from "@sribich/ts-utils"
import { Box, Button, Chip, Divider, TextField } from "@sribich/fude"

import type { Document } from "../../../index/document"
import { Overlay, OverlayContent, OverlayTrigger } from "../../../ui/components/Overlay/Overlay"
import { assertProperty } from "../../../ui/hooks/useProperty"
import { useSchema } from "../../../ui/hooks/useSchema"
import type { PropertyFieldProps } from "../PropertyComponent"
import type { SelectProperty, SelectPropertyOption } from "./select"

export const SelectField = (props: PropertyFieldProps) => {
    const schema = useSchema()
    const property = assertProperty(props.property, "select")

    const value = schema.property.getValue(property, props.document)
    const option = property.config.options.find((option) => option.id === value)

    return (
        <Overlay>
            <OverlayTrigger className="px-0">
                <Chip rawColor={option?.color || undefined}>{option?.name ?? ""}</Chip>
            </OverlayTrigger>
            <OverlayContent>
                <SelectFieldEdit document={props.document} property={property} selectedOption={option} />
            </OverlayContent>
        </Overlay>
    )
}

/*

export const SelectValue = ({ editor, page, ...props }: FieldProps) => {
    const field = useFieldKind(props.field, FieldKind.Select)
    const value = schema.field.getValue(field, page)

    const valueComponents = field.config.options
        .filter((option) => value?.includes(option.id))
        .map((option) => {
            return (
                <Tag key={option.id} color={option.color}>
                    <span style={{ color: getBestContractColor(option.color) }}>{option.name}</span>
                </Tag>
            )
        })

    return <div className="w-full">{valueComponents}</div>
}
*/

////////////////////////////////////////////////////////////////////////////////
/// SelectFieldEdit
////////////////////////////////////////////////////////////////////////////////
interface SelectFieldEditProps {
    document: Immutable<Document>
    property: SelectProperty
    selectedOption?: SelectPropertyOption | undefined
}

const SelectFieldEdit = (props: SelectFieldEditProps) => {
    const schema = useSchema()

    const change = (id: string) => {
        schema.property.updateValue(props.property, props.document, (current) => {
            return id
        })
    }

    const optionComponents = props.property.config.options.map((option) => (
        <Button onPress={() => change(option.id)}>
            <Chip>{option.name}</Chip>
        </Button>
    ))

    return (
        <Box className="flex flex-col">
            <Box className="flex-0 flex items-center p-1.5">
                {props.selectedOption ? <Chip>{props.selectedOption.name}</Chip> : null}
                <TextField label="" />
            </Box>
            <Divider />
            <Box className="flex flex-1 flex-col p-1.5">
                <span>Select an option or create one</span>
                {optionComponents}
            </Box>
        </Box>
    )
    /*
    <div className="flex flex-col bg-[#262626]">
        <div className="flex-0 flex items-center bg-white/[0.05] p-1.5">
            {valueComponents}
            <Input style={{ background: "none", border: 0, boxShadow: "none" }} bordered={false} />
        </div>
        <div className="flex flex-1 flex-col p-1.5">
            <ListItem.Section title="Select an option or create one" withDivider={false} />
            <DraggableList
                renderChildren={optionComponents}
                onListChange={(newList) => {
                    schema.field.updateConfig(field, (initialConfig, update) => {
                        update.rewriteOptions(newList)(initialConfig)
                    })
                }}
            />
        </div>
    </div>
    */
}

/*
export const SelectValueEdit = ({ editor, page, ...props }: FieldProps) => {
    const field = useFieldKind(props.field, FieldKind.Select)
    const value = schema.field.getValue(field, page)

    const createUpdater = (id: string) => {
        return () => {
            schema.field.updateValue(field, page, (oldConfig = []) => {
                if (!oldConfig?.find((it) => it === id)) {
                    oldConfig?.push(id)
                }
                return oldConfig
            })
        }
    }

    const removeValue = (option: (typeof field.config.options)[number]) => {
        schema.field.updateValue(field, page, (prev) => {
            return prev.filter((it) => it !== option.id)
        })
    }

    const optionComponents: Array<DragChild<SelectFieldConfig["options"][number]>> = field.config.options.map(
        (option) => ({
            key: option.name,
            data: option,
            render: (startDrag) => {
                return (
                    <ListItem key={option.id} onClick={createUpdater(option.id)}>
                        <ListItem.Left>
                            <HolderOutlined className="mr-2 cursor-pointer" onPointerDown={startDrag} />
                            <Tag color={option.color}>
                                <span style={{ color: getBestContractColor(option.color) }}>{option.name}</span>
                            </Tag>
                        </ListItem.Left>
                    </ListItem>
                )
            },
        }),
    )

    const valueComponents = field.config.options
        .filter((option) => value?.includes(option.id))
        .map((option) => {
            const contrast = getBestContractColor(option.color)
            return (
                <Tag key={option.id} color={option.color}>
                    <span style={{ color: contrast }}>{option.name}</span>
                    <BasaltButton
                        size="small"
                        className="ml-1 w-auto !p-0 text-xs hover:opacity-50"
                        style={{ color: contrast }}
                        type="link"
                        onClick={() => removeValue(option)}
                    >
                        x
                    </BasaltButton>
                </Tag>
            )
        })

    
}
*/
