import React, { type PropsWithChildren } from "react"

// import { Alert } from "antd"

export interface ErrorViewProps {
    error: Error
}

export const ErrorRoot = (props: ErrorViewProps) => {
    const description = props.error.stack
        ?.split("\n")
        .map((it, idx) => (
            <span key={idx}>
                {it.startsWith("    ") ? <span>&emsp;</span> : ""}
                {it}
            </span>
        ))
        .reduce((acc, cur) => acc.concat(cur, <br />), [] as JSX.Element[])

    return <div>{description}</div>

    // return <Alert type="error" message={props.error.message} description={description} banner closable />
}

export class ErrorBoundary extends React.Component<PropsWithChildren, { error: null | Error }> {
    constructor(props: PropsWithChildren) {
        super(props)

        this.state = { error: null }
    }

    static getDerivedStateFromError(error: Error) {
        return { error }
    }

    override componentDidCatch(error: Error, info: React.ErrorInfo) {
        console.error(error, info.componentStack)
    }

    override render() {
        return this.state.error ? <ErrorRoot error={this.state.error} /> : this.props.children
    }
}
