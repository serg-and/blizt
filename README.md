# blizt

A blazingly fast type validation librari for Javascript runtimes, leveraging the power of native rust code.

Still under development.

Preliminary benchmark results (see [Bench](./bench/)):

```bash
$ bun run index.ts

 > Parse number (2 > n < 50)                     warmup: 10 runs: 1000
------------------------------------------------------------------------
 Blizt                                                     221 ns/iter
 Zod                                                      1782 ns/iter
 Yup                                                      5420 ns/iter
 Joi                                                      3963 ns/iter
 Valibot                                                   899 ns/iter
 myzod                                                     457 ns/iter

 > Parse boolean[4]                              warmup: 10 runs: 1000
------------------------------------------------------------------------
 Blizt                                                     382 ns/iter
 Zod                                                      2554 ns/iter
 Yup                                                     15531 ns/iter
 Joi                                                      9701 ns/iter
 Valibot                                                   867 ns/iter
 myzod                                                     507 ns/iter

 > Parse boolean[100]                            warmup: 10 runs: 1000
------------------------------------------------------------------------
 Blizt                                                    5173 ns/iter
 Zod                                                      9274 ns/iter
 Yup                                                    123931 ns/iter
 Joi                                                     44693 ns/iter
 Valibot                                                  2336 ns/iter
 myzod                                                    1645 ns/iter

 > Parse tuple(string, number)                   warmup: 10 runs: 1000
------------------------------------------------------------------------
 Blizt                                                     393 ns/iter
 Zod                                                      2745 ns/iter
 Yup                                                     12503 ns/iter
 Joi                                                      6130 ns/iter
 Valibot                                                   801 ns/iter
 myzod                                                     686 ns/iter

 > Parse union (4 subtypes)                      warmup: 10 runs: 1000
------------------------------------------------------------------------
 Blizt                                                     219 ns/iter
 Zod                                                      6054 ns/iter
 Joi                                                      9035 ns/iter
 Valibot                                                  1569 ns/iter
 myzod                                                    3760 ns/iter

 > Parse simple object                           warmup: 10 runs: 1000
------------------------------------------------------------------------
 Blizt                                                     498 ns/iter
 Zod                                                      3007 ns/iter
 Yup                                                     15187 ns/iter
 Joi                                                      6364 ns/iter
 Valibot                                                   668 ns/iter
 myzod                                                     777 ns/iter

 > Parse complex object                          warmup: 10 runs: 1000
------------------------------------------------------------------------
 Blizt                                                    3202 ns/iter
 Zod                                                      9299 ns/iter
 Yup                                                     70602 ns/iter
 Joi                                                     35570 ns/iter
 Valibot                                                  3914 ns/iter
 myzod                                                    3523 ns/iter
```
