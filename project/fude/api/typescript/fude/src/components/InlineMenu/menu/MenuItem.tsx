/* import { ComponentPropsWithoutRef, ElementRef } from "react"

import { cn } from "../../util/utils"

type Props = ComponentPropsWithoutRef<"div">

export const MenuItem = <ElementRef<"div">, Props>(({ children, className, ...props }, ref) => {
    return (
        <div
            {...props}
            ref={ref}
            role="button"
            className={cn(
                "flex w-full rounded cursor-pointer px-1.5 py-1 mb-0.5 hover:bg-white hover:bg-opacity-5",
                className,
            )}
            data-menu-item
        >
            {children}
        </div>
    )
})

MenuItem.displayName = "MenuItem"
 */
