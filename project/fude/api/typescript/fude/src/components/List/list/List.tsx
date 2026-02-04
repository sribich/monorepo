/*
import { ComponentPropsWithoutRef, ElementRef,  } from "react"

import { cva } from "class-variance-authority"

import { cn } from "../../util/utils"

export const listVariants = cva("", {
    variants: {
        size: {
            default: "",
            sm: "[&>.list-item]:",
            lg: "",
        },
    },
    defaultVariants: {
        size: "default",
    },
})

type Props = ComponentPropsWithoutRef<"div">

export const List = <ElementRef<"div">, Props>(({ className, ...props }, ref) => (
    <div ref={ref} className={cn("", className)} {...props} />
))

List.displayName = "List"

// TODO: Header/Footer
// flex flex-col items-start
*/
