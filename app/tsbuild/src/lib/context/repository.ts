import { dirname, join } from "path"
import { readFile } from "fs/promises"
import { glob } from "glob"
import yaml from "js-yaml"

import { exists } from "../util/fs.js"
import {
    type PackageManager,
    type Workspace,
    getPackageManager,
    getWorkspaceFile,
} from "../util/package-manager.js"
import { Context } from "./context.js"

export class RepositoryContext extends Context {
    private _project:
        | {
              name: string
              path: string
              dependencies: string[]
              dependencyVersions: Map<string, string>
          }
        | undefined
    private workspace: Workspace | undefined

    private repositoryRoot!: string
    private packageManager!: PackageManager

    override async initialise(): Promise<void> {
        const packageManager = await getPackageManager(process.cwd())
        if (!packageManager) {
            throw new Error(
                "Unable to determine the package manager in use. Have you installed dependencies and generated a lockfile?",
            )
        }

        this.packageManager = packageManager
        this.repositoryRoot = dirname(this.packageManager.path)

        this._project = await this.getCurrentProject()
        this.workspace = await getWorkspaceFile(this.packageManager)
    }

    override async terminate(): Promise<void> {}

    ///==========================================================================
    /// Accessors
    ///=========================================================================
    public get project(): {
        name: string
        path: string
        dependencies: string[]
        dependencyVersions: Map<string, string>
    } {
        if (!this._project) {
            throw new Error("RepositoryContext has not been intialised")
        }

        return this._project
    }

    ///==========================================================================
    /// Workspace
    ///=========================================================================
    public isWorkspace(): boolean {
        return !!this.workspace
    }

    public async locateProjects(): Promise<Array<{ name: string; path: string }>> {
        if (!this.workspace) {
            return []
        }

        const content = await readFile(this.workspace.path, "utf8")
        const json = yaml.load(content) as { packages: string[] }

        const result = await glob(json.packages, { cwd: this.repositoryRoot })

        const jsons = await Promise.all(
            result
                .map((path) => join(path, "package.json"))
                .map(async (path) => {
                    const fullPath = join(this.repositoryRoot, path)
                    if (!(await exists(fullPath))) {
                        return null
                    }

                    const data = JSON.parse(await readFile(fullPath, "utf8")) as Record<
                        string,
                        unknown
                    >

                    if ("name" in data) {
                        return {
                            name: data["name"] as string,
                            path: dirname(fullPath),
                        }
                    }

                    return null
                }),
        )

        return jsons.filter(Boolean)
    }

    ///=========================================================================
    /// Configuration
    ///=========================================================================
    public async getCurrentProject(): Promise<{
        name: string
        path: string
        dependencies: Array<string>
        dependencyVersions: Map<string, string>
    }> {
        const path = join(process.cwd(), "package.json")

        if (!(await exists(path))) {
            throw new Error("Not in a project")
        }

        const data = JSON.parse(await readFile(path, "utf8")) as Record<string, unknown>

        const dependencies = []
        const dependencyVersions: Map<string, string> = new Map()

        if ("dependencies" in data && typeof data["dependencies"] === "object") {
            const deps = data["dependencies"] as Record<string, string>

            dependencies.push(...Object.keys(deps ?? []))

            for (const dep in deps) {
                dependencyVersions.set(dep, deps[dep] as string)
            }
        }
        if ("devDependencies" in data && typeof data["devDependencies"] === "object") {
            const deps = data["devDependencies"] as Record<string, string>
            dependencies.push(...Object.keys(data["devDependencies"] ?? []))
            for (const dep in deps) {
                dependencyVersions.set(dep, deps[dep] as string)
            }
        }
        if ("peerDependencies" in data && typeof data["peerDependencies"] === "object") {
            dependencies.push(...Object.keys(data["peerDependencies"] ?? []))
        }

        if ("name" in data) {
            return {
                name: data["name"] as string,
                path: dirname(path),
                dependencies,
                dependencyVersions,
            }
        }

        throw new Error("Not in a project")
    }
}
