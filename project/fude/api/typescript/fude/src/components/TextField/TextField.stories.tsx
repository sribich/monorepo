import preview from "@/preview"

import { stylesToArgTypes } from "../../theme/props"
import { TextField, textFieldStyles } from "./TextField"

const meta = preview.meta({
    title: "Data Entry/TextField",
    component: TextField,
    ...stylesToArgTypes(textFieldStyles),
})

export const Basic = meta.story({
    render: (props) => (
        <div className="h-full w-full bg-white p-12">
            <TextField label="name" placeholder="Placeholder" {...props} />
        </div>
    ),
})
