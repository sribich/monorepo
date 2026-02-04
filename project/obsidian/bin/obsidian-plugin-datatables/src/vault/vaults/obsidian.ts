import { type App, type CachedMetadata, type Plugin, TAbstractFile, TFile, TFolder } from "obsidian"

import { File, Vault } from "../vault"

export const assertObsidianFile = (file: File): file is ObsidianFile => {
    if (file instanceof ObsidianFile) {
        return true
    }

    throw new Error(`TODO`)
}

export class ObsidianFile extends File {
    constructor(private file: TAbstractFile) {
        super()
    }

    getInner(): TAbstractFile {
        return this.file
    }

    asFile(): TFile {
        if (this.file instanceof TFile) {
            return this.file
        }

        throw new Error(`TODO`)
    }

    asDirectory(): TFolder {
        if (this.file instanceof TFolder) {
            return this.file
        }

        throw new Error(`TODO`)
    }

    override get path(): string {
        return this.file.path
    }

    override isFile(): boolean {
        return this.file instanceof TFile
    }

    override isDirectory(): boolean {
        return this.file instanceof TFolder
    }
}

export class ObsidianVault extends Vault {
    private app: App

    constructor(private plugin: Plugin) {
        super()

        this.app = plugin.app
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Update
    ////////////////////////////////////////////////////////////////////////////
    public override async createEmptyMarkdownFile(folder: File, fileName: string): Promise<File> {
        if (!assertObsidianFile(folder)) {
            throw new Error(`TODO`)
        }

        return new ObsidianFile(
            await this.app.fileManager.createNewMarkdownFile(folder.asDirectory(), fileName),
        )
    }

    public override async create(path: string, content: string): Promise<File> {
        return new ObsidianFile(await this.app.vault.create(path, content))
    }

    public override async getMarkdownFiles(): Promise<ObsidianFile[]> {
        return this.app.vault.getMarkdownFiles().map((file) => new ObsidianFile(file))
    }

    public override async getFile(path: string): Promise<File | null> {
        const file = this.app.vault.getAbstractFileByPath(path)
        console.log(path, file)
        if (!file) {
            return null
        }

        return new ObsidianFile(file)
    }

    public override async read(file: File): Promise<string> {
        if (!assertObsidianFile(file)) {
            throw new Error(`TODO`)
        }

        return this.app.vault.read(file.asFile())
    }

    public override async cachedRead(file: File): Promise<string> {
        if (!assertObsidianFile(file)) {
            throw new Error(`TODO`)
        }

        return this.app.vault.cachedRead(file.asFile())
    }

    public override async cachedMetadata(file: File): Promise<CachedMetadata | null> {
        if (!assertObsidianFile(file)) {
            throw new Error(`TODO`)
        }

        return this.app.metadataCache.getFileCache(file.asFile())
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Update
    ////////////////////////////////////////////////////////////////////////////
    public override async modifyContent(file: File, content: string): Promise<void> {
        if (!assertObsidianFile(file)) {
            throw new Error(`TODO`)
        }

        await app.vault.modify(file.asFile(), content)
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Other
    ////////////////////////////////////////////////////////////////////////////
    public override emitWithoutLock(event: string, content: any): void {
        console.log("Emit", event, content)
        this.plugin.app.workspace.trigger(event, content)
    }

    public override register(callback: () => void): void {
        this.plugin.register(callback)
    }
}
