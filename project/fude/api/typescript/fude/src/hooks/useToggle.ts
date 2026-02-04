import { useCallback, useState } from "react"

export const useToggle = (initialValue = false): [boolean, () => void] => {
    const [value, setValue] = useState(initialValue)

    const toggleValue = useCallback(() => {
        setValue((prev) => !prev)
    }, [initialValue])

    return [value, toggleValue]
}
