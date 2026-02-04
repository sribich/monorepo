/*
import { ComponentPropsWithoutRef, ElementRef } from "react"

import { cva, type VariantProps } from "class-variance-authority"

import { getBestContractColor } from "../../../../../util/color"
import { cn } from "../../util/utils"

const tagVariants = cva(
    // focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2
    "inline-flex items-center rounded-md border-black border bg-black/10 text-xs pe-2 ps-2 transition-all",
    {
        variants: {
            variant: {
                default: "",
                // "border-transparent bg-primary text-primary-foreground shadow hover:bg-primary/80",
                secondary: "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80",
                destructive:
                    "border-transparent bg-destructive text-destructive-foreground shadow hover:bg-destructive/80",
                outline: "text-foreground",
            },
        },
        defaultVariants: {
            variant: "default",
        },
    },
)

export type TagElement = ElementRef<"div">
export type TagProps = Omit<ComponentPropsWithoutRef<"div">, "color"> &
    VariantProps<typeof tagVariants> & {
        color?: string
    }

export const Tag = TagElement, TagProps>(({ className, color, ...props }, ref) => {
    const customStyle = color
        ? {
              color: getBestContractColor(color),
              backgroundColor: color,
          }
        : {}

    return <div ref={ref} className={cn(tagVariants(props), className)} style={customStyle} {...props} />
})

Tag.displayName = "Tag"
*/
