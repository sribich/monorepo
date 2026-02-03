import assert from "node:assert"
import { basename } from "node:path"
import { type DefaultTreeAdapterMap, parse, serialize, type Token } from "parse5"
import type { OutputChunk } from "rolldown"
import type { Plugin } from "../../plugin/plugin.js"

const getInlineAsset = (id: string) => {
    ;/[?&]inline-asset/
}

const isInlineAsset = (id: string): boolean => {
    return /[?&]inline-asset/.test(id)
}

export const htmlPlugin: Plugin = async (context) => {
    const inlineAssetCache = new Map<string, string>()
    const documentCache = new Map<string, Document>()

    return {
        rolldown: {
            name: "test",
            resolveId(id) {
                if (isInlineAsset(id)) {
                    return id
                }

                return null
            },
            load(id) {},

            transform(code, id, meta) {
                if (!id.endsWith(".html")) {
                    return
                }

                let js = ""

                const document = parse(code, {
                    sourceCodeLocationInfo: true,
                })

                traverseNode(document, (node) => {
                    if (node.nodeName === "script") {
                        const scriptMetadata = parseScriptElement(node)

                        if (scriptMetadata) {
                            js += `\nimport "${scriptMetadata.src}"`
                        }
                    }
                })

                documentCache.set(id, document)

                return {
                    code: `${js}`,
                    moduleSideEffects: "no-treeshake",
                }
            },
            async generateBundle(outputOptions, bundle, isWrite) {
                const bundleValues = Object.values(bundle)

                for (const [id, document] of documentCache) {
                    const entry = bundleValues.find(
                        (entry) =>
                            entry.type === "chunk" && entry.isEntry && entry.facadeModuleId === id,
                    ) as OutputChunk | undefined

                    const cssAssets = bundleValues.filter((entry) =>
                        entry.fileName.endsWith(".css"),
                    )

                    if (entry) {
                        addToBody(
                            document,
                            createScriptElement({
                                path: `/${entry.fileName}`,
                            }),
                        )

                        for (const css of cssAssets) {
                            addToHead(
                                document,
                                createLinkElement({
                                    path: `/${css.fileName}`,
                                }),
                            )
                        }

                        await context.plugin.transformIndexHtml(document)

                        this.emitFile({
                            type: "asset",
                            originalFileName: id,
                            fileName: basename(id),
                            source: serialize(document),
                        })
                    }
                }
            },
        },
    }
}

type Attribute = Token.Attribute
export type Document = DefaultTreeAdapterMap["document"]
type Element = DefaultTreeAdapterMap["element"]
type Node = DefaultTreeAdapterMap["node"]
type TextNode = DefaultTreeAdapterMap["textNode"]
type ParentNode = DefaultTreeAdapterMap["parentNode"]
type ChildNode = DefaultTreeAdapterMap["childNode"]

const traverseNode = (node: Node, visitor: (node: Node) => void) => {
    visitor(node)

    if (isElement(node) || node.nodeName === "#document") {
        if (!node.childNodes) {
            return
        }

        for (const childNode of node.childNodes) {
            traverseNode(childNode, visitor)
        }
    }
}

const getChildElement = (parentNode: ParentNode, tagName: string): Element | undefined => {
    const node = parentNode.childNodes.find((node) => isElement(node) && node.tagName === tagName)

    return (node as Element) || undefined
}

const isElement = (node?: Node): node is Element => {
    return !!node && node.nodeName !== "#comment" && node.nodeName !== "#text"
}

type CreateElementOptions = {
    // parentNode: ParentNode
    path: string
    children?: Array<ChildNode>

    // basedir: string;
    // crossorigin: Crossorigin | undefined;
    // defer: boolean | undefined;
    // integrity: HashAlgorithm | undefined;
    // outputPath: string;
    // publicPath: string | undefined;
    // useModuleType: boolean | undefined;
}

const createElement = (
    // parentNode: ParentNode,
    tagName: string,
    attrs: Attribute[] = [],
    children: ChildNode[] = [],
): Element => {
    return {
        attrs,
        childNodes: children,
        namespaceURI: "http://www.w3.org/1999/xhtml" as Element["namespaceURI"],
        nodeName: tagName,
        parentNode: null,
        tagName,
    }
}

export const createTextNode = (parentNode: ParentNode, value: string): TextNode => {
    return {
        nodeName: "#text",
        parentNode: parentNode,
        value,
    }
}

export const createScriptElement = (options: CreateElementOptions): Element => {
    const attrs: Attribute[] = [
        { name: "src", value: options.path },
        { name: "type", value: "module" },
    ].filter((it) => !!it.value)

    return createElement(/*options.parentNode,*/ "script", attrs, options.children)
}

export const createLinkElement = (options: CreateElementOptions): Element => {
    const attrs: Attribute[] = [
        { name: "href", value: options.path },
        { name: "rel", value: "stylesheet" },
    ].filter((it) => !!it)

    return createElement(/*options.parentNode, */ "link", attrs)
}

export const addToHead = (document: Document, element: Element): void => {
    const html = getChildElement(document, "html")
    assert(html, "Missing <html> element")

    const head = getChildElement(html, "head")
    assert(head, "Missing <head> element")

    head.childNodes.push(element)
}

export const addToBody = (document: Document, element: Element): void => {
    const html = getChildElement(document, "html")
    assert(html, "Missing <html> element")

    const body = getChildElement(html, "body")
    assert(body, "Missing <body> element")

    body.childNodes.push(element)
}

interface ScriptElement {
    src: string | undefined
    isModule: boolean
    isAsync: boolean
}

const parseScriptElement = (element: Element): ScriptElement | null => {
    let src: string | undefined
    let isModule = false
    let isAsync = false

    for (const attribute of element.attrs) {
        if (attribute.prefix) {
            continue
        }

        switch (true) {
            case attribute.name === "src": {
                src = attribute.value
                break
            }
            case attribute.name === "type" && attribute.value === "module": {
                isModule = true
                break
            }
            case attribute.name === "async": {
                isAsync = true
                break
            }
            case attribute.name === "bundler-ignore": {
                element.attrs = element.attrs.filter(
                    (attribute) => attribute.name !== "bundler-ignore",
                )

                return null
            }
        }
    }

    return {
        src,
        isModule,
        isAsync,
    }
}
