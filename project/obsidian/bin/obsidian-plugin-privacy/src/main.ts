import { Logger } from "@basalt/obsidian-logger"
import "@total-typescript/ts-reset"
import { Plugin } from "obsidian"
import { networkInterfaces } from "os"

/**
 * This plugin is responsible for determining whether obsidian has
 * any kind of network connectivity. If it does, obsidian will exit.
 *
 * The method to do so is rather crude, only checking to see whether or not
 * a network interface even exists. This is doable because I run obsidian
 * in a dedicated network namespace, so it will not have any interfaces.
 */
export default class BasaltOfflinePlugin extends Plugin {
    private logger = new Logger(BasaltOfflinePlugin.name)

    override async onload() {
        this.registerChecker()

        this.logger.log(`[${this.manifest.id}] loaded version ${this.manifest.version}`)
    }

    private registerChecker() {
        if (this.isOnline()) {
            this.exitElectron()
        }

        const interval = setInterval(() => {
            if (this.isOnline()) {
                this.exitElectron()
            }
        }, 5000)

        this.register(() => {
            clearTimeout(interval)
        })
    }

    private isOnline(): boolean {
        if (window.navigator.onLine) {
            return true
        }

        const interfaces = networkInterfaces()

        return Object.keys(interfaces).length !== 0
    }

    private exitElectron(): void {
        console.log(`[${this.manifest.id}] Detected a network connection. Exiting electron.`)

        try {
            window.electron.remote.app.quit()
        } catch (e) {
            // This shouldn't happen, but if it does we can kill the
            // immediate electron window without killing the underlying
            // process.
            process.exit(0)
        }

        setTimeout(() => {
            process.exit(0)
        }, 5000)
    }
}
