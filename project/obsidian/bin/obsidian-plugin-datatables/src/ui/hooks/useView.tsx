import { createGenericContext } from "@sribich/fude"
import pluralize from "pluralize"
import { useMemo } from "react"

import type { DatatableContext } from "../../processor/processors/datatable"
import { useMountContext } from "./useMountContext"

export const [useViewContext, ViewProvider] = createGenericContext<string>()

export const useView = () => {
    const { proxy, proxyMut } = useMountContext<DatatableContext>()

    /*
    if (!proxy.codeBlock.view) {
        throw new Error(`View is not defined.`)
    }
    */

    const source = useMemo(() => {
        return pluralize((proxy.codeBlock.source ?? "").substring(1))
    }, [proxy.codeBlock.source])

    return {
        source,
        view: proxy.codeBlock.view,
        setView: (view: string | number) => {
            if (typeof view === "number") {
                throw new Error(
                    `Tried to set view to incompatible value of type ${typeof view}: ${view}`,
                )
            }

            proxyMut.codeBlock.view = view
        },
    }
}
