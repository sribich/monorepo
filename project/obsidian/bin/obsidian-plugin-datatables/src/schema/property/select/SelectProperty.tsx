import { ChevronDownCircle } from "lucide-react"

import { type PropertyComponent } from "../PropertyComponent"
import { SelectConfig } from "./SelectConfig"
import { SelectField } from "./SelectField"

export const SelectProperty = {
    name: "Select",
    icon: ChevronDownCircle,
    config: SelectConfig,
    field: SelectField,
    filter: () => null,
    filterContent: () => null,
    sort: () => null,
} as const satisfies PropertyComponent
