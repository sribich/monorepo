import { SidebarProvider } from "@sribich/fude"
import { QueryClientProvider } from "@tanstack/react-query"
import { createBrowserHistory, createRouter, RouterProvider } from "@tanstack/react-router"
import { useEffect, useState } from "react"
import { ThemeContext } from "./context/theme"
import { ApiHostContext, useApiPort } from "./hooks/useApiPort"
// import { routeTree } from "./generated/routeTree"
import { routeTree } from "./router"
import { queryClient } from "./query-client"

const router = createRouter({
    routeTree,
    history: createBrowserHistory(),
})

export const App = () => {
    const [isDark, setDark] = useState(false)

    const { host, port, isLoading } = useApiPort()

    useEffect(() => {
        window.fetch__monkeyPatch ??= window.fetch

        window.fetch = (...args) => {
            if (typeof args[0] === "string") {
                args[0] = new URL(args[0], `http://127.0.0.1:${port}`)
            }

            return window.fetch__monkeyPatch(...args)
        }
    }, [port])

    if (isLoading) {
        return null
    }

    return (
        <ThemeContext
            value={{
                currentTheme: isDark ? "dark" : "light",
                setTheme: (theme) => setDark(theme === "dark"),
            }}
        >
            <QueryClientProvider client={queryClient}>
                <SidebarProvider>
                    <ApiHostContext value={{ host }}>
                        <RouterProvider router={router} />
                    </ApiHostContext>
                </SidebarProvider>
            </QueryClientProvider>
        </ThemeContext>
    )
}
