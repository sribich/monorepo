import { basename } from "node:path"
import { findUp } from "find-up"

export type PackageManager = (typeof packageManagers)[number] & {
    path: string
}

const packageManagers = [
    { name: "npm", lockfile: "package-lock.json" },
    { name: "pnpm", lockfile: "pnpm-lock.yaml" },
    { name: "yarn", lockfile: "yarn.lock" },
] as const

export type Workspace = (typeof workspaces)[number] & { path: string }

const workspaces = [{ name: "pnpm", workspaceFile: "pnpm-workspace.yaml" }] as const

export const getPackageManager = async (cwd: string): Promise<PackageManager | undefined> => {
    const possibleLockfiles = packageManagers.map((it) => it.lockfile)

    const path = await findUp(possibleLockfiles, { cwd })

    if (!path) {
        return undefined
    }

    const packageManager = packageManagers.find((it) => it.lockfile === basename(path))

    return packageManager ? { ...packageManager, path } : undefined
}

export const getWorkspaceFile = async (
    packageManager: PackageManager,
): Promise<Workspace | undefined> => {
    let workspaceFile: string

    switch (packageManager.name) {
        case "npm":
            return undefined
        case "pnpm":
            workspaceFile = "pnpm-workspace.yaml"
            break
        default:
            throw undefined
    }

    const path = await findUp(workspaceFile)

    if (!path) {
        return undefined
    }

    const workspace = workspaces.find((it) => it.workspaceFile === basename(path))

    return workspace ? { ...workspace, path } : undefined
}
