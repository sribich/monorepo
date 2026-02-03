import { Server } from "../server.js"

export class AppServer extends Server {
    override async start(/*result: BuildResult<BuildOptions>, entrypoint: string*/): Promise<void> {
        /*
        const entrypointBasename = parse(basename(entrypoint)).name

        if (result.metafile) {
            const outFile = Object.keys(result.metafile.outputs).find((file) => {
                return parse(file).name === entrypointBasename
            })

            if (!outFile) {
                return
            }

            await killCurrentProcess()

            runProcess(outFile, this.config.nodeArgs as string[])
        }
        */
    }
}
