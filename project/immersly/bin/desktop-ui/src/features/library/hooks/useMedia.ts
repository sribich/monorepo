import { useMemo } from "react"
import { getSeries, type MediaView } from "../../../generated/rpc-client/library_GetSeries"
import { getMedia } from "../../../generated/rpc-client/library_GetMedia"

export const useMedia = (mediaId: string) => {
    const result = getMedia(["get_media", mediaId], { id: mediaId })

    console.log(result.data)

    const media = useMemo(() => {
        const baseResult = result.data ?? []

        return baseResult.reduce(
            (acc, curr) => {
                acc[curr.kind] ??= []
                acc[curr.kind]?.push(curr)

                return acc
            },
            {} as Record<string, MediaView[]>,
        )
    }, [result.data])

    return {
        media,
        isLoading: result.isLoading,
        error: result.error,
    }
}
