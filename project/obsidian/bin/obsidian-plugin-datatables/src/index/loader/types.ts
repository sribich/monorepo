import type { Immutable } from "@sribich/ts-utils"

import type { DtSchema } from "../../schema/schema-definition"
import type { Document, ReadDocument } from "../document"

export interface WorkerRequest {
    path: string
    readDocument: ReadDocument
    schema: Immutable<DtSchema>
}

export type WorkerResponse = ProcessedDocument | ProcessingError

export interface BaseResponse<T> {
    kind: T
    path: string
}

export interface ProcessedDocument extends BaseResponse<"processing-success"> {
    result: Document
}

export interface ProcessingError extends BaseResponse<"processing-error"> {
    result: string
}
