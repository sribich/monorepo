import { Text } from "lucide-react"

import type { PropertyComponent } from "../PropertyComponent"
import { TextConfig } from "./TextConfig"
import { TextField } from "./TextField"

export const TextProperty = {
    name: "Text",
    icon: Text,
    config: TextConfig,
    field: TextField,
    filter: () => null,
    filterContent: () => null,
    sort: () => null,
} as const satisfies PropertyComponent
