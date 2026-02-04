import { scope, type Type, type } from "arktype"
import type { Immutable } from "../../util/immutable.js"

export interface AssetConfig {
    /**
     * Whether or not the plugin should be enabled.
     *
     * @default true
     */
    enabled?: boolean
    /**
     * The list of assets to copy.
     *
     * When given a string, it will be normalized into a
     * CopyableAsset. If the string is a file, the file will
     * be copied to the `toDirectory`. If the string is a
     * directory, the folder itself will be copied.
     *
     * For example, if the string was "foo/bar/dir", the folder
     * "dir" would be copied to the `toDirectory`.
     *
     * @see CopyableAsset
     */
    globs: (string | CopyableAsset)[]
}

export const assetConfig: Type<
    AssetConfig,
    {
        asset: {
            input: string,
            glob?: string,
            output?: string
        },
        definition: {
            enabled?: boolean,
            globs: (string | {
                input: string,
                glob?: string,
                output?: string }
            )[]
        }
    }
> = scope({
    asset: type({
        /**
         * The base input file or directory to copy from.
         *
         * If input is a file, it will be copied to the
         * output directory, erroring if a glob is given.
         */
        input: "string",
        /**
         * When `input` is a directory, this glob
         *
         * * If `input` is not a directory, an error will be thrown.
         */
        "glob?": "string",
        /**
         * The base directory that assets will be copied to.
         */
        "output?": "string",
    }),
    definition: {
        "enabled?": "boolean",
        globs: "(string | asset)[]",
    },
}).export().definition

export type AssetGlob = Immutable<AssetConfig["globs"][number]>
export type Asset = CopyableAsset

export interface AssetPluginOptions {
    /**
     * The base directory assets will be copied from.
     *
     * This must be an absolute path and asset inputs
     * should be defined relative to this directory.
     */
    fromDirectory: string
    /**
     * The base directory assets will be copied to.
     *
     * This must be an absolute path and asset outputs
     * should be defined relative to this directory.
     *
     * Any link that resolves outside of this directory
     * will cause an exception to be thrown.
     */
    toDirectory: string
    /**
     *
     */
    globs: AssetConfig["globs"]
}

export interface CopyableAsset {
    /**
     * The base input file or directory to copy from.
     *
     * If input is a file, it will be copied to the
     * output directory, erroring if a glob is given.
     */
    input: string
    /**
     * When `input` is a directory, this glob
     *
     * * If `input` is not a directory, an error will be thrown.
     */
    glob?: string
    /**
     * The base directory that assets will be copied to.
     */
    output?: string
}

export type PluginOptions = {
    /**
     * The base directory assets will be copied from.
     *
     * This must be an absolute path and asset inputs
     * should be defined relative to this directory.
     */
    fromDirectory: string
    /**
     * The base directory assets will be copied to.
     *
     * This must be an absolute path and asset outputs
     * should be defined relative to this directory.
     *
     * Any link that resolves outside of this directory
     * will cause an exception to be thrown.
     */
    toDirectory: string
    /**
     * Whether to re-copy assets on detected changes.
     *
     * Only changed assets will be copied, compared to
     * a full directory copy.
     *
     * ! Deleted files will NOT be updated.
     */
    watch?: boolean
    /**
     * The list of assets to copy.
     *
     * When given a string, it will be normalized into a
     * CopyableAsset. If the string is a file, the file will
     * be copied to the `toDirectory`. If the string is a
     * directory, the folder itself will be copied.
     *
     * For example, if the string was "foo/bar/dir", the folder
     * "dir" would be copied to the `toDirectory`.
     *
     * @see CopyableAsset
     */
    assets: (string | CopyableAsset)[]
}
