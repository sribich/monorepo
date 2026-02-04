import { useCallback, useState } from "react"

export const usePersistentToggle = (key: string, initialValue = false): [boolean, () => void] => {
    const [value, setValue] = useState(() => {
        const value = window.localStorage.getItem(key)

        return value === null ? initialValue : value === "true"
    })

    const toggleValue = useCallback(() => {
        setValue((prev) => {
            window.localStorage.setItem(key, String(!prev))

            return !prev
        })
    }, [])

    return [value, toggleValue]
}
