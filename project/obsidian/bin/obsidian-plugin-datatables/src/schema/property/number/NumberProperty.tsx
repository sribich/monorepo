import { Hash } from "lucide-react"

import { type PropertyComponent } from "../PropertyComponent"
import { NumberConfig } from "./NumberConfig"
import { NumberField } from "./NumberField"

export const NumberProperty = {
    name: "Number",
    icon: Hash,
    config: NumberConfig,
    field: NumberField,
    filter: () => null,
    filterContent: () => null,
    sort: () => null,
} as const satisfies PropertyComponent
