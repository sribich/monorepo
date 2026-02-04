import type { CachedMetadata } from "obsidian"

export abstract class File {
    abstract get path(): string

    /**
     * Returns true if the path points to a file on the filesystem.
     */
    abstract isFile(): boolean

    /**
     * Returns true if the path points to a directory on the filesystem.
     */
    abstract isDirectory(): boolean
}

/**
 * The vault abstracts all pieces of Obsidian that we use into
 * an interface that we can override to allow for functional
 * testing.
 */
export abstract class Vault {
    private locked = false
    private lockCount = 0
    private pendingEmits = {} as Record<string, unknown>

    ////////////////////////////////////////////////////////////////////////////
    /// Update
    ////////////////////////////////////////////////////////////////////////////
    public abstract createEmptyMarkdownFile(folder: File, fileName: string): Promise<File>

    public abstract create(path: string, content: string): Promise<File>

    public abstract getMarkdownFiles(): Promise<File[]>

    public abstract getFile(path: string): Promise<File | null>

    public abstract read(file: File): Promise<string>

    public abstract cachedRead(file: File): Promise<string>
    public abstract cachedMetadata(file: File): Promise<CachedMetadata | null>

    ////////////////////////////////////////////////////////////////////////////
    /// Update
    ////////////////////////////////////////////////////////////////////////////
    public abstract modifyContent(file: File, content: string): Promise<void>

    /*
    public abstract getFolder(path: string): void
    public abstract getFile(path: string): void    
    */

    ////////////////////////////////////////////////////////////////////////////
    /// Other
    ////////////////////////////////////////////////////////////////////////////
    public lock(): void {
        this.lockCount = Math.max(0, this.lockCount + 1)
        this.locked = true
    }

    public unlock(): void {
        this.lockCount = Math.max(0, this.lockCount - 1)
        this.locked = this.lockCount !== 0

        if (!this.locked) {
            for (const [event, content] of Object.entries(this.pendingEmits)) {
                this.emitWithoutLock(event, content)
            }
        }
    }

    public async withLock(fn: () => Promise<unknown>): Promise<void> {
        this.lock()
        await fn()
        this.unlock()
    }

    public emit(event: string, content: any): void {
        if (!this.locked) {
            return this.emitWithoutLock(event, content)
        }

        this.pendingEmits[event] = content
    }

    public abstract emitWithoutLock(event: string, content: any): void

    public abstract register(callback: () => void): void
}
