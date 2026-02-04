// import { type BuildOptions, build } from "esbuild"
// import { existsSync } from "fs"
// import { lstat, mkdir, open, readdir, rm } from "fs/promises"
// import { join } from "path"
// import { describe } from "vitest"
//
// import { assetsPlugin } from "./plugin.js"
// import type { CopyableAsset } from "./types.js"
//
// describe("esbuild-plugin-assets", () => {
//     const inputDirectory = join(__dirname, "../../test/input")
//     const outputDirectory = join(__dirname, "../../test/output")
//
//     const getBuildArgs = (assets: (string | CopyableAsset)[]): BuildOptions => ({
//         bundle: true,
//         entryPoints: [join(inputDirectory, "index.js")],
//         logLevel: "silent",
//         write: false,
//         plugins: [
//             assetsPlugin({
//                 fromDirectory: inputDirectory,
//                 toDirectory: outputDirectory,
//                 assets,
//             }),
//         ],
//     })
//
//     afterEach(async () => {
//         // Recreate the test/output directory after each test
//         {
//             const gitkeepFile = join(outputDirectory, ".gitkeep")
//
//             try {
//                 await lstat(outputDirectory)
//             } catch (_) {
//                 throw new Error(`Output directory '${outputDirectory}' does not exist.`)
//             }
//
//             await rm(outputDirectory, { recursive: true })
//             await mkdir(outputDirectory)
//
//             const gitkeep = await open(gitkeepFile, "a")
//             await gitkeep.close()
//         }
//     })
//
//     it("Does not copy assets if to/from directories are not absolute paths", async () => {
//         const logs: string[] = []
//
//         const errorLog = console.error
//         const infoLog = console.info
//
//         console.error = (log) => logs.push(log)
//         console.info = (log) => logs.push(log)
//
//         const args = getBuildArgs([])
//
//         args.plugins = [
//             assetsPlugin({
//                 toDirectory: "./relative/path",
//                 fromDirectory: "/tmp",
//                 assets: ["randomFileDoesNotExist"],
//             }),
//         ]
//
//         await build(args)
//
//         console.error = errorLog
//         console.info = infoLog
//
//         expect(logs.length).toEqual(4)
//         expect(logs[3]).toMatch("Assets will not be copied")
//
//         const outputContents = await readdir(outputDirectory)
//
//         expect(outputContents.length).toEqual(1)
//         expect(outputContents[0]).toEqual(".gitkeep")
//     })
//
//     it("Can copy a string file asset to the output directory", async () => {
//         await build(getBuildArgs(["fileTest.txt"]))
//
//         const exists = existsSync(join(outputDirectory, "fileTest.txt"))
//
//         expect(exists).toBe(true)
//     })
//
//     it("Can copy a nested string file to the output directory", async () => {
//         await build(getBuildArgs(["nestedFileTest/nestedFileTest.txt"]))
//
//         const exists = existsSync(join(outputDirectory, "nestedFileTest.txt"))
//
//         expect(exists).toBe(true)
//     })
//
//     it("Can copy a string directory asset to the output directory", async () => {
//         await build(getBuildArgs(["directoryTest"]))
//
//         const fileOneExists = existsSync(join(outputDirectory, "directoryTest/file.txt"))
//         const fileTwoExists = existsSync(
//             join(outputDirectory, "directoryTest/folder/nestedFile.txt"),
//         )
//
//         expect(fileOneExists).toBe(true)
//         expect(fileTwoExists).toBe(true)
//     })
//
//     it("Errors if a glob exists in an asset input", async () => {
//         console.warn = jest.fn()
//
//         const promise = build(
//             getBuildArgs([
//                 {
//                     input: "directoryTest/**",
//                 },
//             ]),
//         )
//
//         await expect(promise).rejects.toThrowError(
//             "assets-plugin: Asset inputs may not contain globs. Please define the glob pattern in the glob field instead.",
//         )
//         expect(console.warn).toBeCalledTimes(1)
//     })
//
//     it("Errors if a glob input is not a directory", async () => {
//         console.warn = jest.fn()
//
//         const promise = build(
//             getBuildArgs([
//                 {
//                     input: "directoryTest/file.txt",
//                     glob: "**/*",
//                 },
//             ]),
//         )
//
//         await expect(promise).rejects.toThrowError(
//             "assets-plugin: Using globs with file inputs is not supported.",
//         )
//         expect(console.warn).toBeCalledTimes(1)
//     })
//
//     it("Can copy narrowed glob assets to the output directory", async () => {
//         const shouldResolve = build(
//             getBuildArgs([
//                 {
//                     input: "directoryTest",
//                     glob: "**/nestedFile.txt",
//                 },
//             ]),
//         )
//
//         await expect(shouldResolve).resolves.toBeTruthy()
//
//         const outputContents = await readdir(outputDirectory)
//         expect(outputContents.length).toEqual(2)
//
//         const filteredContents = outputContents.filter((it) => [".gitkeep", "folder"].includes(it))
//         expect(filteredContents.length).toEqual(2)
//     })
//
//     it("Can copy full glob assets to the output directory", async () => {
//         const shouldResolve = build(
//             getBuildArgs([
//                 {
//                     input: "directoryTest",
//                     glob: "**/*",
//                 },
//             ]),
//         )
//
//         await expect(shouldResolve).resolves.toBeTruthy()
//
//         const outputContents = await readdir(outputDirectory)
//         expect(outputContents.length).toEqual(3)
//
//         const filteredContents = outputContents.filter((it) =>
//             [".gitkeep", "folder", "file.txt"].includes(it),
//         )
//         expect(filteredContents.length).toEqual(3)
//     })
// })
//
