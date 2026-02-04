import type { Immutable } from "@sribich/ts-utils"
import type { LucideIcon } from "lucide-react"
import type { FC } from "react"

import type { Document } from "../../index/document"
import type { PropertySchema } from "./property-schema"

export interface PropertyConfigProps {
    property: Immutable<PropertySchema>
}

export interface PropertyFieldProps {
    property: Immutable<PropertySchema>
    document: Immutable<Document>
}

export interface PropertyFilterProps {
    property: Immutable<PropertySchema>
    filter: any
}

export interface PropertySortProps {
    property: Immutable<PropertySchema>
}

export interface PropertyComponent {
    name: string
    icon: LucideIcon
    config: FC<PropertyConfigProps>
    field: FC<PropertyFieldProps>

    filter: FC<PropertyFilterProps>
    filterContent: FC<PropertyFilterProps>

    sort: FC<PropertySortProps>
}
