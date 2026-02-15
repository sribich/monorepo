import type { BaseCollection } from "@react-aria/collections"
import type { CollectionBase, PressEvents } from "@react-types/shared"
import { cardMarker } from "@sribich/fude-theme/markers.stylex"
import { type ReactNode, type Ref, type RefObject, use, useRef } from "react"
import {
    type AriaFocusRingProps,
    type HoverProps,
    useButton,
    useFocusRing,
    useHover,
} from "react-aria"
import {
    Collection,
    CollectionBuilder,
    CollectionRendererContext,
    createLeafComponent,
} from "react-aria-components"

import { useObjectRef } from "../../hooks/useObjectRef"
import { useRenderProps } from "../../hooks/useRenderProps"
import { useStyles, type VariantProps } from "../../theme/props"
import { MultiProvider } from "../MultiProvider"
import { mergeProps } from "../../utils/mergeProps"
import type { RenderProps } from "../../utils/props"
import { BoxContext } from "../Box/Box"
import { HeadingProvider } from "../Heading/Heading"
import { CardStyleContext, cardStyles } from "./Card.stylex"

const useCardStyles = (props: Card.Props) => {
    const styleTypeCheck = useRef(false)

    if (CardStyleContext.isProvided()) {
        styleTypeCheck.current = true

        return use(CardStyleContext)
    }

    if (styleTypeCheck.current) {
        throw new Error(
            `Component must be rendered exlusively within a collection, or outside. It may not be conditionally rendered inside or outside of a collection.`,
        )
    }

    return useStyles(cardStyles, props)
}

//===============================================================================
// CardView
//===============================================================================
export namespace CardView {
    export interface Props extends CollectionBase<Card.Props>, VariantProps<typeof cardStyles> {
        ref?: RefObject<HTMLDivElement>
    }
}

export const CardView = <T,>(props: CardView.Props) => {
    const styles = useStyles(cardStyles, props)

    return (
        <CardStyleContext value={styles}>
            <CollectionBuilder content={<Collection {...props} />}>
                {(collection: BaseCollection<Card.Props>) => (
                    <InnerCardView props={props} collection={collection} />
                )}
            </CollectionBuilder>
        </CardStyleContext>
    )
}

//===============================================================================
// InnerCardView
//===============================================================================
export namespace InnerCardView {
    export interface Props {
        collection: BaseCollection<Card.Props>
        props: CardView.Props
    }
}

export const InnerCardView = (props: InnerCardView.Props) => {
    const ref = useObjectRef(props.props.ref)

    const { CollectionRoot } = use(CollectionRendererContext)
    const { styles, values } = useStyles(cardStyles, {})

    return (
        <div {...styles.wrapper()} ref={ref}>
            <CollectionRoot collection={props.collection} scrollRef={ref} />
        </div>
    )
}

//==============================================================================
// Card
//==============================================================================
export namespace Card {
    export interface Props
        extends AriaFocusRingProps,
            HoverProps,
            RenderProps<RenderPropValues>,
            VariantProps<typeof cardStyles> {
        children: ReactNode
    }

    export interface PressableProps extends Props, PressEvents {
        isPressable: true
    }

    export interface RenderPropValues {
        isHovered: boolean
        isFocused: boolean
        isFocusVisible: boolean
        isPressed: boolean
    }
}

export const Card = createLeafComponent("item", (props: Card.Props | Card.PressableProps) => {
    const ref = useObjectRef<HTMLButtonElement>()

    const { isFocused, isFocusVisible, focusProps } = useFocusRing(props)
    const { isHovered, hoverProps } = useHover(props)

    const { buttonProps, isPressed } = useButton(props, ref)

    const { children, ...renderProps } = useRenderProps(props, {
        isHovered,
        isFocused,
        isFocusVisible,
        isPressed,
    })

    const { styles, values } = useStyles(cardStyles, props)

    const Component = "isPressable" in props ? "button" : "article"

    return (
        <Component
            {...mergeProps(
                buttonProps,
                focusProps,
                hoverProps,
                styles.card(cardMarker),
                renderProps,
            )}
        >
            <MultiProvider
                values={[
                    [CardStyleContext, { styles, values }],
                    [
                        BoxContext,
                        {
                            slots: {
                                menuArea: {
                                    ...styles.menuArea(),
                                },
                            },
                        },
                    ],
                    [
                        HeadingProvider,
                        {
                            level: 4,
                        },
                    ],
                ]}
            >
                {children}
            </MultiProvider>
        </Component>
    )
})

//==============================================================================
// CardHeader
//==============================================================================
export namespace CardHeader {
    export interface Props {
        ref?: Ref<HTMLDivElement>

        children: ReactNode
    }
}

export const CardHeader = (props: CardHeader.Props) => {
    const ref = useObjectRef(props.ref)

    const { styles } = CardStyleContext.use()

    return (
        <div {...styles.header()} ref={ref}>
            {props.children}
        </div>
    )
}

//==============================================================================
// CardBody
//==============================================================================
export namespace CardBody {
    export interface Props {
        children: ReactNode
    }
}

export const CardBody = (props: CardBody.Props) => {
    const { styles } = CardStyleContext.use()

    return <div {...styles.body()}>{props.children}</div>
}

//==============================================================================
// CardFooter
//==============================================================================
export namespace CardFooter {
    export interface Props {
        children: ReactNode
    }
}

export const CardFooter = (props: CardFooter.Props) => {
    const { styles } = CardStyleContext.use()

    return <div {...styles.footer()}>{props.children}</div>
}
