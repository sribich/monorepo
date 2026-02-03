import ts from "typescript"
import { parentPort, workerData } from "worker_threads"

import { TypescriptBackend } from "../../codegen/typescript/backend.js"

const { config, format, outdir, entrypoints } = workerData

const getOptions = (format: "cjs" | "esm", outDir: string) => {
    return format === "cjs"
        ? {
              module: ts.ModuleKind.CommonJS,
              moduleResolution: ts.ModuleResolutionKind.Node10,
              // composite: false,
              declaration: true,
              noEmit: false,
              emitDeclarationOnly: true,
              verbatimModuleSyntax: false,
              outDir,
          }
        : {
              module: ts.ModuleKind.ES2020,
              moduleResolution: ts.ModuleResolutionKind.Bundler,
              // composite: false,
              declaration: true,
              noEmit: false,
              emitDeclarationOnly: true,
              outDir,
          }
}

const unit = new TypescriptBackend(config, {
    options: {
        ...getOptions(format, outdir),
    },
    fileNames: Array.from(entrypoints),
})

unit.initialise().then(() => {
    parentPort?.postMessage("ready")
})

parentPort?.on("message", (message) => {
    switch (message) {
        case "exit":
            unit.terminate().finally(() => {
                parentPort?.close()
            })
            return
        case "emit":
            try {
                unit.compile().finally(() => {
                    parentPort?.postMessage("emitComplete")
                })
            } catch (e) {
                console.log("error happened?", e)
            }
    }
})
