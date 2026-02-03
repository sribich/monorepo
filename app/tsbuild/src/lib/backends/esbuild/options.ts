import type { BuildOptions } from "esbuild"
import { gt } from "semver"
import type { RunnerContext } from "../../context/runner.js"

const UNSET_TYPE_CHECK = Symbol.for("tsbuild")

type TypeCheckObjectFilter<T extends object> = {
    -readonly [K in keyof T as T[K] extends typeof UNSET_TYPE_CHECK ? never : K]: Exclude<
        T[K],
        typeof UNSET_TYPE_CHECK
    >
}

const removeTypeCheckedFields = <T extends object>(obj: T) => {
    return Object.fromEntries(
        Object.entries(obj).filter((entry) => entry[1] !== UNSET_TYPE_CHECK),
    ) as TypeCheckObjectFilter<T>
}

type Require<T> = T extends Record<string, any>
    ? { [K in keyof T]-?: T[K] | typeof UNSET_TYPE_CHECK }
    : never

export const ESM_REQUIRE_SHIM = `
await (async () => {
  const { dirname } = await import("path");
  const { fileURLToPath } = await import("url");

  /**
   * Shim entry-point related paths.
   */
  if (typeof globalThis.__filename === "undefined") {
    globalThis.__filename = fileURLToPath(import.meta.url);
  }
  if (typeof globalThis.__dirname === "undefined") {
    globalThis.__dirname = dirname(globalThis.__filename);
  }
  /**
   * Shim require if needed.
   */
  if (typeof globalThis.require === "undefined") {
    const { default: module } = await import("module");
    globalThis.require = module.createRequire(import.meta.url);
  }
})();
`

