# red_oxide

CLI to help uploading to REDactec, inspired by REDBetter.

## Installing

1. Install [intermodal](https://github.com/casey/intermodal#installation) and add it to your PATH
2. Install lame, sox & flac and add them to your PATH
3. download the latest release from [here](https://github.com/DevYukine/red_oxide/releases)

## Usage

### CLI

You have to specify api-key, torrent-directory, content-directory, transcode-directory & spectrogram-directory either via the config file or via the CLI

```
Arguments:
  [URLS]...  The perma links of torrents to transcode

Options:
      --debug
          If debug logs should be shown
  -a, --automatic-upload
          If the upload should be done automatically
      --transcode-in-parallel
          If multiple formats should be transcoded in parallel (this will increase memory & cpu usage a lot, make sure you can handle it)
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

This is useful if you don't want a super long CLI command and your configs do not change often

```json
{
  "api_key": "YOUR_API_KEY",
  "torrent_directory": "FULL_PATH_WHERE_TORRENT_FILES_WILL_BE_STORED",
  "content_directory": "FULL_PATH_WHERE_CONTENT_IS_LOCATED",
  "transcode_directory": "FULL_PATH_WHERE_TRANSCODED_CONTENT_WILL_BE_PUT",
  "spectrogram_directory": "FULL_PATH_WHERE_SPECTROGRAMS_WILL_BE_PUT",
  "move_transcode_to_content": true,
  "automatic_upload": true
}

```

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
