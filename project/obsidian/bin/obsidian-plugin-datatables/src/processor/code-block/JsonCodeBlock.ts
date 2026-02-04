import type { Immutable } from "@sribich/ts-utils"
import { ArkErrors, type Type } from "arktype"

import { objectToPrettyJson } from "../../util/json"
import type { CodeBlockContext } from "../render"

export interface JsonCodeBlock<TType> {
    data: Immutable<TType>
    update: (receiver: (incoming: TType) => Partial<TType>) => Promise<void>
}

export const jsonCodeBlock = async <TType extends Record<string, unknown>>(
    codeBlock: CodeBlockContext,
    type: Type<TType>,
    notifier?: (data: TType) => void,
) => {
    const content = codeBlock.readContent()

    if (!content) {
        throw new Error(`Codeblock must contain {}`)
    }

    const data = type(JSON.parse(content))

    if (data instanceof ArkErrors) {
        throw new Error(data.summary)
    }

    // Because we don't fully re-render the component when the underlying
    // code block changes, we need to keep a working copy of the data that
    // we manually keep in sync.
    let workingData = { ...data } as TType

    const update = async (receiver: (incoming: TType) => Partial<TType>) => {
        workingData = {
            ...workingData,
            ...receiver(workingData),
        }

        await codeBlock.writeContent(objectToPrettyJson(workingData))

        notifier?.(workingData)
    }

    notifier?.(workingData)

    return {
        get data() {
            return workingData as Immutable<TType>
        },
        update,
    }
}
