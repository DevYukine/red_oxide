# red_oxide

CLI to help uploading to REDacted, inspired by REDBetter.

## Installing

1. Install [intermodal](https://github.com/casey/intermodal#installation) and add it to your PATH
2. Install lame, sox & flac and add them to your PATH
3. download the latest release from [here](https://github.com/DevYukine/red_oxide/releases)

## Usage

### CLI

#### Transcode subcommand (red_oxide transcode)

You have to specify api-key, torrent-directory, content-directory, transcode-directory & spectrogram-directory either via the config file or via the CLI

```
Transcode FLACs to other co-existing formats

Usage: red_oxide transcode [OPTIONS] [URLS]...

Arguments:
  [URLS]...  The Perma URLs (PL's) of torrents to transcode

Options:
      --debug
          If debug logs should be shown
  -a, --automatic-upload
          If the upload should be done automatically
      --concurrency <CONCURRENCY>
          How many tasks (for transcoding as example) should be run in parallel, defaults to your CPU count
      --api-key <API_KEY>
          The Api key from Redacted to use there API with
      --content-directory <CONTENT_DIRECTORY>
          The path to the directory where the downloaded torrents are stored
      --transcode-directory <TRANSCODE_DIRECTORY>
          The path to the directory where the transcoded torrents should be stored
      --torrent-directory <TORRENT_DIRECTORY>
          The path to the directory where the torrents should be stored
      --spectrogram-directory <SPECTROGRAM_DIRECTORY>
          The path to the directory where the spectrograms should be stored
  -c, --config-file <CONFIG_FILE>
          The path to the config file
  -f, --allowed-transcode-formats <ALLOWED_TRANSCODE_FORMATS>
          List of allowed formats to transcode to, defaults to all formats if omitted [possible values: flac24, flac, mp3320, mp3-v0]
  -m, --move-transcode-to-content
          If the transcode should be moved to the content directory, useful when you want to start seeding right after you upload
      --skip-hash-check
          If the hash check of the original torrent should be skipped, defaults to false, not recommended and if enabled done at own risk!
      --skip-spectrogram
          If the spectrogram check of the original torrent should be skipped, defaults to false, not recommended and if enabled done at own risk!
  -d, --dry-run
          If this is a dry run, no files will be uploaded to Redacted
  -h, --help
          Print help

```

### Example config.json

This is useful if you don't want a super long CLI command and your configs do not change often, note that all the options can be specified via the CLI as well and are fully optional in this config file (will be merged with the CLI options if specified)

```json
{
  "api_key": "YOUR_API_KEY",
  "torrent_directory": "FULL_PATH_WHERE_TORRENT_FILES_WILL_BE_STORED",
  "content_directory": "FULL_PATH_WHERE_CONTENT_IS_LOCATED",
  "transcode_directory": "FULL_PATH_WHERE_TRANSCODED_CONTENT_WILL_BE_PUT",
  "spectrogram_directory": "FULL_PATH_WHERE_SPECTROGRAMS_WILL_BE_PUT",
  "move_transcode_to_content": true,
  "automatic_upload": true,
  "skip_hash_check": false,
  "skip_spectrogram": false,
  "allowed_transcode_formats": ["Flac", "Mp3320", "Mp3V0"],
  "concurrency": 16
}

```

### Notes for people using sox under windows

if you use the binaries from [here](https://sourceforge.net/projects/sox/files/sox/), and you want utf-8 support for paths (this is needed for Japanese/Chinese/Korean names in paths for example) you have to download the files from [here](https://anonfiles.com/g7i1G1m8z5/sox_windows_fix_zip) and follow the steps below

1. Extract the files from the zip
2. Run the PreferExternalManifest.reg file and let it overwrite the registry entry
3. Copy the sox.exe.manifest file to the folder where sox.exe is located
4. Enjoy sox working with utf-8 paths :)

## Built With

- [Rust](https://www.rust-lang.org/) - The language used
- [clap](https://github.com/clap-rs/clap) - CLI Framework
- [tokio](https://tokio.rs/) - Async runtime
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [serde](https://serde.rs/) - Serialization/Deserialization
- [intermodal](https://github.com/casey/intermodal) - Used for Torrent Hash checking & creation
- [audiotags](https://docs.rs/audiotags/latest/audiotags/) - Reading/Writing Audio Metadata

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code
of conduct, and the process for submitting pull requests to us.

## Versioning

We use [Semantic Versioning](http://semver.org/) for versioning. For the versions
available, see the [tags on this
repository](https://github.com/DevYukine/red_oxide/tags).

## Authors

- **[DevYukine](https://github.com/DevYukine)** - *Initial Work*

See also the list of
[contributors](https://github.com/DevYukine/red_oxide/contributors)
who participated in this project.

## License

This project is licensed under the [MIT](LICENSE) See the [LICENSE.md](LICENSE) file for details
