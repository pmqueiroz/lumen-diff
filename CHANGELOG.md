# Changelog

## [0.3.0](https://github.com/pmqueiroz/lumen-diff/compare/v0.2.0...v0.3.0) (2026-04-26)


### Features

* support .yml extension for config file ([d2441d1](https://github.com/pmqueiroz/lumen-diff/commit/d2441d1584ed400244e0ff36d55e2eae87717df2))


### Bug Fixes

* add timeouts to CDP operations to prevent process hang ([d249ae1](https://github.com/pmqueiroz/lumen-diff/commit/d249ae179e3c76838d3033e722bbcd78add0f0ad))
* format message before warn ([1ebbd2b](https://github.com/pmqueiroz/lumen-diff/commit/1ebbd2ba8b3e598d9b6d61f05cb968386b8a993c))
* handle --update flag in diff step ([6a2159e](https://github.com/pmqueiroz/lumen-diff/commit/6a2159e739689599ea8b4765305c1721b2226841))


### Performance Improvements

* eliminate double disk reads and skip diff image alloc on pass ([869f52e](https://github.com/pmqueiroz/lumen-diff/commit/869f52e21d34a9bd232c6d2e8a2cf20a7a50cbe8))
* replace fixed server sleep with readiness probe ([7c68f8d](https://github.com/pmqueiroz/lumen-diff/commit/7c68f8d144a1389ede65e3c043e017644b57a365))

## [0.2.0](https://github.com/pmqueiroz/lumen-diff/compare/v0.1.0...v0.2.0) (2026-03-15)


### Features

* limit chromium with args ([cda19f0](https://github.com/pmqueiroz/lumen-diff/commit/cda19f0495affd672aa2726fd2cee2adf85d8ba0))

## 0.1.0 (2026-03-15)


### Features

* add reusable action ([889fa8e](https://github.com/pmqueiroz/lumen-diff/commit/889fa8ee0957ab5e171eac420b5f6883cc01a550))
* append breakpoint suffix ([480c918](https://github.com/pmqueiroz/lumen-diff/commit/480c918b0e08fa7e4eb1d7a4ebb006876e51ce47))
* apply cli args to config ([a93c413](https://github.com/pmqueiroz/lumen-diff/commit/a93c41331cecb2d6df1073f56bde7a02843eeb40))
* better logging ([5f874cd](https://github.com/pmqueiroz/lumen-diff/commit/5f874cda954385f9930cbe774357eca3ac0faa8a))
* build story id from title and name ([6789bab](https://github.com/pmqueiroz/lumen-diff/commit/6789bab9833ae0fcc92c7a826f4c3d7637decf6b))
* calculate diff ([3d04c30](https://github.com/pmqueiroz/lumen-diff/commit/3d04c30277e491efd61b9e43a38612cc59c800f7))
* config file ([88a5c9d](https://github.com/pmqueiroz/lumen-diff/commit/88a5c9ddb01756e58943eb93b53b71b311043ed0))
* fetch stories using static build ([4e7d14a](https://github.com/pmqueiroz/lumen-diff/commit/4e7d14a0b1a28e5f72f2b31eba7ab18e9bc5b925))
* minimal storybook integration impl ([f9e5d77](https://github.com/pmqueiroz/lumen-diff/commit/f9e5d7712fc70157fe2ccd70f5a7b95c4276f17a))
* run scrapper with concurrency ([631ed44](https://github.com/pmqueiroz/lumen-diff/commit/631ed44d28bb0359caa3c16ef546463b6e1f02ee))
* serve static dir ([629c28b](https://github.com/pmqueiroz/lumen-diff/commit/629c28bc5acf5b15f2f7292d16d0070d45c1ae62))
* take screenshots ([7c4658a](https://github.com/pmqueiroz/lumen-diff/commit/7c4658abb59b4da07ed6e73239a047268cad6a4d))


### Performance Improvements

* early return if bytes are same ([3dfd259](https://github.com/pmqueiroz/lumen-diff/commit/3dfd259b0c79a2bedeb39b48cdcce3a1ecb23a7a))
