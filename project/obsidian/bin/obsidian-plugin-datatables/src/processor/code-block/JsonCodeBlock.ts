import type { Immutable } from "@sribich/ts-utils"
import type { Type } from "arktype"

import { objectToPrettyJson } from "../../util/json"
import type { CodeBlockContext } from "../render"

export interface JsonCodeBlock<TType> {
    data: Immutable<TType>
    update: (receiver: (incoming: TType) => Partial<TType>) => Promise<void>
}

/**
 * TODO: This is buggy as fuck
 */
export const jsonCodeBlock = async <TType extends Record<string, unknown>>(
    codeBlock: CodeBlockContext,
    type: Type<TType>,
    notifier?: (data: TType) => void,
) => {
    const content = codeBlock.readContent()

    if (!content) {
        // TODO: Figure out why this is throwing rogue {}s in our code.
        // await codeBlock.writeContent("{}")
        throw new Error(`Codeblock must contain {}`)
        // content = codeBlock.readContent()
    }

    const { problems, data } = type(JSON.parse(content))

    if (problems) {
        throw new Error(problems.summary)
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
