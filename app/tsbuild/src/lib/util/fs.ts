import { access, accessSync } from "node:fs"

export const exists = (path: string): Promise<boolean> => {
    return new Promise((resolve) => {
        access(path, (e) => resolve(!e))
    })
}

export const existsSync = (path: string): boolean => {
    try {
        accessSync(path)
    } catch (e) {
        return false
    }

    return true
}
