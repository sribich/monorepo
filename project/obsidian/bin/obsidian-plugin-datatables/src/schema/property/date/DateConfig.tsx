import { Button, Dialog, DialogTrigger, Flex, Popover, TypographyText } from "@sribich/fude"

import { assertProperty } from "../../../ui/hooks/useProperty"
import type { PropertyConfigProps } from "../PropertyComponent"

export const DateConfig = (props: PropertyConfigProps) => {
    return (
        <>
            <DateFormatConfig {...props} />
            <TimeFormatConfig {...props} />
        </>
    )
}

const DateFormatConfig = (props: PropertyConfigProps) => {
    const property = assertProperty(props.property, "date")

    /*
    const onChange = (value: (typeof dateFormats)[number]) => {
        if (value !== field.config.dateFormat) {
            schema.updateField(field, (initialConfig) => {
                initialConfig.dateFormat = value
            })
        }

        close()
    }
    */

    return (
        <DialogTrigger>
            <Button size="sm" variant="light" fullWidth>
                <Flex className="w-full">
                    <span className="block flex-1 text-start">Date format</span>
                    <TypographyText color="secondary" size="sm">
                        {property.config.dateFormat}
                    </TypographyText>
                </Flex>
            </Button>
            <Popover>
                <Dialog></Dialog>
            </Popover>
        </DialogTrigger>
    )

    /*
    const components = dateFormats.map((format, index) => (
        <Button
            key={index}
            className="px-1 text-left hover:bg-white hover:bg-opacity-5"
            type="ghost"
            onClick={() => onChange(format)}
        >
            <div className="flex">
                <div className="flex flex-1 items-center">
                    <span>{format}</span>
                </div>
                <div className="flex-0 flex items-center">
                    {field.config.dateFormat === format && <CheckOutlined />}
                </div>
            </div>
        </Button>
    ))

    return <div className="flex max-h-[50vh] w-48 flex-col">{components}</div>
    */
}

const TimeFormatConfig = (props: PropertyConfigProps) => {
    const property = assertProperty(props.property, "date")

    /*
    const onChange = (value: (typeof timeFormats)[number]) => {
        if (value !== field.config.timeFormat) {
            schema.updateField(field, (initialConfig) => {
                initialConfig.timeFormat = value
            })
        }

        close()
    }
    */

    return (
        <DialogTrigger>
            <Button size="sm" variant="light" fullWidth>
                <Flex className="w-full">
                    <span className="block flex-1 text-start">Time format</span>
                    <TypographyText color="secondary" size="sm">
                        {property.config.timeFormat}
                    </TypographyText>
                </Flex>
            </Button>
            <Popover>
                <Dialog></Dialog>
            </Popover>
        </DialogTrigger>
    )

    /*
    const components = timeFormats.map((format, index) => (
        <Button
            key={index}
            className="px-1 text-left hover:bg-white hover:bg-opacity-5"
            type="ghost"
            onClick={() => onChange(format)}
        >
            <div className="flex">
                <div className="flex flex-1 items-center">
                    <span>{format}</span>
                </div>
                <div className="flex-0 flex items-center">
                    {field.config.timeFormat === format && <CheckOutlined />}
                </div>
            </div>
        </Button>
    ))

    return <div className="flex max-h-[50vh] w-48 flex-col">{components}</div>
    */
}
