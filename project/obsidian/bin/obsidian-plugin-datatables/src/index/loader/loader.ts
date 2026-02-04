import { availableParallelism } from "node:os"

import type { Immutable } from "@sribich/ts-utils"

import type { DtSchema } from "../../schema/schema-definition"
import type { Vault } from "../../vault/vault"
import type { Document, DocumentMetadata, ReadDocument } from "../document"
// @ts-expect-error - Worker exports are defined at build time by the bundler
import InlineWorker from "./loader.worker?worker&inline"
import type { WorkerRequest, WorkerResponse } from "./types"

export class Loader {
    private workers: Worker[] = []
    private freeWorkers: number[] = []

    /**
     * A map of file paths to a list of callbacks that are waiting for
     * the file to be processed.
     *
     * If a file is queued but already being processed, the callback will
     * be added to this list.
     */
    private processingDocuments = new Map<
        string,
        [(data: Document) => void, (data: string) => void][]
    >()

    /**
     * A queue of documents that are waiting to be processed.
     */
    private processingQueue: DocumentMetadata[] = []

    constructor(
        private vault: Vault,
        private schema: Immutable<DtSchema>,
    ) {
        // Having too many workers causes indexing to be slower, so let's
        // limit the number of workers to 4.
        const numThreads = Math.min(4, availableParallelism())

        this.workers = Array.from({ length: numThreads }, (_, i) => {
            if (process.env["IN_TEST_RUNNER"]) {
                class _Worker {
                    public onmessage: (props: any) => any = () => {}

                    constructor(private url: string) {}

                    postMessage(event: MessageEvent<any>) {
                        this.onmessage?.(event)
                    }

                    terminate() {}
                }

                window.Worker = _Worker as any // Test only hack
            }

            const worker = new InlineWorker() as Worker
            // const worker = new Worker(new URL("./loader.worker.ts", import.meta.url), {
            //     type: "module",
            // })

            worker.onmessage = (event) => {
                this.freeWorkers.push(i)
                this.handleWorkerMessage(event)
            }

            this.freeWorkers.push(i)

            return worker
        })

        this.vault.register(() => {
            for (const worker of this.workers) {
                worker.terminate()
            }
        })
    }

    /**
     * Loads a document, returning a promise that resolves when the
     * document has been processed.
     */
    public async load(document: DocumentMetadata): Promise<Document> {
        const isProcessing = this.processingDocuments.has(document.path)

        const promise = new Promise<Document>((resolve, reject) => {
            if (!isProcessing) {
                this.processingDocuments.set(document.path, [[resolve, reject]])
            } else {
                this.processingDocuments.get(document.path)?.push([resolve, reject])
            }
        })

        if (!isProcessing) {
            await this.process(document)
        }

        return promise
    }

    /**
     * Processes a file using the next available worker.
     *
     * If a worker is not available, the file will be queued and processed
     * in first-in-first-out order.
     */
    private async process(document: DocumentMetadata): Promise<void> {
        const workerId = this.freeWorkers.pop()

        if (workerId === undefined) {
            this.processingQueue.push(document)
            return
        }

        const readDocument = await this.getDocumentContent(document)

        this.workers[workerId]?.postMessage({
            path: document.path,
            readDocument: readDocument,
            schema: this.schema,
        } satisfies WorkerRequest)
    }

    private async getDocumentContent(document: DocumentMetadata): Promise<ReadDocument> {
        switch (document.kind) {
            case "markdown":
                return {
                    kind: "markdown",
                    content: await this.vault.cachedRead(document.file),
                    metadata: await this.vault.cachedMetadata(document.file),
                }
        }
    }

    private async handleWorkerMessage(event: MessageEvent) {
        const data = event.data as WorkerResponse

        const callbacks = this.processingDocuments.get(data.path) ?? []
        this.processingDocuments.delete(data.path)

        switch (data.kind) {
            case "processing-success":
                for (const [resolve, _] of callbacks) {
                    resolve(data.result)
                }
                break
            case "processing-error":
                for (const [_, reject] of callbacks) {
                    reject(data.result)
                }
                break
            default:
                throw new Error(`Unknown worker response kind: ${data}`)
        }

        const nextDocument = this.processingQueue.shift()

        if (nextDocument) {
            this.process(nextDocument)
        }
    }
}
