import {
    Slider as RacSlider,
    SliderTrack as RacSliderTrack,
    SliderThumb as RacSliderThumb,
} from "react-aria-components"
import { type RefObject, use } from "react"
import { type VariantProps, useStyles } from "../../theme/props"
import { mergeProps } from "../../utils/mergeProps"
import { SliderStyleContext, sliderStyles } from "./Slider.stylex"
import type { SliderProps, SliderThumbProps, SliderTrackProps } from "react-aria-components"

//==============================================================================
// Slider
//==============================================================================
export namespace Slider {
    export interface Props extends SliderProps, VariantProps<typeof sliderStyles> {
        ref?: RefObject<HTMLDivElement>
        // TODO: Fix this, variant is a string so as it is isDisabled does nothing.
        //       Need to fix useStyles to check for true or false and make them the
        //       string variant
        isDisabled?: boolean
    }
}

export const Slider = ({ children, ...props }: Slider.Props) => {
    const cachedStyles = useStyles(sliderStyles, props)
    const { styles } = cachedStyles

    return (
        <SliderStyleContext value={cachedStyles}>
            <RacSlider {...mergeProps(props, styles.slider())}>
                <SliderTrack {...styles.trackWrapper()}>
                    <div {...styles.track()}>
                        <SliderThumb />
                    </div>
                </SliderTrack>
            </RacSlider>
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
    export interface Props extends SliderTrackProps {
        // trackProps: ReturnType<typeof useSlider>["trackProps"]
        // noTrack?: boolean | undefined
        // trackGradient?: string[] | undefined
    }
}

export const SliderTrack = (props: SliderTrack.Props) => {
    const { styles } = use(SliderStyleContext)

    return (
        <RacSliderTrack
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
    export interface Props extends SliderThumbProps {}
}

export const SliderThumb = (props: SliderThumb.Props) => {
    const { styles } = use(SliderStyleContext)

    return <RacSliderThumb {...mergeProps(props, styles.thumb())} />
}
