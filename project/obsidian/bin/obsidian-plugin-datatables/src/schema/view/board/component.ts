import { KanbanSquare } from "lucide-react"

import { ViewComponent } from "../components"
import { BoardView } from "./BoardView"

export const BoardComponent: ViewComponent = {
    icon: KanbanSquare,
    component: BoardView,
}
