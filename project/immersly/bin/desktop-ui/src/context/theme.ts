import { createContext } from "react"

export interface ThemeContext {
    currentTheme: "light" | "dark"
    setTheme: (theme: "light" | "dark") => void
}

export const ThemeContext = createContext<ThemeContext>({
    currentTheme: "light",
    setTheme: () => void 0,
})
