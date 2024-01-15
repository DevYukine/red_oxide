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



# [0.6.0](https://github.com/DevYukine/red_oxide/compare/v0.5.0...v0.6.0) (2023-04-23)


### Bug Fixes

* **redacted:** check if files exceed redacteds allowed path limit ([020afab](https://github.com/DevYukine/red_oxide/commit/020afabfda15655c00f6dee6b869fc1b0f65c593))


### Features

* add way to set allowed transcode targets in cli & via the config ([90d2b02](https://github.com/DevYukine/red_oxide/commit/90d2b0261345ab3fcb8d7c439419272f3d86a7c3))
* **redacted:** add version to upload description ([3b56f9d](https://github.com/DevYukine/red_oxide/commit/3b56f9d0e4dd48346bacf4bc80b201bf78715711))



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



