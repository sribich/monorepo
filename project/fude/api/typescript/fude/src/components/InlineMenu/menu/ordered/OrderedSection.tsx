/* import { ComponentPropsWithoutRef, ElementRef, ReactElement, useEffect, useState } from "react"

import { DndContext, DragEndEvent, PointerSensor, useSensor } from "@dnd-kit/core"
import { restrictToParentElement } from "@dnd-kit/modifiers"
import { SortableContext, verticalListSortingStrategy } from "@dnd-kit/sortable"

import { cn } from "../../../util/utils"
import { OrderedMenuContext } from "../context/OrderedMenuContext"
import { Menu } from "../Menu"
import { OrderedItemProps } from "./OrderedItem"

type Props = Omit<ComponentPropsWithoutRef<"div">, "children" | "onDragEnd"> & {
    children: ReactElement<OrderedItemProps>[]
    onDragEnd(event: DragEndEvent): void
    title?: string
}

/**
 * TODO: Optional <Separator />
 *
export const OrderedSection = <ElementRef<"div">, Props>(
    ({ children = [], className, onDragEnd, title, ...props }, ref) => {
        const sensor = useSensor(PointerSensor, { activationConstraint: { distance: 5 } })

        const [itemIds, setItemIds] = useState([] as string[])

        if (children && !Array.isArray(children)) {
            children = [children]
        }

        return (
            <div ref={ref} className={cn("flex flex-col", className)} {...props}>
                <Menu.Section />
                <DndContext sensors={[sensor]} modifiers={[restrictToParentElement]} onDragEnd={onDragEnd}>
                    <SortableContext items={itemIds} strategy={verticalListSortingStrategy}>
                        <OrderedMenuContext.Provider value={{ setItemIds }}>{children}</OrderedMenuContext.Provider>
                    </SortableContext>
                </DndContext>
            </div>
        )
    },
)

OrderedSection.displayName = "OrderedSection"
 */
