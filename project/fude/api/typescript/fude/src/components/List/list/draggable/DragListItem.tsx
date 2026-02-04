/* import { ComponentPropsWithoutRef, ElementRef, , ReactNode } from "react"

import { DraggableAttributes } from "@dnd-kit/core"
import { useSortable } from "@dnd-kit/sortable"
import { CSS } from "@dnd-kit/utilities"

import { cn } from "../../../util/utils"

export type DragListItemProps =
    | (ComponentPropsWithoutRef<"div"> & {
          dragId: string
          render?: never
      })
    | (Omit<ComponentPropsWithoutRef<"div">, "children"> & {
          dragId: string
          children?: never
          render: (triggers: DraggableAttributes) => ReactNode
      })

export const DragListItem = <ElementRef<"div">, DragListItemProps>(
    ({ children, className, dragId, render, ...props }, ref) => {
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
            <div ref={setRef} className={cn("flex", className)} style={style} {...props} {...renderProps}>
                {renderChild}
            </div>
        )
    },
)

DragListItem.displayName = "DragListItem"
 */
