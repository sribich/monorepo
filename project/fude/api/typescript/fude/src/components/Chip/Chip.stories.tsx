import preview from "@/preview"

import { stylesToArgTypes } from "../../theme/props"
import { Chip } from "./Chip"
import { chipStyles } from "./Chip.stylex"

const meta = preview.meta({
    title: "Data Display/Chip",
    component: Chip,
    tags: ["autodocs"],
    ...stylesToArgTypes(chipStyles),
})

export const Overview = meta.story({ render: (props) => <Chip {...props}>Button Text</Chip> })
