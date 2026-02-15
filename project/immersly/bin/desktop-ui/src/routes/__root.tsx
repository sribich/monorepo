import { RouterProvider } from "@sribich/fude"
import { lightTheme, darkTheme } from "@sribich/fude-theme"
import { create, props as stylexProps } from "@stylexjs/stylex"
import { Outlet, createRootRoute, useLocation, useNavigate } from "@tanstack/react-router"
import { lazy, use, type PropsWithChildren } from "react"
import { fonts } from "@sribich/fude-theme/vars/fonts.stylex"

import { ThemeContext } from "../context/theme"
import { KnownWords, useKnownWords } from "../context/knownWords"

export const Route = createRootRoute({
    component: () => <RootRoute />,
})

export const RootRoute = () => {
    const { words, count, isLoading, error } = useKnownWords()

    return (
        <KnownWords value={[count, words]}>
            <ThemeWrapper>
                <NavigationWrapper>
                    {/*<TanStackRouterDevtools initialIsOpen={false} position="bottom-right" />*/}
                    <Outlet />
                </NavigationWrapper>
            </ThemeWrapper>
        </KnownWords>
    )
}

const otherThemeStuff = create({
    wrapper: {
        fontFamily: fonts.default
    }
})

const ThemeWrapper = (props: PropsWithChildren) => {
    const theme = use(ThemeContext)

    const themeStyles = {
        light: lightTheme,
        dark: darkTheme,
    }[theme.currentTheme]

    return <div {...stylexProps(themeStyles, otherThemeStuff.wrapper)}>{props.children}</div>
}

const NavigationWrapper = (props: PropsWithChildren) => {
    const navigation = useNavigate()

    return (
        <RouterProvider useLocation={useLocation} navigate={(path) => navigation({ to: path })}>
            {props.children}
        </RouterProvider>
    )
}

const TanStackRouterDevtools =
    process.env.NODE_ENV === "production"
        ? () => null
        : lazy(() =>
              import("@tanstack/router-devtools").then((res) => ({
                  default: res.TanStackRouterDevtools,
              })),
          )
