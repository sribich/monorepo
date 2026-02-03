import { Button, makeStyles, Slider, useStyles } from "@sribich/fude"
import { create } from "@stylexjs/stylex"
import { Pause, Play } from "lucide-react"
import {
    type ComponentProps,
    createContext,
    type Ref,
    type RefObject,
    use,
    useImperativeHandle,
} from "react"
import { type HTMLMediaControls, type HTMLMediaState, useAudio } from "../../hooks/useAudio"

//==============================================================================
//
//==============================================================================
export namespace AudioPlayer {
    export type ImperativeHandle = HTMLMediaControls

    export interface Props extends Omit<ComponentProps<"audio">, "ref"> {
        ref?: Ref<ImperativeHandle>
    }
}

const AudioPlayerContext = createContext<{ state: HTMLMediaState; controls: HTMLMediaControls }>(
    {} as never,
)

export const AudioPlayer = (props: AudioPlayer.Props) => {
    const { children, ref, ...restProps } = props

    const [element, state, controls] = useAudio(restProps)

    useImperativeHandle(ref, () => controls, [controls])

    return (
        <AudioPlayerContext value={{ state, controls }}>
            {element}
            {props.children}
        </AudioPlayerContext>
    )
}

//==============================================================================
//
//==============================================================================
export const AudioPlayerTime = () => {
    const context = use(AudioPlayerContext)

    const { styles } = useStyles(timeStyles, {})

    const test = (value: number) => {
        context.controls.seek(value * (context.state.duration / 100))
    }

    return (
        <div {...styles.wrapper()}>
            <span>{formatTime(context.state.time, context.state.duration)}</span>
            <div {...styles.slider()}>
                <Slider
                    aria-label="duration"
                    value={100 * (context.state.time / context.state.duration) || 0}
                    onChange={test}
                    step={1 / context.state.duration}
                />
            </div>
            <span>{formatTime(context.state.duration, context.state.duration)}</span>
        </div>
    )
}

const formatTime = (seconds: number, duration: number): string => {
    const useHours = duration > 3600

    const h = Math.floor(seconds / 3600)
    const m = Math.floor((seconds % 3600) / 60)
    const s = Math.floor(seconds % 60)

    const hh = h.toString().padStart(2, "0")
    const mm = m.toString().padStart(2, "0")
    const ss = s.toString().padStart(2, "0")

    if (useHours) {
        return `${hh}:${mm}:${ss}`
    }

    return `${mm}:${ss}`
}

const timeStyles = makeStyles({
    slots: create({
        wrapper: {
            display: "flex",
            alignItems: "center",
            flexDirection: "row",
            gap: "16px",
            padding: "4px 0px",
            width: "100%",
        },
        slider: {
            flex: "1 0 auto",
        },
    }),
    variants: {},
    defaultVariants: {},
})

//==============================================================================
// PlayButton
//==============================================================================
export const PlayButton = () => {
    const context = use(AudioPlayerContext)

    const icon = context.state.playing ? <Pause /> : <Play />

    const onPress = () => {
        context.state.playing ? context.controls.pause() : context.controls.play()
    }

    return <Button onPress={onPress}>{icon}</Button>
}
