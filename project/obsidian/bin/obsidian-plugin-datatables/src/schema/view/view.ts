import { boardView } from "./board/board.view"
import { calendarView } from "./calendar/calendar.view"
import { galleryView } from "./gallery/gallery.view"
import { listView } from "./list/list.view"
import { tableView } from "./table/table.view"
import { timelineView } from "./timeline/timeline.view"
import { type ViewKind } from "./view.kind"

export const views = [
    boardView,
    calendarView,
    galleryView,
    listView,
    tableView,
    timelineView,
] as const

export type ViewRepr<TKind extends ViewKind> = Extract<(typeof views)[number], { kind: TKind }>

/**
 * Returns the view definition for a given `kind`.
 *
 * ? ViewRepr<`${TKind}`> is used to coerce enum types into their string
 * ? representation. Changing it will result in us returning never.
 */
export const getView = <TKind extends ViewKind>(kind: TKind): ViewRepr<`${TKind}`> => {
    const definition = views.find((it) => it.kind === kind)

    if (!definition) {
        throw new Error(`Attempted to get view metadata for unknown kind: ${kind}`)
    }

    return definition as ViewRepr<`${TKind}`>
}
