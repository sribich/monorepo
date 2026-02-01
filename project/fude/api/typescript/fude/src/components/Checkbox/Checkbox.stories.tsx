import preview from "@/preview"
import { stylesToArgTypes } from "../../theme/props"
import { Checkbox } from "./Checkbox"
import { checkboxStyles } from "./Checkbox.stylex"

const meta = preview.meta({
    title: "Data Entry/Checkbox",
    component: Checkbox,
    ...stylesToArgTypes(checkboxStyles),
})

export const Overview = meta.story((props) => <Checkbox {...props}>Label</Checkbox>)
