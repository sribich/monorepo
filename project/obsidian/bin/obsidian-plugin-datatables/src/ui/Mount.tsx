import { createContext, createElement, type FunctionComponent } from "react"

import { ErrorBoundary, ErrorRoot } from "./ErrorRoot"

export interface MountProps<
    TProps extends Record<string, unknown>,
    TContext extends Record<string, unknown>,
> {
    component: FunctionComponent<TProps>
    componentProps: TProps
    context: TContext
}

export interface MountErrorProps {
    error: Error
}

export const Mount = <
    TProps extends Record<string, unknown>,
    TContext extends Record<string, unknown>,
>(
    props: MountProps<TProps, TContext>,
) => {
    const { component, componentProps, context } = props

    return (
        <MountContext.Provider value={context}>
            <ErrorBoundary>{createElement(component, componentProps)}</ErrorBoundary>
        </MountContext.Provider>
    )
}

export const MountError = (props: MountErrorProps) => {
    return <ErrorRoot error={props.error} />
}

export const MountContext = createContext<unknown>({})
