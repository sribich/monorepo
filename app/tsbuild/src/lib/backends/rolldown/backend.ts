import { type InputOptions, type OutputOptions, type RolldownBuild, rolldown } from "rolldown"
import { RunnerMode } from "../../runner.js"
import {
    type DeepRequire,
    UNSET_TYPE_CHECK,
    removeTypeCheckedFields,
} from "../../util/typecheck.js"
import { Backend } from "../backend.js"

export class RolldownBackend extends Backend {
    private build: RolldownBuild | undefined = undefined

    override async initialise(): Promise<void> {
        this.build = await rolldown({
            ...(await this.getConfig()),
            ...(this.context.config.rolldown?.input ?? {}),
        })
    }

    override async terminate(): Promise<void> {
        await this.build?.close()
    }

    override async compile(): Promise<void> {
        const options = await this.getOutputOptions()

        await this.build?.generate(options)

        if (this.context.build.mode === RunnerMode.BUILD) {
            await this.build?.write(options)
        }
    }

    async getConfig(): Promise<InputOptions> {
        const config = this.context.config

        return removeTypeCheckedFields({
            input: [...config.entrypoints],
            plugins: [...this.context.plugin.getRolldownPlugins()],
            // external: (id: string) => id.indexOf("node_modules") >= 0,
            // external: config.build.externals
            //     ? (id: string) => config.build.externals!.some((it) => id.indexOf(it) >= 0)
            //     : UNSET_TYPE_CHECK,
            external: UNSET_TYPE_CHECK,
            resolve: {
                // TODO: Fix this
                conditionNames: ["development"],
            } as any,
            cwd: UNSET_TYPE_CHECK,
            platform: UNSET_TYPE_CHECK,
            shimMissingExports: false, // UNSET_TYPE_CHECK,
            preserveEntrySignatures: UNSET_TYPE_CHECK,
            treeshake: false /*{
                moduleSideEffects: false,
                annotations: false,
                manualPureFunctions: [],
                unknownGlobalSideEffects: false,
            }, // UNSET_TYPE_CHECK,
            */,
            logLevel: UNSET_TYPE_CHECK,
            onLog: UNSET_TYPE_CHECK,
            onwarn: (warning: any, handler: any) => {
                if (
                    warning.code === "CIRCULAR_DEPENDENCY" &&
                    warning.message.includes("node_modules")
                ) {
                    return
                }

                handler(warning)
            },
            moduleTypes: UNSET_TYPE_CHECK,
            experimental: {
                resolveNewUrlToAsset: false,
                devMode: false, // devMode starts a hmr server
            },
            define: UNSET_TYPE_CHECK,
            inject: UNSET_TYPE_CHECK,
            profilerNames: UNSET_TYPE_CHECK,
            jsx: UNSET_TYPE_CHECK,
            // transform: UNSET_TYPE_CHECK,
            transform: {
                // target: "",
                dropLabels: [],
                jsx: {
                    // runtime: UNSET_TYPE_CHECK,
                    // development: UNSET_TYPE_CHECK,
                    // throwIfNamespace: UNSET_TYPE_CHECK,
                    // pure: UNSET_TYPE_CHECK,
                    // importSource: UNSET_TYPE_CHECK,
                    // pragma: UNSET_TYPE_CHECK,
                    // pragmaFrag: UNSET_TYPE_CHECK,
                    // useBuiltIns: UNSET_TYPE_CHECK,
                    // useSpread: UNSET_TYPE_CHECK,
                    // refresh: true,
                },
                // assumptions: UNSET_TYPE_CHECK,
                // typescript: UNSET_TYPE_CHECK,
                // target: UNSET_TYPE_CHECK,
                // helpers: UNSET_TYPE_CHECK,
                // decorator: UNSET_TYPE_CHECK,
            },
            optimization: {
                inlineConst: false,
            },
            watch: UNSET_TYPE_CHECK,
            checks: {
                circularDependency: true,
                eval: true,
                missingGlobalName: true,
                missingNameOptionForIifeExport: true,
                mixedExports: true,
                unresolvedEntry: true,
                unresolvedImport: true,
                filenameConflict: true,
                commonJsVariableInEsm: true,
                importIsUndefined: true,
                configurationFieldConflict: true,
            } as any,

            /**
             * TODO: This is almost always certainly a code smell. We should instead
             *       error on absolute imports.
             */
            makeAbsoluteExternalsRelative: true,

            //
            debug: UNSET_TYPE_CHECK,
        } as any satisfies DeepRequire<InputOptions>)
    }

    async getOutputOptions(): Promise<OutputOptions> {
        return removeTypeCheckedFields({
            dir: "dist",
            file: UNSET_TYPE_CHECK,
            exports: UNSET_TYPE_CHECK,
            hashCharacters: UNSET_TYPE_CHECK,
            format: "esm",
            sourcemap: UNSET_TYPE_CHECK,
            sourcemapBaseUrl: UNSET_TYPE_CHECK,
            sourcemapDebugIds: UNSET_TYPE_CHECK,
            sourcemapIgnoreList: UNSET_TYPE_CHECK,
            sourcemapPathTransform: UNSET_TYPE_CHECK,
            postBanner: UNSET_TYPE_CHECK,
            postFooter: UNSET_TYPE_CHECK,

            footer: UNSET_TYPE_CHECK,
            intro: UNSET_TYPE_CHECK,
            outro: UNSET_TYPE_CHECK,
            extend: UNSET_TYPE_CHECK,
            esModule: UNSET_TYPE_CHECK,

            entryFileNames: "[name].js", // UNSET_TYPE_CHECK, // "[name]-other.js",
            chunkFileNames: "[name].js", // UNSET_TYPE_CHECK, // "[name]-test.js",
            cssEntryFileNames: "[name].css", // UNSET_TYPE_CHECK,
            cssChunkFileNames: "[name].css", // UNSET_TYPE_CHECK,
            sanitizeFileName: UNSET_TYPE_CHECK,
            minify: UNSET_TYPE_CHECK,
            name: UNSET_TYPE_CHECK,
            globals: UNSET_TYPE_CHECK,
            externalLiveBindings: false, // UNSET_TYPE_CHECK,
            codeSplitting: false,
            legalComments: UNSET_TYPE_CHECK,
            plugins: [],
            polyfillRequire: UNSET_TYPE_CHECK,
            paths: UNSET_TYPE_CHECK,
            generatedCode: UNSET_TYPE_CHECK,
            dynamicImportInCjs: UNSET_TYPE_CHECK,
            hoistTransitiveImports: UNSET_TYPE_CHECK,
            preserveModules: UNSET_TYPE_CHECK,
            virtualDirname: UNSET_TYPE_CHECK,
            preserveModulesRoot: UNSET_TYPE_CHECK,
            topLevelVar: UNSET_TYPE_CHECK,

            cleanDir: UNSET_TYPE_CHECK,

            // minify
            minifyInternalExports: UNSET_TYPE_CHECK,
            /**
             * TODO: We should have a check here for whether or not there are any decorators, or
             *       detect nest (?) or non web apps. This is not ideal for frontend code.
             */
            keepNames: true,

            // Unused
            assetFileNames: UNSET_TYPE_CHECK,
            banner: UNSET_TYPE_CHECK,

            strictExecutionOrder: UNSET_TYPE_CHECK,

            // Deprecated
            advancedChunks: UNSET_TYPE_CHECK,

            inlineDynamicImports: UNSET_TYPE_CHECK,

            manualChunks: UNSET_TYPE_CHECK,
        } satisfies DeepRequire<OutputOptions>)
    }
}
