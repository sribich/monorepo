import { exec } from "node:child_process"
import { join, dirname } from "node:path"
import { stat } from "node:fs/promises"
import { cpSync, existsSync, rmSync, writeFile } from "node:fs"
import type { AstroIntegration } from "astro"
import chokidar from "chokidar"
import { findUp } from "find-up"

export type DocsOptions = {
    outDir: string
}

export type DocsIntegrations = {
    integrations: AstroIntegration[]
    sidebars: Sidebar[]
}

type Sidebar = {
    label: string
    autogenerate?: {
        directory: string
        // { label: 'Example Guide', link: '/guides/example/' },
    }
}

type DocRoot = {
    name: string
    root: string
}

const loadProjects = async () => {
    const projects: string = await new Promise((resolve, reject) => {
        exec("moon query projects", (error, stdout, stderr) => {
            if (error) {
                return reject(error)
            }

            if (stderr) {
                return reject(new Error(stderr))
            }

            return resolve(stdout)
        })
    })

    return projects
        .split("\n")
        .map((item) => {
            const split = item.split("|")

            if (split[0] && split[1]) {
                return [split[0].trim(), split[1].trim()]
            }

            return null
        })
        .filter(Boolean) as [string, string][]
}

const filterDocRoots = async (
    projects: [string, string][],
    workspaceRoot: string,
): Promise<DocRoot[]> => {
    const roots = projects.map((it) => {
        const root = join(workspaceRoot, it[1], "docs")

        return new Promise<DocRoot | null>((resolve) => {
            stat(root)
                .then(() =>
                    resolve({
                        name: it[0],
                        root,
                    }),
                )
                .catch(() => resolve(null))
        })
    })

    return (await Promise.all(roots)).filter((it): it is DocRoot => it !== null)
}

export const generateDocs = async (options: DocsOptions): Promise<DocsIntegrations> => {
    const rootDirectory = await findUp(".moon", { type: "directory" })

    if (!rootDirectory) {
        throw new Error("not a moon repo")
    }

    const workspaceDirectory = dirname(rootDirectory)

    const projects = await loadProjects()
    const docProjects = await filterDocRoots(projects, workspaceDirectory)

    const outDir = options.outDir

    const watcher = chokidar.watch([], {
        persistent: true,
        ignoreInitial: true,
    })
    const sidebar = [] as Sidebar[]

    docProjects.forEach((it) => {
        const destRoot = join(outDir, it.name)

        if (existsSync(destRoot)) {
            rmSync(destRoot, { recursive: true })
        }

        cpSync(it.root, destRoot, { force: true, recursive: true })

        watcher.add(it.root)

        sidebar.push({
            label: it.name,
            autogenerate: { directory: `generated/${it.name}` },
        })
    })

    const getUpdateInfo = (path: string) => {
        const project = docProjects.find((it) => path.includes(it.root))

        if (!project) {
            return
        }

        const relativePath = path.replace(project.root, "")

        return { project, relativePath }
    }

    // TODO: Make sure that any of these joined paths cannot escape the current directory
    return {
        integrations: [
            {
                name: "astro-plugin-docs",
                hooks: {
                    "astro:config:setup": () => {
                        watcher.on("add", (path) => {
                            const info = getUpdateInfo(path)

                            if (info) {
                                cpSync(path, join(outDir, info.project.name, info.relativePath))
                            }
                        })
                        watcher.on("change", (path) => {
                            const info = getUpdateInfo(path)

                            if (info) {
                                cpSync(path, join(outDir, info.project.name, info.relativePath))
                            }
                        })
                        watcher.on("unlink", (path) => {
                            const info = getUpdateInfo(path)

                            if (info) {
                                rmSync(join(outDir, info.project.name, info.relativePath))
                            }
                        })
                        watcher.on("unlinkDir", () => {
                            // TODO
                        })
                    },
                    "astro:server:done": () => {
                        watcher.close()
                    },
                },
            },
        ],
        sidebars: sidebar,
    }
}
