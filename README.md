## Getting started

First, you need to download the binary file:

```
curl -LJO https://github.com/fgerschau/image-downloader/releases/download/v0.0.1-alpha
```

Then unzip the file like this. The name may differ depending on the machine you're working with:

```
unzip image-downloader_v0.0.1-alpha_x86_64-apple-darwin.zip
```

On macOS, you need to allow the binary to be executed since it's from an unidentified developer:

```
xattr -r -d com.apple.quarantine image-downloader
```

Move the binary file to the bin folder:

```
mv image-downloader /usr/local/bin
```

And finally, you can add the following to your bash profile:

```
export PATH=$PATH:/usr/local/bin/image-downloader
```

You can now run the `image-downloader` command anywhere in your terminal.

## Development

To run the project locally:

```
cargo run
```
