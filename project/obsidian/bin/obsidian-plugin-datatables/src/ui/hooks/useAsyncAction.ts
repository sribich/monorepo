import { useState } from "react"

export const useAsyncAction = <T extends Function>(action: T): [boolean, T] => {
    const [isRunning, setRunning] = useState(false)

    const runAction = async (...args: any[]) => {
        if (!isRunning) {
            setRunning(true)
            await action(...args)
            setRunning(false)
        }
    }

    return [isRunning, runAction as never as T]
}
