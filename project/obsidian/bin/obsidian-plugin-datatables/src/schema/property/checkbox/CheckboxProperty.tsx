import { CheckSquare } from "lucide-react"

import type { PropertyComponent } from "../PropertyComponent"
import { CheckboxConfig } from "./CheckboxConfig"
import { CheckboxField } from "./CheckboxField"
import { CheckboxFilter, CheckboxFilterContent } from "./CheckboxFilter"

export const CheckboxProperty = {
    name: "Checkbox",
    icon: CheckSquare,
    config: CheckboxConfig,
    field: CheckboxField,
    filter: CheckboxFilter,
    filterContent: CheckboxFilterContent,
    sort: () => null,
} satisfies PropertyComponent
