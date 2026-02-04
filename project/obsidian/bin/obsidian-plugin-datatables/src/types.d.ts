import "obsidian"

declare module "obsidian" {
    interface Vault {
        on(name: "raw", callback: (fileName: string) => void): EventRef
    }

    interface Workspace {
        on(
            name: "obby:schema:database-file-changed",
            callback: (data: { from: string; to: string }) => void,
        ): EventRef

        on(
            name: "datatables:index:changed" | "datatables:schema:changed",
            callback: () => void,
        ): EventRef
    }

    interface MarkdownPostProcessorContext {
        removeChild(child: MarkdownRenderChild): void
        forceReload(): void
    }

    interface FileManager {
        createNewMarkdownFile: (folder: TFolder | undefined, filename: string) => Promise<TFile>
    }
}

declare global {
    interface Document {
        __dev__dt_dark_mode: string
        __dev__dt_cleanup_hooks: (() => void)[]
    }
}
