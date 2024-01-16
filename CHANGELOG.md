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



# [0.6.0](https://github.com/DevYukine/red_oxide/compare/v0.5.0...v0.6.0) (2023-04-23)


### Bug Fixes

* **redacted:** check if files exceed redacteds allowed path limit ([020afab](https://github.com/DevYukine/red_oxide/commit/020afabfda15655c00f6dee6b869fc1b0f65c593))


### Features

* add way to set allowed transcode targets in cli & via the config ([90d2b02](https://github.com/DevYukine/red_oxide/commit/90d2b0261345ab3fcb8d7c439419272f3d86a7c3))
* **redacted:** add version to upload description ([3b56f9d](https://github.com/DevYukine/red_oxide/commit/3b56f9d0e4dd48346bacf4bc80b201bf78715711))



