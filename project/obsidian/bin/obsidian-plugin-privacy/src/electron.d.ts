import "electron/electron"

declare global {
    interface Window {
        electron: {
            remote: Electron.RemoteMainInterface
        }
    }
}
