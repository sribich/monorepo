import type { PropertyComponent } from "./PropertyComponent"
import { BackreferenceProperty } from "./backreference/BackreferenceProperty"
import { CheckboxProperty } from "./checkbox/CheckboxProperty"
import { DateProperty } from "./date/DateProperty"
import { NumberProperty } from "./number/NumberProperty"
import type { PropertyKind } from "./property-kind"
import { ReferenceProperty } from "./reference/ReferenceProperty"
import { SelectProperty } from "./select/SelectProperty"
import { TextProperty } from "./text/TextProperty"
import { TitleProperty } from "./title/TitleProperty"

export const propertyComponents = {
    text: TextProperty,
    numbers: NumberProperty,
    select: SelectProperty,
    // UnorderedListOutlined -> MultiSelect
    // MonitorOutlined -> Status
    date: DateProperty,
    // PaperClipOutlined -> Attachment
    checkbox: CheckboxProperty,
    // LinkOutlined -> URL
    // MailOutlined -> Email
    // PhoneOutlined -> Phone
    // FunctionOutlined -> Formula
    // FileOutlined -> Relation
    reference: ReferenceProperty,
    backreference: BackreferenceProperty,
    title: TitleProperty,
} as const satisfies Record<PropertyKind, PropertyComponent>
