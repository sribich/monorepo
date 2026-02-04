import type { Immutable } from "@sribich/ts-utils"

import { ExtendedSet } from "../../util/set"

/**
 * The tag index contains a bi-directional mapping between tags and pages.
 *
 * ! Tags are case-insensitive. Expecting cased tags to be unique will result
 * ! in unexpected behavior.
 */
export class TagIndex {
    private allTags = new ExtendedSet<string>()

    private pageToTags = new Map<string, ExtendedSet<string>>()
    private tagToPages = new Map<string, ExtendedSet<string>>()

    getAllTags(): Immutable<ExtendedSet<string>> {
        return this.allTags
    }

    getTags(page: string): Immutable<ExtendedSet<string>> {
        return this.pageToTags.get(page) ?? new ExtendedSet()
    }

    getPages(tag: string): Immutable<ExtendedSet<string>> {
        return this.tagToPages.get(tag.toLocaleLowerCase()) ?? new ExtendedSet()
    }

    has(tag: string) {
        return this.allTags.has(tag.toLocaleLowerCase())
    }

    set(page: string, tags: ExtendedSet<string>) {
        const mappedTags = new ExtendedSet(tags.map((tag) => tag.toLocaleLowerCase()))

        const currentSet = this.pageToTags.get(page) ?? new Set()

        currentSet.forEach((tag) => {
            if (!mappedTags.has(tag)) {
                this.tagToPages.get(tag)?.delete(page)
            }
        })

        this.pageToTags.set(page, mappedTags)

        mappedTags.forEach((tag) => {
            if (!this.tagToPages.has(tag)) {
                this.tagToPages.set(tag, new ExtendedSet([page]))
            } else {
                this.tagToPages.get(tag)?.add(page)
            }

            this.allTags.add(tag)
        })
    }

    delete(page: string): void {
        const currentTags = this.pageToTags.get(page) ?? new ExtendedSet()

        currentTags.forEach((tag) => {
            this.tagToPages.get(tag)?.delete(page)
        })

        this.pageToTags.delete(page)
    }

    rename(oldPage: string, newPage: string): void {
        const currentTags = this.pageToTags.get(oldPage) ?? new ExtendedSet()

        // This handles removing any reference to the old page.
        this.delete(oldPage)
        this.set(newPage, currentTags)
    }

    clear(): void {
        this.allTags.clear()

        this.pageToTags.clear()
        this.tagToPages.clear()
    }
}
