import type { RefObject } from "react"
import { createNewGenericContext } from "@sribich/fude"

import type { AudioPlayer } from "../../../components/AudioPlayer/AudioPlayer"

export const AudioRefContext =
    createNewGenericContext<RefObject<AudioPlayer.ImperativeHandle | null>>()
