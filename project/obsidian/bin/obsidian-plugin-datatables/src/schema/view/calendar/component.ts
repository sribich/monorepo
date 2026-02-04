import { Calendar } from "lucide-react"

import { ViewComponent } from "../components"
import { CalendarView } from "./CalendarView"

export const CalendarComponent: ViewComponent = {
    icon: Calendar,
    component: CalendarView,
}
