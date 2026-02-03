import { join } from "node:path"
import { parse, type Token, type DefaultTreeAdapterMap } from "parse5"
import { Context } from "./context.js"
import assert from "node:assert"
import type { RunnerContext } from "./runner.js"
import { existsSync } from "../util/fs.js"
import { logger } from "../logger.js"
import { exit } from "node:process"

// public readonly entrypoints: Set<string>

// entrypoints: Set<string>

export class EntrypointContext extends Context {
    private context: RunnerContext

    constructor(context: RunnerContext) {
        super()

        this.context = context
    }
}

type Attribute = Token.Attribute
type Element = DefaultTreeAdapterMap["element"]
type Node = DefaultTreeAdapterMap["node"]
type ChildNode = DefaultTreeAdapterMap["childNode"]
type ParentNode = DefaultTreeAdapterMap["parentNode"]
