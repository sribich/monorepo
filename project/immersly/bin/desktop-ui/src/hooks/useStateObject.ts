import { useRef, useState } from "react"

export const useStateObject = <T extends object>(init: T) => {
    const initialState = useRef(init).current
    const [state, setState] = useState<T>(init)

    return {
        ...state,
        set: setState,
        setItem: <K extends keyof T>(key: K, value: T[K]) => {
            setState((prev) => ({ ...prev, [key]: value }))
        },
        merge: (newState: Partial<T>) => {
            setState((prev) => ({ ...prev, ...newState }))
        },
        reset: () => {
            setState({ ...initialState })
        },
    }
}
