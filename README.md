# blizt

A blazingly fast type validation library for Javascript runtimes, leveraging the power of native rust code.

Still under development, more data types are comming and parsing large objects can be sped up.

Preliminary benchmark results (see [Bench](./bench/)):

```bash
# Ryzen 5600 32GB
$ bun run index.ts

 > Parse number (2 > n < 50)                     warmup: 10 runs: 1000
------------------------------------------------------------------------
 Blizt                                                     237 ns/iter
 Zod                                                      1389 ns/iter
 Yup                                                      5851 ns/iter
 Joi                                                      3989 ns/iter
 Valibot                                                   887 ns/iter
 myzod                                                     455 ns/iter

 > Parse boolean[4]                              warmup: 10 runs: 1000
------------------------------------------------------------------------
 Blizt                                                     238 ns/iter
 Zod                                                      2472 ns/iter
 Yup                                                     14208 ns/iter
 Joi                                                      9523 ns/iter
 Valibot                                                   853 ns/iter
 myzod                                                     595 ns/iter

 > Parse boolean[100]                            warmup: 10 runs: 1000
------------------------------------------------------------------------
 Blizt                                                    2835 ns/iter
 Zod                                                      9174 ns/iter
 Yup                                                    125434 ns/iter
 Joi                                                     44986 ns/iter
 Valibot                                                  2268 ns/iter
 myzod                                                    1611 ns/iter

 > Parse tuple(string, number)                   warmup: 10 runs: 1000
------------------------------------------------------------------------
 Blizt                                                     359 ns/iter
 Zod                                                      2753 ns/iter
 Yup                                                     12964 ns/iter
 Joi                                                      5908 ns/iter
 Valibot                                                   759 ns/iter
 myzod                                                     713 ns/iter

 > Parse union (4 subtypes)                      warmup: 10 runs: 1000
------------------------------------------------------------------------
 Blizt                                                     228 ns/iter
 Zod                                                      6020 ns/iter
 Joi                                                      8886 ns/iter
 Valibot                                                  1598 ns/iter
 myzod                                                    3860 ns/iter

 > Parse simple object                           warmup: 10 runs: 1000
------------------------------------------------------------------------
 Blizt                                                     475 ns/iter
 Zod                                                      2989 ns/iter
 Yup                                                     15327 ns/iter
 Joi                                                      6195 ns/iter
 Valibot                                                   710 ns/iter
 myzod                                                     812 ns/iter

 > Parse complex object                          warmup: 10 runs: 1000
------------------------------------------------------------------------
 Blizt                                                    2554 ns/iter
 Zod                                                      9501 ns/iter
 Yup                                                     69895 ns/iter
 Joi                                                     35500 ns/iter
 Valibot                                                  3750 ns/iter
 myzod                                                    3520 ns/iter
```
