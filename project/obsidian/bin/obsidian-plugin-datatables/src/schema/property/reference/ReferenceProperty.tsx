import { ArrowRight } from "lucide-react"

import { type PropertyComponent } from "../PropertyComponent"
import { ReferenceConfig } from "./ReferenceConfig"
import { ReferenceField } from "./ReferenceField"

export const ReferenceProperty = {
    name: "Reference",
    icon: ArrowRight,
    config: ReferenceConfig,
    field: ReferenceField,
    filter: () => null,
    filterContent: () => null,
    sort: () => null,
} as const satisfies PropertyComponent
