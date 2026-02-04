import { GanttChart } from "lucide-react"

import { ViewComponent } from "../components"
import { TimelineView } from "./TimelineView"

export const TimelineComponent: ViewComponent = {
    icon: GanttChart,
    component: TimelineView,
}
