import { documentParsers } from "./document-parsers"
import type { WorkerRequest, WorkerResponse } from "./types"

onmessage = async (event: MessageEvent<WorkerRequest>) => {
    const { path, readDocument, schema } = event.data

    try {
        const result = await documentParsers[readDocument.kind](path, readDocument, schema)

        postMessage({ kind: "processing-success", path, result } satisfies WorkerResponse)
    } catch (error) {
        postMessage({
            kind: "processing-error",
            path,
            result: `Failed to index file: ${path}: ${error}`,
        } satisfies WorkerResponse)
    }
}
