import { DatePicker } from "@sribich/fude"

import { Overlay, OverlayContent, OverlayTrigger } from "../../../ui/components/Overlay/Overlay"
import { assertProperty } from "../../../ui/hooks/useProperty"
import { useSchema } from "../../../ui/hooks/useSchema"
import type { PropertyFieldProps } from "../PropertyComponent"

export const DateField = (props: PropertyFieldProps) => {
    const schema = useSchema()
    const property = assertProperty(props.property, "date")

    const value = schema.property.getValue(property, props.document)

    const onChange = (date: Date) => {
        schema.property.updateValue(property, props.document, () => ({
            date,
            dateEnd: null,
        }))
    }

    return (
        <Overlay>
            <OverlayTrigger>{value?.date ? new Date(value?.date).toDateString() : null}</OverlayTrigger>
            <OverlayContent>
                <DatePicker
                    onChange={(value) => onChange(value.toDate(Intl.DateTimeFormat().resolvedOptions().timeZone))}
                />
            </OverlayContent>
        </Overlay>
    )
}

/*
import { useState } from "react"

import { DatePicker } from "antd"
import dayjs from "dayjs"

import { getFieldDefinition } from "../../../../schema/field/field"
import { FieldKind } from "../../../../schema/field/field.definition"
import { FieldProps } from "../Field"

export const DateValueEdit = (props: FieldProps) => {
    const { editor, field, page } = props

    if (field.kind !== FieldKind.Date) {
        return null
    }

    const value = props.schema.field.getValue(field, page)

    const performUpdate = (startDate: number) => {
        schema.field.updateValue(field, page, (oldConfig) => {
            return {
                dateStart: startDate,
                dateEnd: oldConfig?.dateEnd ?? null,
                useTime: oldConfig?.useTime ?? false,
            }
        })
    }

    return (
        <div className="flex flex-col bg-[#262626]">
            <DatePicker
                allowClear={false}
                showToday={false}
                onChange={(date) => {
                    if (!date) {
                        return
                    }

                    performUpdate(date?.unix())
                }}
                defaultValue={value?.dateStart ? dayjs.unix(value.dateStart) : dayjs()}
                renderExtraFooter={() => {
                    // TODO: End date
                    // TODO: Include time
                    // TODO: Timezone
                    return null
                }}
            />
        </div>
    )
}
*/
