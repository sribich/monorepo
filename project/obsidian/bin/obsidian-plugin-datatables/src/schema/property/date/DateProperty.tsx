import { Calendar } from "lucide-react"

import type { PropertyComponent } from "../PropertyComponent"
import { DateConfig } from "./DateConfig"
import { DateField } from "./DateField"
import { DateFilter, DateFilterContent } from "./DateFilter"

export const DateProperty = {
    name: "Date",
    icon: Calendar,
    config: DateConfig,
    field: DateField,

    filter: DateFilter,
    filterContent: DateFilterContent,
    // filter: () => null,
    sort: () => null,
} as const satisfies PropertyComponent
