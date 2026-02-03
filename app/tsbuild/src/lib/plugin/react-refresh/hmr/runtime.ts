// @ts-nocheck

type ModuleNamespace = Record<string, any> & {
    [Symbol.toStringTag]: "Module"
}

interface ImportMetaHot {
    accept(cb: (mod: ModuleNamespace) => void): void
}

interface ImportMeta {
    hot: ImportMetaHot | undefined
}

if (!window.__hmr__) {
    window.__hmr__ = {
        contexts: {},
    }

    const socketURL = new URL("/__hmr__", window.location.href.replace(/^http(s)?:/, "ws$1:"))
    socketURL.port = 9094

    const socket = (window.__hmr__.socket = new WebSocket(socketURL.href))
    socket.addEventListener("message", async (event) => {
        const payload = JSON.parse(event.data)
        console.log(event)
        switch (payload?.type) {
            case "reload":
                window.location.reload()
                break
            case "hmr":
                if (!payload.updates?.length) return

                let anyAccepted = false

                const date = Date.now()
                for (const update of payload.updates) {
                    console.log(update)
                    if (window.__hmr__.contexts[update.id]) {
                        const mod = await import(update.url + "?t=" + date)
                        console.log(mod)

                        const accepted = window.__hmr__.contexts[update.id].emit(mod)

                        if (accepted) {
                            console.log("[HMR] Updated accepted by", update.id)
                            anyAccepted = true
                        }
                    }
                }

                if (!anyAccepted) {
                    console.log("[HMR] Updated rejected, reloading...")
                    window.location.reload()
                }
                break
        }
    })
}

export function createHotContext(id: string): ImportMetaHot {
    let callback: undefined | ((mod: ModuleNamespace) => void)
    let disposed = false

    const hot = {
        accept: (cb) => {
            if (disposed) {
                throw new Error("import.meta.hot.accept() called after dispose()")
            }
            if (callback) {
                throw new Error("import.meta.hot.accept() already called")
            }
            callback = cb
        },
        dispose: () => {
            disposed = true
            callback = undefined
        },
        emit(self: ModuleNamespace) {
            if (disposed) {
                throw new Error("import.meta.hot.emit() called after dispose()")
            }
            // return true

            if (callback) {
                console.log("in emit?")
                callback(self)
                return true
            }
            return false
        },
    }

    if (window.__hmr__.contexts[id]) {
        console.log("here i guess?")
        window.__hmr__.contexts[id].dispose()
        window.__hmr__.contexts[id] = undefined
    }
    window.__hmr__.contexts[id] = hot

    return hot
}

declare global {
    interface Window {
        __hmr__: any
    }
}
