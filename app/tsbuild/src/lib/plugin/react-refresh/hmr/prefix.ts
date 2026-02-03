// @ts-expect-error -- This is part of the HMR plugin
import { createHotContext as $$createHotContext } from "virtual:hmr-runtime"

type ModuleNamespace = Record<string, unknown> & {
    [Symbol.toStringTag]: "Module"
}

interface ImportMetaHot {
    accept(cb: (mod: ModuleNamespace) => void): void
}

declare global {
    interface ImportMeta {
        hot: ImportMetaHot | undefined
    }
}

if (import.meta) {
    import.meta.hot = $$createHotContext(
        //@ts-expect-error
        $id$,
    )
}
