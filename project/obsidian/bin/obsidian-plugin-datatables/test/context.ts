// TODO: Fix this by modifying tsbuild to allow explicit ts excludes external to our
//       tsconfig file so that we don't need to manage multiple configs externally.
//       Maybe make only src default
// @ts-nocheck
import { cp, mkdir, mkdtemp, stat } from "fs/promises"
import { dirname, join, sep } from "path"
import { URL, fileURLToPath } from "url"
import { type TestAPI, test } from "vitest"

import { Index } from "../src/index/index"
import { SettingsContainer } from "../src/settings/settings-container"
import { LocalVault } from "../src/vault/vaults/local"

const FIXTURES_DIR = join(dirname(fileURLToPath(new URL(import.meta.url))), "fixtures")

const TMP_DIR = join(dirname(fileURLToPath(new URL(import.meta.url))), "tmp")

export interface Fixture {
    settings: SettingsContainer
    index: Index
    vault: LocalVault
}

export interface TestContext {
    loadFixture: (path: string) => Promise<Fixture>
}

export const it = test.extend<TestContext>({
    loadFixture: async ({ task }, use) =>
        use(async (fixture: string): Promise<Fixture> => {
            const fixtureRoot = join(FIXTURES_DIR, fixture)

            try {
                await stat(fixtureRoot)
            } catch {
                throw new Error(`The requested fixture does not exist: ${fixture}`)
            }

            await mkdir(`${TMP_DIR}${sep}${task.id}`, { recursive: true })
            const fixtureTestDir = await mkdtemp(`${TMP_DIR}${sep}${task.id}${sep}`)

            await cp(`${fixtureRoot}/`, `${fixtureTestDir}/`, { recursive: true })

            const vault = new LocalVault(fixtureTestDir)

            return {
                settings: new SettingsContainer({
                    schema: {
                        templateDir: "Templates",
                        folder: "Schema",
                        datatablesFile: "schema.json",
                    },
                }),
                index: await Index.create(vault),
                vault,
            }
        }),
}) satisfies TestAPI<{
    loadFixture: (path: string) => Promise<Fixture>
}>
