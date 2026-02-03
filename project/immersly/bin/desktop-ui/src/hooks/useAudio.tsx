import {
    createElement,
    useEffect,
    useRef,
    type AudioHTMLAttributes,
    type ComponentRef,
    type ReactEventHandler,
    type RefObject,
    type SyntheticEvent,
} from "react"
import { useStateObject } from "./useStateObject"

export const parseTimeRanges = (ranges: TimeRanges) => {
    const result: { start: number; end: number }[] = []

    for (let i = 0; i < ranges.length; i++) {
        result.push({
            start: ranges.start(i),
            end: ranges.end(i),
        })
    }

    return result
}

export interface HTMLAudioProps extends AudioHTMLAttributes<HTMLAudioElement> {
    ref?: RefObject<HTMLAudioElement>
    src: string
}

export interface HTMLMediaState {
    buffered: any[]
    duration: number
    paused: boolean
    muted: boolean
    time: number
    volume: number
    playing: boolean
}

export interface HTMLMediaControls {
    play: () => Promise<void> | void
    pause: () => void
    mute: () => void
    unmute: () => void
    volume: (volume: number) => void
    seek: (time: number) => void

    getTime: () => number
}

export const useAudio = (props: HTMLAudioProps) => {
    const state = useStateObject<HTMLMediaState>({
        buffered: [],
        time: 0,
        duration: 0,
        paused: true,
        muted: false,
        volume: 1,
        playing: false,
    })
    const ref = useRef<ComponentRef<"audio" | "video">>(null)

    const wrapEvent = (proxyEvent: ReactEventHandler, userEvent?: ReactEventHandler) => {
        return (event: SyntheticEvent<HTMLAudioElement | HTMLVideoElement>) => {
            proxyEvent(event)
            userEvent?.(event)
        }
    }

    const onPlay = () => state.merge({ paused: false })
    const onPlaying = () => state.merge({ playing: true })
    const onWaiting = () => state.merge({ playing: false })
    const onPause = () => state.merge({ paused: true, playing: false })
    const onVolumeChange = () => {
        if (ref.current) {
            state.merge({
                muted: ref.current.muted,
                volume: ref.current.volume,
            })
        }
    }
    const onDurationChange = () => {
        if (ref.current) {
            state.merge({
                duration: ref.current.duration,
                buffered: parseTimeRanges(ref.current.buffered),
            })
        }
    }
    const onTimeUpdate = () => {
        if (ref.current) {
            state.merge({ time: ref.current.currentTime })
        }
    }
    const onProgress = () => {
        if (ref.current) {
            state.merge({ buffered: parseTimeRanges(ref.current.buffered) })
        }
    }

    const element = createElement("audio", {
        controls: false,
        ...props,
        ref,
        onPlay: wrapEvent(onPlay, props.onPlay),
        onPlaying: wrapEvent(onPlaying, props.onPlaying),
        onWaiting: wrapEvent(onWaiting, props.onWaiting),
        onPause: wrapEvent(onPause, props.onPause),
        onVolumeChange: wrapEvent(onVolumeChange, props.onVolumeChange),
        onDurationChange: wrapEvent(onDurationChange, props.onDurationChange),
        onTimeUpdate: wrapEvent(onTimeUpdate, props.onTimeUpdate),
        onProgress: wrapEvent(onProgress, props.onProgress),
    })

    const controls = {
        play: () => {
            if (ref.current) {
                return ref.current.play()
            }
        },
        pause: () => {
            if (ref.current) {
                return ref.current.pause()
            }
        },
        isPlaying: () => {
            return (
                ref.current &&
                ref.current.currentTime > 0 &&
                !ref.current.paused &&
                !ref.current.ended
            )
        },
        toggle: () => {
            if (controls.isPlaying()) {
                controls.pause()
            } else {
                controls.play()
            }
        },
        seek: (time: number) => {
            if (ref.current) {
                ref.current.currentTime = Math.min(ref.current.duration, Math.max(0, time))
            }
        },
        volume: (volume: number) => {
            if (ref.current) {
                const clippedVolume = Math.min(1, Math.max(0, volume))
                ref.current.volume = clippedVolume
                state.merge({ volume: clippedVolume })
            }
        },
        mute: () => {
            if (ref.current) {
                ref.current.muted = true
            }
        },
        unmute: () => {
            if (ref.current) {
                ref.current.muted = false
            }
        },
        getTime: () => {
            return ref.current?.currentTime ?? 0
        },
    }

    useEffect(() => {
        if (!ref.current) {
            if (process.env.NODE_ENV !== "production") {
                console.error(
                    "useAudio() ref to <audio> element is empty at mount. " +
                        "It seem you have not rendered the audio element, which it " +
                        "returns as the first argument const [audio] = useAudio(...).",
                )
            }
            return
        }

        state.merge({
            volume: ref.current.volume,
            muted: ref.current.muted,
            paused: ref.current.paused,
        })

        // Start media, if autoPlay requested.
        if (props.autoPlay && ref.current.paused) {
            controls.play()
        }
    }, [props.src])

    return [element, state, controls, ref] as const
}
