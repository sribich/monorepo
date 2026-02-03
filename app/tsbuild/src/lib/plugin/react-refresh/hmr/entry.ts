/*
import RefreshRuntime from "react-refresh/runtime"

declare global {
    interface Window {
        // @ts-ignore
        $RefreshReg$: any
        // @ts-ignore
        $RefreshSig$: any

        $RefreshRuntime$: typeof RefreshRuntime
    }
}

// @ts-ignore
var prevRefreshReg = window.$RefreshReg$
// @ts-ignore
var prevRefreshSig = window.$RefreshSig$

// @ts-ignore
window.$RefreshReg$ = (type, id) => {
    const fullId = id
    RefreshRuntime.register(type, fullId)
}

window.$RefreshReg$ = prevRefreshReg
window.$RefreshSig$ = prevRefreshSig
window.$RefreshSig$ = RefreshRuntime.createSignatureFunctionForTransform

window.$RefreshRuntime$ = RefreshRuntime
window.$RefreshRuntime$.injectIntoGlobalHook(window)

window.$RefreshReg$ = () => {}
// @ts-ignore
window.$RefreshSig$ = () => (type) => type
*/
