# [0.5.0](https://github.com/DevYukine/red_oxide/compare/v0.4.0...v0.5.0) (2023-04-21)


### Bug Fixes

* **redacted:** correct Blu-ray -> Blu-Ray for api response ([aa72cb1](https://github.com/DevYukine/red_oxide/commit/aa72cb12604ee01a128c69f26f7056108791062e))


### Features

* **threads:** remove useless threads argument ([9cc7be8](https://github.com/DevYukine/red_oxide/commit/9cc7be8c937493f92092cfa26e28b872242388b8))
* **transcode:** add total progress bar, fix ghost bar showing up ([ba947c0](https://github.com/DevYukine/red_oxide/commit/ba947c0614e5cd161b3f70174e0c43ccea142074))



# [0.4.0](https://github.com/DevYukine/red_oxide/compare/v0.3.0...v0.4.0) (2023-04-20)


### Bug Fixes

* **path:** correctly parse reds file_path property for utf-8 chars ([342d6dc](https://github.com/DevYukine/red_oxide/commit/342d6dc1788681fe681acd9b80b48aaab4b2f73d))
* **spectrograms:** correct error message ([18035b4](https://github.com/DevYukine/red_oxide/commit/18035b48d3b6f48c13b4e1a6b0ca425be7b4e449))


### Features

* **spectrogram:** add progress bar for spectrogram creation ([386682d](https://github.com/DevYukine/red_oxide/commit/386682d17ebd192aa7d4905fdd9720834a89c62e))



# [0.3.0](https://github.com/DevYukine/red_oxide/compare/v0.2.0...v0.3.0) (2023-04-19)


### Features

* **config:** add missing options and flags to configmake & optional ([e23dba0](https://github.com/DevYukine/red_oxide/commit/e23dba042d8b2d0cbe4ab6f318897bd38db19009))



# [0.2.0](https://github.com/DevYukine/red_oxide/compare/dce30083d50b0ffd11eb3ada54d8fd9bf87df43f...v0.2.0) (2023-04-19)


### Bug Fixes

* Flac24 -> Flac do not need tags copied ([0264471](https://github.com/DevYukine/red_oxide/commit/02644717f10f507d8da5421b59675b81f57a2db0))
* **group_name:** make sure group names do not have colons in name ([e18b95b](https://github.com/DevYukine/red_oxide/commit/e18b95b89864de0671ebd1533aefc7c692769890))
* **imdl:** set correct source for RED ([39bb591](https://github.com/DevYukine/red_oxide/commit/39bb591a1addf440a0fa3f2d47f9480160fe494a))
* **red:** fix TRACKER_URL having an extra slash ([142db81](https://github.com/DevYukine/red_oxide/commit/142db819adb64ad7873a339b93e2cb8c5de5a16e))
* **spectrograms:** skip upload when spectrogram check is negative ([ad3198d](https://github.com/DevYukine/red_oxide/commit/ad3198d96789141fa3764dda615d2040da96c00e))
* **tags:** correctly check subfolders for flac files ([678d071](https://github.com/DevYukine/red_oxide/commit/678d071dc89364664462997e51dc9a23c154a5f5))
* **tags:** correctly copy tags for FLAC24 -> FLAC transcodes ([929a0c0](https://github.com/DevYukine/red_oxide/commit/929a0c07cf55075d2a228491f2d76a3de4c2d956))
* **tags:** correctly handle vinyl edge cases for track_number tags ([b53f16d](https://github.com/DevYukine/red_oxide/commit/b53f16d32447b2abea5091f7d381ad050c369979))
* **transcode:** correct display command for FLAC24 -> FLAC ([adf0fab](https://github.com/DevYukine/red_oxide/commit/adf0fab4325c59f83cfecf7978bdaef5052fe17c))
* **transcode:** correct use .flac file extension for FLAC24 -> FLAC ([080dab7](https://github.com/DevYukine/red_oxide/commit/080dab78e0a848852d4f0a97426837c27d409e10))
* **transcode:** fix path in recursion while copying over extra files ([74ab6e2](https://github.com/DevYukine/red_oxide/commit/74ab6e2f7d1aaa6e7e5860e480475a65cfe2f908))
* **upload:** correctly implement auto upload ([9b24231](https://github.com/DevYukine/red_oxide/commit/9b24231b9363e882f86e3cda5981d3e6d4afa92a))


### Features

* **cli:** add short alias for move_transcode_to_content ([77722ef](https://github.com/DevYukine/red_oxide/commit/77722ef6be62e4831acbd87afbf745682ddcdb10))
* implement manual mode ([4be871c](https://github.com/DevYukine/red_oxide/commit/4be871c0652303535725eb3c77c12d764db22637))
* initial upload functionality ([dce3008](https://github.com/DevYukine/red_oxide/commit/dce30083d50b0ffd11eb3ada54d8fd9bf87df43f))
* **spectrograms:** add filename info ([76892f5](https://github.com/DevYukine/red_oxide/commit/76892f5916dda0007de7463d0891700e7ad87c8e))
* **spectrograms:** add spectrogram creation and let users manually check ([34aad94](https://github.com/DevYukine/red_oxide/commit/34aad9428789c91a02a30be5c8e9c287fbe6d419))
* **upload:** add log message once upload is done ([5521e25](https://github.com/DevYukine/red_oxide/commit/5521e25d65942816ca9f423c160298c216bfb940))


### Performance Improvements

* **main:** remove double multichannel check ([44e0df3](https://github.com/DevYukine/red_oxide/commit/44e0df3b045e5797509f90c02867b4057d230a09))



