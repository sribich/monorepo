import { AnimatePresence, clamp, motion } from "framer-motion"
import type { Key } from "react-aria"
import type { ReactElement } from "react"

import { useStyles, type VariantProps } from "../../theme/props"
import { rippleStyles } from "./Ripple.styles"

////////////////////////////////////////////////////////////////////////////////
/// Component
////////////////////////////////////////////////////////////////////////////////
export namespace Ripple {
    export interface Props extends VariantProps<typeof rippleStyles> {
        ripples: Ripple[]
        clearRipple(key: Key): void
    }

    export interface Ripple {
        key: Key
        size: number
        x: number
        y: number
    }
}

export const Ripple = (props: Ripple.Props): Array<ReactElement> => {
    const { styles } = useStyles(rippleStyles, props)

    return props.ripples.map((ripple) => (
        <AnimatePresence key={ripple.key}>
            <motion.span
                {...styles.ripple(styles.ripple.position(ripple))}
                initial={{ transform: "scale(0.0)", opacity: 0.4 }}
                animate={{ transform: "scale(1.35)", opacity: 0.0 }}
                transition={{ duration: 0.25 }}
                onAnimationComplete={() => {
                    props.clearRipple(ripple.key)
                }}
            />
        </AnimatePresence>
    ))
}
