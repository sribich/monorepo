import { File } from "lucide-react"

import { type PropertyComponent } from "../PropertyComponent"
import { TitleConfig } from "./TitleConfig"
import { TitleField } from "./TitleField"

export const TitleProperty = {
    name: "Title",
    icon: File,
    config: TitleConfig,
    field: TitleField,
    filter: () => null,
    filterContent: () => null,
    sort: () => null,
} as const satisfies PropertyComponent
