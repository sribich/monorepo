import { createContext, useMemo } from "react"
import { knownWords } from "../generated/rpc-client/analyze_KnownWords"

export const KnownWords = createContext<[number, Map<string, Map<string, boolean>>]>({})

export const useKnownWords = () => {
    const { data, isLoading, error } = knownWords(["known_words"], {})

    const words = useMemo(() => {
        if (!data?.words) {
            return []
        }

        return data.words.reduce((prev, curr) => {
            prev[curr.word] ??= {}
            prev[curr.word][curr.reading] = true

            if (!(curr.reading in prev)) {
                prev[curr.reading] ??= {}
                prev[curr.reading][curr.reading] = true
            }

            return prev
        }, {})
    }, [data])

    return {
        words,
        count: data?.words.length ?? 0,
        isLoading,
        error,
    }
}
