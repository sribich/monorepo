import { type Type, type } from "arktype"

export type AssetInfo = {
    name: string
}

export const outputConfig: Type<OutputPluginConfig> = type({
    "assetFileNames?": type(["Function", "|", "undefined"]) as Type<
        ((assetInfo: AssetInfo) => string) | undefined
    >,
})

export interface OutputPluginConfig {
    assetFileNames?: ((assetInfo: AssetInfo) => string) | undefined
}
