import type { ElementType, ReactNode, RefObject } from "react"

import { useStyleProps } from "../../hooks/useRenderProps"
import { useStyles, type VariantProps } from "../../theme/props"
import { mergeProps } from "../../utils/mergeProps"
import type { StyleProps } from "../../utils/props"
import { headingLevels, typographyStyles } from "./Typography.stylex"

//==============================================================================
// Heading
//==============================================================================
// export const [useHeadingContext, HeadingProvider] = createControlledContext<
//     TypographyHeading.Props,
//     HTMLHeadingElement
// >()

export namespace TypographyHeading {
    export interface Props extends StyleProps, Omit<VariantProps<typeof typographyStyles>, "size"> {
        children: ReactNode
        level: 1 | 2 | 3 | 4 | 5 | 6
    }
}

export const TypographyHeading = (props: TypographyHeading.Props) => {
    const Component = `h${props.level ?? 2}` as ElementType

    const styleProps = useStyleProps(props, {})
    const { styles } = useStyles(typographyStyles, {
        ...props,
        ...headingLevels[props.level],
    })

    return <Component {...mergeProps(styles.text(), styleProps)}>{props.children}</Component>
}

//==============================================================================
// Text
//==============================================================================
export namespace Text {
    export interface Props extends StyleProps, VariantProps<typeof typographyStyles> {
        children: ReactNode
    }
}

export const Text = (props: Text.Props) => {
    const { styles } = useStyles(typographyStyles, props)
    const styleProps = useStyleProps(props, {})

    return null
}

////////////////////////////////////////////////////////////////////////////////
/// TypographyText
////////////////////////////////////////////////////////////////////////////////
/**
 * @alpha
 */
export interface TypographyTextProps extends StyleProps, VariantProps<typeof typographyStyles> {
    ref?: RefObject<HTMLSpanElement>
    children: ReactNode
}

export const TypographyText = (props: TypographyTextProps) => {
    const { styles } = useStyles(typographyStyles, props)
    const styleProps = useStyleProps(props, {})

    return (
        <span {...mergeProps(styles.text(), styleProps)} ref={props.ref}>
            {props.children}
        </span>
    )
}
