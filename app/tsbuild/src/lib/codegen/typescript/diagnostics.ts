import type { Diagnostic } from "typescript"

// Typescript diagnostic message for 6193 & 6194: Found {0} error{s}. Watching for file changes.
// https://github.com/microsoft/TypeScript/blob/d45012c5e2ab122919ee4777a7887307c5f4a1e0/src/compiler/diagnosticMessages.json#L4759-L4766
export const isTypescriptErrorCode = (code: number): boolean => [6193, 6194].includes(code)

export const getErrorCountFromDiagnostic = (diagnostic: Diagnostic): number => {
    if (typeof diagnostic.messageText !== "string") {
        return 1
    }

    return Number.parseInt(/Found (\d+) error/.exec(diagnostic.messageText)?.[1] || "0")
}
