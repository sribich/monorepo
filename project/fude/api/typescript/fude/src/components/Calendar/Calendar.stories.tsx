import preview from "@/preview"

import { stylesToArgTypes } from "../../theme/props"
import { Calendar } from "./Calendar"
import { calendarStyles } from "./Calendar.stylex"

const meta = preview.meta({
    component: Calendar,
    ...stylesToArgTypes(calendarStyles),
})

export const Overview = meta.story((props) => <Calendar {...props} />)
