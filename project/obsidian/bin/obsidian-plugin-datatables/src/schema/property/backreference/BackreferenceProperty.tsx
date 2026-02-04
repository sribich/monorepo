import { ArrowLeft } from "lucide-react"

import type { PropertyComponent } from "../PropertyComponent"
import { BackreferenceConfig } from "./BackreferenceConfig"
import { BackreferenceField } from "./BackreferenceField"

export const BackreferenceProperty = {
    name: "Backreference",
    icon: ArrowLeft,
    config: BackreferenceConfig,
    field: BackreferenceField,
    filter: () => null,
    filterContent: () => null,
    sort: () => null,
} satisfies PropertyComponent
