import { useEffect } from "react"

export const useSingleKey = (key: string, onKeyPressed: () => void) => {
    useEffect(() => {
        const handler = (e: KeyboardEvent) => {
            if (e.key === key) {
                e.preventDefault()
                onKeyPressed()
            }
        }

        document.addEventListener("keydown", handler)

        return () => {
            document.removeEventListener("keydown", handler)
        }
    }, [key, onKeyPressed])
}
