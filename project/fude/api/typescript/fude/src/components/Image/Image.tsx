import { type ComponentProps, cloneElement } from "react"

import { transitionStyles } from "../../theme/atomics/transition"
import { useStyles, type VariantProps } from "../../theme/props"
import { mergeProps } from "../../utils/mergeProps"
import { imageStyles } from "./Image.stylex"

export namespace Image {
    export interface Props extends ComponentProps<"img">, VariantProps<typeof imageStyles> {
        blur?: boolean
        zoom?: boolean
    }
}

export const Image = (props: Image.Props) => {
    const { styles } = useStyles(imageStyles, props)

    const originalImage = <img {...mergeProps(props, styles.image(transitionStyles.movement))} />
    let processedImage = originalImage

    if (props.zoom) {
        processedImage = <div {...styles.zoom()}>{processedImage}</div>
    }

    if (props.blur) {
        return (
            <div {...styles.wrapper()}>
                {processedImage}
                {cloneElement(originalImage, styles.blur())}
            </div>
        )
    }

    if (props.zoom) {
        return <div {...styles.wrapper}>{processedImage}</div>
    }

    return processedImage
}
