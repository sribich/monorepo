import { type ReactNode, type RefObject, use, useMemo } from "react"
import {
    Slider as AriaSlider,
    type SliderProps as AriaSliderProps,
    SliderThumb as AriaSliderThumb,
    type SliderThumbProps as AriaSliderThumbProps,
    SliderTrack as AriaSliderTrack,
    type SliderTrackProps as AriaSliderTrackProps,
} from "react-aria-components"

import { createNewGenericContext } from "../../hooks/context"
import type { CachedStyles } from "../../theme/props"
import { useStyles, type VariantProps } from "../../theme/props"
import { mergeProps } from "../../utils/mergeProps"
import { sliderStyles } from "./Slider.stylex"

//==============================================================================
// Slider Context
//==============================================================================
export const SliderStyleContext = createNewGenericContext<CachedStyles<typeof sliderStyles>>()

//==============================================================================
// Slider
//==============================================================================
export namespace Slider {
    export interface Props
        extends Omit<AriaSliderProps, "onChange" | "onChangeEnd">,
            VariantProps<typeof sliderStyles> {
        ref?: RefObject<HTMLDivElement>

        // TODO: Fix this, variant is a string so as it is isDisabled does nothing.
        //       Need to fix useStyles to check for true or false and make them the
        //       string variant
        isDisabled?: boolean

        onChange: (value: number) => void
        onChangeEnd: (value: number) => void

        renderTrack?: (props: SliderTrack.Props) => ReactNode
        trackGradient: string
    }
}

export const Slider = ({ children, ...props }: Slider.Props) => {
    const [onChange, onChangeEnd] = useMemo(
        () => [
            (value: number | number[]) => {
                if (Array.isArray(value)) {
                    console.error("Slider invariant broken: slider has multiple thumbs")
                } else {
                    props.onChange?.(value)
                }
            },
            (value: number | number[]) => {
                if (Array.isArray(value)) {
                    console.error("Slider invariant broken: slider has multiple thumbs")
                } else {
                    props.onChangeEnd?.(value)
                }
            },
        ],
        [props.onChange, props.onChangeEnd],
    )

    const { styles, values } = useStyles(sliderStyles, props)

    const Track = props.renderTrack ? props.renderTrack : SliderTrack

    return (
        <SliderStyleContext value={{ styles, values }}>
            <AriaSlider
                {...mergeProps(props, styles.slider())}
                onChange={onChange}
                onChangeEnd={onChangeEnd}
            >
                <Track {...styles.trackWrapper()}>
                    <div
                        {...styles.track(
                            props.trackGradient && styles.track.trackGradient(props.trackGradient),
                        )}
                    >
                        <SliderThumb />
                    </div>
                </Track>
            </AriaSlider>
        </SliderStyleContext>
    )
}

// //==============================================================================
// // RangeSlider
// //==============================================================================
// export namespace RangeSlider {
//     export interface Props extends SliderProps, VariantProps<typeof sliderStyles> {
//         // TODO: Fix this, variant is a string so as it is isDisabled does nothing.
//         //       Need to fix useStyles to check for true or false and make them the
//         //       string variant
//         isDisabled?: boolean
//     }
// }
//
// export const RangeSlider = (props: RangeSliderProps) => {
//     const styles = useStyles(sliderStyles, props)
//
//     const { defaultValue = props.defaultValue ? props.defaultValue : [0, 100] } = props
//
//     return (
//         <SliderStyleProvider value={styles}>
//             <SliderView {...props} defaultValue={defaultValue}>
//                 <SliderTrack />
//                 <SliderThumb slot="lower" />
//                 <SliderThumb slot="upper" />
//             </SliderView>
//         </SliderStyleProvider>
//     )
// }

// ////////////////////////////////////////////////////////////////////////////////
// /// SliderView
// ////////////////////////////////////////////////////////////////////////////////
// interface SliderViewProps<T> extends AriaSliderProps<T>, StyleProps {
//     children?: ReactNode
//     /**
//      * The number format the slider represents.
//      */
//     formatOptions?: NumberFormatOptions
//     /**
//      * The position of the label relative to the slider track.
//      * @default "top"
//      */
//     labelPosition?: "top" | "side"
//     /**
//      * The background of the track, specified as steps in a CSS `linear-gradient` background.
//      */
//     trackGradient?: string[]
//     /**
//      * Whether a custom track is being used to remove the default track
//      * background in case the custom track has alpha.
//      */
//     noTrack?: boolean
// }
//
// /**
//  * TODO: Docs
//  */
// const SliderView = <T extends number | number[]>(props: SliderViewProps<T>) => {
//     const { labelPosition = "top", minValue = 0, maxValue = 100 } = props
//
//     return (
//         <div
//             {...mergeProps(
//                 filterDOMProps(props),
//                 groupProps,
//                 styles.slider(
//                     props.labelPosition === "side"
//                         ? styles.slider.labelSide
//                         : styles.slider.labelTop,
//                 ),
//                 styleProps,
//             )}
//         >
//             {props.label && (
//                 <div {...styles.labelWrapper()}>
//                     <label {...mergeProps(labelProps, styles.labelTitle())}>{props.label}</label>
//                     {/*TODO: ContextHelp*/}
//                     {labelPosition === "top" && sliderValue}
//                 </div>
//             )}
//
//             <div
//                 {...mergeProps(
//                     trackProps,
//                     styles.trackWrapper(props.noTrack && styles.track.customTrack),
//                 )}
//                 ref={trackRef}
//             >
//                 {props.children}
//             </div>
//             {labelPosition === "side" && (
//                 <output {...mergeProps(outputProps, styles.labelValueSideWrapper())}>
//                     {sliderValue}
//                 </output>
//             )}
//         </div>
//     )
// }

//==============================================================================
// SliderTrack
//==============================================================================
export namespace SliderTrack {
    export interface Props extends Omit<AriaSliderTrackProps, "children"> {
        children: ReactNode
    }
}

export const SliderTrack = (props: SliderTrack.Props) => {
    const { styles } = use(SliderStyleContext)

    return (
        <AriaSliderTrack
            {...mergeProps(
                props,
                // styles.track(props.noTrack && styles.track.customTrack),
                /*
                {
                    //
                    style: props.trackGradient
                        ? {
                              "--track-empty": `linear-gradient(to right, ${props.trackGradient?.join(
                                  ",",
                              )})`,
                          }
                        : {},
                },
                */
            )}
        />
    )
}

//==============================================================================
// SliderThumb
//==============================================================================
export namespace SliderThumb {
    export interface Props extends AriaSliderThumbProps {}
}

export const SliderThumb = (props: SliderThumb.Props) => {
    const { styles } = use(SliderStyleContext)

    return <AriaSliderThumb {...mergeProps(props, styles.thumb())} />
}