export const getDefaultOptions = (
    context: RunnerContext,
    isMaster: boolean,
    format?: string,
): BuildOptions => {
    const config = context.config
    const repository = context.repository
    const plugin = context.plugin
    const buildContext = context.build

    /** @see https://esbuild.github.io/api/# */
    const options = removeTypeCheckedFields({
        //======================================================================
        // General Options
        //======================================================================
        /** @see https://esbuild.github.io/api/#bundle */
        bundle: !!config.bundle,
        /** @see https://esbuild.github.io/api/#platform */
        platform: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#tsconfig */
        tsconfig: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#tsconfig-raw */
        tsconfigRaw: UNSET_TYPE_CHECK,

        //======================================================================
        // Input
        //======================================================================
        /**
         * Handled by the bundler plugin. Because esbuild only transpiles a single
         * file when not bundling, we must gather the entrypoints ourself and
         * generate an entrypoint manifest to transpile an entire build.
         *
         * @see https://esbuild.github.io/api/#entry-points
         */
        entryPoints: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#loader */
        loader: {
            ".node": "copy",
        },
        /** @see https://esbuild.github.io/api/#stdin */
        stdin: UNSET_TYPE_CHECK,

        //======================================================================
        // Output Contents
        //======================================================================
        /** @see https://esbuild.github.io/api/#banner */
        banner: {
            js: format !== "esm" || config.platform === "browser" ? "" : ESM_REQUIRE_SHIM,
        },
        /** @see https://esbuild.github.io/api/#charset */
        charset: config.charset ?? "utf8",
        /** @see https://esbuild.github.io/api/#footer */
        footer: {},
        /**
         * The output format is defined by us when configuring each separate
         * build unit.
         *
         * @see https://esbuild.github.io/api/#format
         */
        format: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#global-name */
        globalName: UNSET_TYPE_CHECK,
        /**
         * TODO: Add an option for this.
         *
         * @see https://esbuild.github.io/api/#legal-comments
         */
        legalComments: "external",
        /** @see https://esbuild.github.io/api/#line-limit */
        lineLimit: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#splitting */
        splitting: false,

        //======================================================================
        // Output Location
        //======================================================================
        /** @see https://esbuild.github.io/api/#allow-overwrite */
        allowOverwrite: false,
        /** @see https://esbuild.github.io/api/#asset-names */
        assetNames: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#chunk-names */
        chunkNames: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#entry-names */
        entryNames: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#out-extension */
        outExtension: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#outbase */
        outbase: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#outdir */
        outdir: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#outfile */
        outfile: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#public-path */
        publicPath: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#write */
        write: true,

        //======================================================================
        // Path Resolution
        //======================================================================
        /** @see https://esbuild.github.io/api/#alias */
        alias: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#conditions */
        conditions: (config.conditions as string[]) ?? [],
        /**
         * This option only works when using `bundle: true`, so we typically
         * instead want to rely on the externals plugin instead.
         *
         * @see https://esbuild.github.io/api/#external
         */
        external: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#main-fields */
        mainFields: (config.mainFields as string[]) ?? UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#node-paths */
        nodePaths: UNSET_TYPE_CHECK,
        /**
         * This can be handled by our externals plugin instead.
         *
         * @see https://esbuild.github.io/api/#packages
         */
        packages: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#preserve-symlinks */
        preserveSymlinks: false,
        /** @see https://esbuild.github.io/api/#resolve-extensions */
        resolveExtensions: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#working-directory */
        absWorkingDir: UNSET_TYPE_CHECK,
        absPaths: UNSET_TYPE_CHECK,
        //======================================================================
        // Transformation
        //======================================================================
        /** @see https://esbuild.github.io/api/#jsx */
        jsx: (() => {
            const react = repository.project.dependencyVersions.get("react")

            if (!react) {
                return "automatic"
            }

            return gt(react, "17.0.0") ? "automatic" : "preserve"
        })(),
        /** @see https://esbuild.github.io/api/#jsx-dev */
        jsxDev: process.env["NODE_ENV"] !== "production" && !config.release,
        /** @see https://esbuild.github.io/api/#jsx-factory */
        jsxFactory: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#jsx-fragment */
        jsxFragment: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#jsx-import-source */
        jsxImportSource: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#jsx-side-effects */
        jsxSideEffects: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#supported */
        supported: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#target */
        target: UNSET_TYPE_CHECK,

        //======================================================================
        // Optimization
        //======================================================================
        /** @see https://esbuild.github.io/api/#define */
        define: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#drop */
        drop: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#drop-labels */
        dropLabels: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#ignore-annotations */
        ignoreAnnotations: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#inject */
        inject: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#keep-names */
        // TODO: config.build.platform === "node" ? true : UNSET_TYPE_CHECK,
        keepNames: !!config.minify,
        /** @see https://esbuild.github.io/api/#mangle-props */
        mangleProps: UNSET_TYPE_CHECK,
        reserveProps: UNSET_TYPE_CHECK,
        mangleQuoted: UNSET_TYPE_CHECK,
        mangleCache: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#minify */
        minify: buildContext.mode === "build" ? !!config.minify : false,
        minifyWhitespace: UNSET_TYPE_CHECK,
        minifyIdentifiers: UNSET_TYPE_CHECK,
        minifySyntax: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#pure */
        pure: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#tree-shaking */
        treeShaking: UNSET_TYPE_CHECK,

        //======================================================================
        // Source Maps
        //======================================================================
        /** @see https://esbuild.github.io/api/#source-root */
        sourceRoot: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#sourcemap */
        sourcemap: config.sourcemap
            ? config.sourcemap
            : process.env["NODE_ENV"] !== "production" && !config.release
              ? "linked"
              : UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#sources-content */
        sourcesContent: UNSET_TYPE_CHECK,

        //======================================================================
        // Build Metadata
        //======================================================================
        /** @see https://esbuild.github.io/api/#metafile */
        metafile: true,

        //======================================================================
        // Logging
        //======================================================================
        /** @see https://esbuild.github.io/api/#color */
        color: true,
        /** @see https://esbuild.github.io/api/#log-level */
        logLevel: "info",
        /** @see https://esbuild.github.io/api/#log-limit */
        logLimit: UNSET_TYPE_CHECK,
        /** @see https://esbuild.github.io/api/#log-override */
        logOverride: UNSET_TYPE_CHECK,

        //======================================================================
        // Other
        //======================================================================
        /** @see https://esbuild.github.io/api/#plugins */
        plugins: plugin.getEsbuildPlugins(isMaster),
    } satisfies Require<BuildOptions>) as BuildOptions

    if (config.platform) {
        options.platform = config.platform
    }

    return options
}
