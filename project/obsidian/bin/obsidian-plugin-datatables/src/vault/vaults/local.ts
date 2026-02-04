import { Stats } from "fs"
import { readFile, readdir, stat, writeFile } from "fs/promises"
import type { CachedMetadata } from "obsidian"
import { dirname, join } from "path"

import { File, Vault } from "../vault"

const exists = async (path: string) => {
    try {
        await stat(path)
    } catch {
        return false
    }

    return true
}

export class LocalFile extends File {
    constructor(
        private virtualPath: string,
        /**
         * @internal
         */
        public readonly fsPath: string,
        private stats: Stats,
    ) {
        super()
    }

    override get path(): string {
        return this.virtualPath
    }

    override isFile(): boolean {
        return this.stats.isFile()
    }

    override isDirectory(): boolean {
        return this.stats.isDirectory()
    }
}

export class LocalVault extends Vault {
    constructor(private vaultRoot: string) {
        super()
    }

    public override cachedRead(file: File): Promise<string> {
        throw new Error("Method not implemented.")
    }
    public override cachedMetadata(file: File): Promise<CachedMetadata | null> {
        throw new Error("Method not implemented.")
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Util
    ////////////////////////////////////////////////////////////////////////////
    private getRealPath(input: string | File) {
        if (input instanceof LocalFile) {
            return input.fsPath
        }

        if (input instanceof File) {
            throw new Error(`InvariantError`)
        }

        return join(this.vaultRoot, input)
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Create
    ////////////////////////////////////////////////////////////////////////////
    public override async createEmptyMarkdownFile(folder: File, fileName: string): Promise<File> {
        const parentPath = this.getRealPath(folder)
        const filePath = join(parentPath, fileName)

        if (!(await exists(parentPath))) {
            throw new Error(
                `Failed to create markdown file. The parent directory ${folder.path} does not exist.`,
            )
        }

        if (await exists(filePath)) {
            throw new Error(
                `Failed to create markdown file. The file ${folder.path}/${fileName} already exists.`,
            )
        }

        await writeFile(filePath, "")

        return new LocalFile(join(folder.path, fileName), filePath, await stat(filePath))
    }

    public override async create(path: string, content: string): Promise<File> {
        const realPath = join(this.vaultRoot, path)
        const schemaDir = dirname(realPath)

        if (await exists(realPath)) {
            throw new Error(`Attempted to create file that already exists: ${realPath}`)
        }

        if (!(await exists(schemaDir))) {
            throw new Error(
                `The schema folder must exist before the schema file can be created: ${schemaDir}`,
            )
        }

        await writeFile(realPath, content)

        return new LocalFile(path, realPath, await stat(realPath))
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Read
    ////////////////////////////////////////////////////////////////////////////
    public override async read(file: File): Promise<string> {
        const path = this.getRealPath(file)

        if (file.isFile()) {
            return (await readFile(path)).toString()
        }

        throw new Error(`Cannot read file: ${path} is not a readable file`)
    }

    public override async getMarkdownFiles(): Promise<LocalFile[]> {
        const files = await readdir(this.vaultRoot, {
            recursive: true,
            withFileTypes: true,
        })

        const markdownFiles = files.filter((entry) => entry.isFile() && entry.path.endsWith(".md"))
        const resultFiles = []

        for (const file of markdownFiles) {
            let virtualPath = file.path.replace(this.vaultRoot, "")

            if (virtualPath.startsWith("/")) {
                virtualPath = virtualPath.substring(1)
            }

            resultFiles.push(new LocalFile(virtualPath, file.path, await stat(file.path)))
        }

        return resultFiles
    }

    public override async getFile(virtualPath: string): Promise<File | null> {
        const realPath = join(this.vaultRoot, virtualPath)

        if (await exists(realPath)) {
            return new LocalFile(virtualPath, realPath, await stat(realPath))
        }

        return null
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Update
    ////////////////////////////////////////////////////////////////////////////
    public override async modifyContent(file: File, content: string): Promise<void> {
        if (file.isDirectory()) {
            throw new Error(`TODO`)
        }

        await writeFile(this.getRealPath(file), content)
    }

    ////////////////////////////////////////////////////////////////////////////
    /// Other
    ////////////////////////////////////////////////////////////////////////////
    public override emitWithoutLock(event: string, content: any): void {
        void 0
    }

    public override register(callback: () => void): void {}
}
