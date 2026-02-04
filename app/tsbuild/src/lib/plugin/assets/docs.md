# Assets

Copies assets to the output folder.

## Why?

It is common practice to colocate assets (images, sounds, etc) within the
client repository that are not directly bundled into the output.

This is useful for local development, but we need to be able to move files
to the proper place for their use within a production environment.

This plugin enables us to copy static assets to the proper place.

## Usage

The plugin exposes a single `assetsPlugin` function which can be used as
an esbuild plugin:

```javascript
esbuild({
    plugins: [
        assetsPlugin({
            fromDirectory: "path/to/static/assets",
            toDirectory: "path/to/output/directory",
            assets: [
                {
                    input: "assets",
                    glob: "**/*"
                }
            ]
        })
    ]
}
```

### Arguments

`assetsPlugin` takes a single argument with the following structure:

```javascript
{
    // The relative, or preferrably absolute path to the folder that contains
    // the assets we want to copy.
    fromDirectory: string,
    // The relative, or preferrably absolute path to the folder that the assets
    // will be copied to. This is commonly `dist` or some subfolder within the
    // `dist` directory.
    toDirectory: string,
    // An array that can contain either a static path relative to the `fromDirectory`,
    // or an `Asset` entry which supports globbing. See below for more details.
    assets: (string | Asset)[]
}
```

### Assets

Assets are defined using either relative path strings, or asset objects.

#### String

An asset string is a relative path to a singular file or folder on the filesystem.
When using string paths, nested folder structures will not be preserved, for example:

```javascript
{
    toDirectory: "../dist",
    assets: [
        "someFolder/file.txt"
    ],
}
```

In this example, `file.txt` will appear in the output as `dist/file.txt` **not** `dist/someFolder/file.txt`.

In order to preserve folder structures, asset objects and globbing must be used.

#### Asset Object

An asset object has the following structure:

```javascript
{
    // Similar to string paths, this is a path to a file or folder.
    input: string,
    // An optional glob may be provided to narrow or expand searches. When using globs,
    // the underlying folder structure will be preserved as files are copied.
    glob?: string,
    // We may specificy a folder under `fromDirectory` at
    // which files will be copied to.
    output?: string,
}
```
