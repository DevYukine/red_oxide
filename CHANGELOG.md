## [0.8.1](https://github.com/DevYukine/red_oxide/compare/v0.8.0...v0.8.1) (2024-03-01)


### Bug Fixes

* **redacted:** minimalize url bbcode to improve integration with other tools ([6a0ac4b](https://github.com/DevYukine/red_oxide/commit/6a0ac4bfc0b3d58e19f671dc2075444491b50c44))
* **spectrogram:** use correct error message for failing to create spectrogram ([132aac3](https://github.com/DevYukine/red_oxide/commit/132aac347d2e6c670d822fd3c0ab97a4e40441f8))



# [0.8.0](https://github.com/DevYukine/red_oxide/compare/v0.7.2...v0.8.0) (2024-01-16)


### Bug Fixes

* **config:** use correct HOME_ENV constant ([ee3584a](https://github.com/DevYukine/red_oxide/commit/ee3584aeaa6b520959c4d466fc5d3bb5f3c0f5e5))
* **transcode:** do not drop reference while borrowed ([00c3ffd](https://github.com/DevYukine/red_oxide/commit/00c3ffda9fbbc7fbf0551b70dbf045b3e593db1a))
* **transcode:** fallback if no extension exists ([346e3d3](https://github.com/DevYukine/red_oxide/commit/346e3d3d0c09d66feda1178012a8153f4bb226ab))


### Features

* add default paths for config file ([c823b08](https://github.com/DevYukine/red_oxide/commit/c823b08239a4aa7c4dbf212c021518f51a0224f0))



## [0.7.2](https://github.com/DevYukine/red_oxide/compare/v0.7.1...v0.7.2) (2024-01-15)


### Bug Fixes

* **ci:** set missing job output ([2334236](https://github.com/DevYukine/red_oxide/commit/2334236fe368d187f0320074e305e522208134bf))



## [0.7.1](https://github.com/DevYukine/red_oxide/compare/v0.7.0...v0.7.1) (2024-01-15)


### Bug Fixes

* **ci:** reference the created tag for release builds ([f9e1d91](https://github.com/DevYukine/red_oxide/commit/f9e1d91a461cdd367999cc75bb4e34b7d26686ad))



# [0.7.0](https://github.com/DevYukine/red_oxide/compare/v0.6.0...v0.7.0) (2024-01-15)


### Bug Fixes

* ignore scene releases as descening is not supported currently ([b0470f7](https://github.com/DevYukine/red_oxide/commit/b0470f7b9deb989d60ec66e919205d3f99aea598))
* **permalink:** properly handle error when permalink can't be parsed ([d92aa0b](https://github.com/DevYukine/red_oxide/commit/d92aa0b40598d418cb41ad9c3e355d21ec4fd7f2))


### Features

* add concurrency option to specify how many tasks should run concurrently ([baee787](https://github.com/DevYukine/red_oxide/commit/baee787e820a50229a24d89fff5381a0db8ec344))
* add skip_existing_formats_check flag ([6a7bae8](https://github.com/DevYukine/red_oxide/commit/6a7bae8690461274eb02f759ab69471adb08b25c))
* don't fail on errors, replace forbidden folder/filename characters ([44b3486](https://github.com/DevYukine/red_oxide/commit/44b3486738a590ff1b7b5cec18a7b607d1fbc222))
* **lossy:** add a warning that lossy web/master transcode require manual report ([784a01f](https://github.com/DevYukine/red_oxide/commit/784a01f659d140da55071886d385eda406c0d096))


### Performance Improvements

* **spectrograms:** add support for concurrency option ([1ecf337](https://github.com/DevYukine/red_oxide/commit/1ecf3377fa4fda9808e103adb5daa34361917e48))



