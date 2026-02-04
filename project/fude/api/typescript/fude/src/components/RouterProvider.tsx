import { RouterProvider as AriaRouterProvider } from "@react-aria/utils"
import { createContext } from "react"

export const RouterProviderContext = createContext(() => ({
    pathname: "",
}))

export namespace RouterProvider {
    type Args<T extends (...args: any) => any> = Parameters<T>[0]

    export interface Props extends Args<typeof AriaRouterProvider> {
        useLocation(): {
            pathname: string
        }
    }
}

export const RouterProvider = (props: RouterProvider.Props) => {
    return (
        // @ts-expect-error
        <AriaRouterProvider navigate={props.navigate} useHref={props.useHref}>
            <RouterProviderContext value={props.useLocation}>
                {props.children}
            </RouterProviderContext>
        </AriaRouterProvider>
    )
}
