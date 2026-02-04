/* import { ComponentPropsWithoutRef, ElementRef, , ReactElement } from "react"

import { DndContext, DragEndEvent, PointerSensor, useSensor } from "@dnd-kit/core"
import { restrictToParentElement } from "@dnd-kit/modifiers"
import { SortableContext, verticalListSortingStrategy } from "@dnd-kit/sortable"

import { cn } from "../../../util/utils"
import { DragListItemProps } from "./DragListItem"

type Props = Omit<ComponentPropsWithoutRef<"div">, "onDragEnd" | "children"> & {
    children: ReactElement<DragListItemProps>[]
    onDragEnd(event: DragEndEvent): void
}

export const DragListBody = <ElementRef<"div">, Props>(
    ({ children = [], className, onDragEnd, ...props }, ref) => {
        const sensor = useSensor(PointerSensor, { activationConstraint: { distance: 5 } })

        if (children && !Array.isArray(children)) {
            children = [children]
        }

        const itemIds = children.map((child) => child.props.dragId)

        return (
            <div ref={ref} className={cn("flex flex-col animate-none", className)} {...props}>
                <DndContext sensors={[sensor]} modifiers={[restrictToParentElement]} onDragEnd={onDragEnd}>
                    <SortableContext items={itemIds} strategy={verticalListSortingStrategy}>
                        {children}
                    </SortableContext>
                </DndContext>
            </div>
        )
    },
)

DragListBody.displayName = "DragListBody"
 */
