/* import { ComponentPropsWithoutRef, ElementRef } from "react"

import { ChevronRight } from "lucide-react"

import { cn } from "../../util/utils"

type Props = ComponentPropsWithoutRef<"div"> & {
    arrow?: boolean
}

export const MenuItemExtra = <ElementRef<"div">, Props>(({ arrow, children, className, ...props }, ref) => {
    return (
        <div ref={ref} className={cn("flex items-center flex-0", className)} {...props}>
            {children}
            {arrow && <ChevronRight size="16" className="ml-1" />}
        </div>
    )
})

MenuItemExtra.displayName = "MenuItemExtra"
 */
