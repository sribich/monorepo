/* import { ComponentPropsWithoutRef, ElementRef, ReactNode } from "react"

import { DraggableAttributes } from "@dnd-kit/core"
import { useSortable } from "@dnd-kit/sortable"
import { CSS } from "@dnd-kit/utilities"
import { GripVertical } from "lucide-react"

import { cn } from "../../../util/utils"
import { useOrderedMenu } from "../hook/useOrderedMenu"

export type OrderedItemProps =
    | (ComponentPropsWithoutRef<"div"> & {
          dragId: string
          render?: never
          grip?: boolean
      })
    | (Omit<ComponentPropsWithoutRef<"div">, "children"> & {
          dragId: string
          children?: never
          render: (triggers: DraggableAttributes) => ReactNode
          grip?: boolean
      })

export const OrderedItem = <ElementRef<"div">, OrderedItemProps>(
    ({ children, className, dragId, grip = false, render, ...props }, ref) => {
        useOrderedMenu(dragId)

        const { attributes, listeners, setNodeRef, transform, transition } = useSortable({
            id: dragId,
        })

        const renderProps = render ? {} : { ...attributes, ...listeners }
        const renderChild = render ? render({ ...attributes, ...listeners }) : children

        const style = {
            ...props.style,
            cursor: "move",
            transition,
            transform: CSS.Transform.toString(transform),
        }

        const setRef = (el: HTMLDivElement | null) => {
            setNodeRef(el)

            if (!ref) {
                return
            }

            if ("current" in ref) {
                ref.current = el
            } else {
                ref(el)
            }
        }

        return (
            <div
                ref={setRef}
                className={cn("flex", className)}
                style={style}
                data-menu-item
                {...props}
                {...renderProps}
            >
                {grip && <GripVertical />}
                {renderChild}
            </div>
        )
    },
)

OrderedItem.displayName = "OrderedItem"
 */
