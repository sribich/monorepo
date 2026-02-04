import type { LucideIcon } from "lucide-react"
import type { FC } from "react"

import { BoardComponent } from "./board/component"
import { CalendarComponent } from "./calendar/component"
import { GalleryComponent } from "./gallery/component"
import { ListComponent } from "./list/component"
import { TableComponent } from "./table/component"
import { TimelineComponent } from "./timeline/component"
import type { ViewKind } from "./view.kind"

export interface ViewComponent {
    icon: LucideIcon
    component: FC
}

export const viewComponents = {
    board: BoardComponent,
    calendar: CalendarComponent,
    gallery: GalleryComponent,
    list: ListComponent,
    table: TableComponent,
    timeline: TimelineComponent,
} as const satisfies Record<ViewKind, ViewComponent>
