import { BenchSuite } from './benchSuite'
import b, { type BValue } from '../index'
import z from 'zod'
import y from 'yup'
import j from 'joi'
import m, { type AnyType as MyzodAnyType } from 'myzod'
import * as v from 'valibot'
import chalk from 'chalk'

const blzS = (schema: BValue) => ({
  name: chalk.bold(chalk.cyanBright('Blizt')),
  schema,
})
const zodS = (schema: z.ZodTypeAny) => ({ name: 'Zod', schema })
const yupS = (schema: { validateSync: (value: unknown) => unknown }) => ({
  name: 'Yup',
  schema: { parse: (value: unknown) => schema.validateSync(value) },
})
const joiS = (schema: { validate: (value: unknown) => unknown }) => ({
  name: 'Joi',
  schema: { parse: (value: unknown) => schema.validate(value) },
})
const valiS = (schema: v.BaseSchema) => ({
  name: 'Valibot',
  schema: { parse: (value: unknown) => v.parse(schema, value) },
})
const myzS = (schema: MyzodAnyType) => ({ name: 'myzod', schema })

const suite = new BenchSuite({
  warmup: 10,
  count: 1000,
})

suite.addBench({
  name: 'Parse number (2 > n < 50)',
  createValue: () => 2 + Math.random(),
  variants: [
    blzS(b.number().min(2).max(50)),
    zodS(z.number().min(2).max(50)),
    yupS(y.number().min(2).max(50)),
    joiS(j.number().min(2).max(50)),
    valiS(v.number([v.minValue(2), v.maxValue(50)])),
    myzS(m.number().min(2).max(50)),
  ],
})

suite.addBench({
  name: 'Parse boolean[4]',
  createValue: () =>
    Array.from({ length: 4 }).map(() => !!(Math.random() > 0.5)),
  variants: [
    blzS(b.array(b.boolean())),
    zodS(z.array(z.boolean())),
    yupS(y.array(y.boolean())),
    joiS(j.array().items(j.boolean())),
    valiS(v.array(v.boolean())),
    myzS(m.array(m.boolean())),
  ],
})

suite.addBench({
  name: 'Parse boolean[100]',
  createValue: () =>
    Array.from({ length: 100 }).map(() => !!(Math.random() > 0.5)),
  variants: [
    blzS(b.array(b.boolean())),
    zodS(z.array(z.boolean())),
    yupS(y.array(y.boolean())),
    joiS(j.array().items(j.boolean())),
    valiS(v.array(v.boolean())),
    myzS(m.array(m.boolean())),
  ],
})

suite.addBench({
  name: 'Parse tuple(string, number)',
  createValue: () => [Math.random().toString(), Math.random() * 360],
  variants: [
    blzS(b.tuple(b.string(), b.number().min(0).max(360))),
    zodS(z.tuple([z.string(), z.number().min(0).max(360)])),
    yupS(y.tuple([y.string(), y.number().min(0).max(360)])),
    joiS(j.array().ordered(j.string(), j.number().min(0).max(360))),
    valiS(v.tuple([v.string(), v.number([v.minValue(0), v.maxValue(360)])])),
    myzS(m.tuple([m.string(), m.number().min(0).max(360)])),
  ],
})

suite.addBench({
  name: 'Parse union (4 subtypes)',
  createValue: () => Math.random(),
  variants: [
    blzS(b.union(b.string(), b.boolean(), b.number(), b.array(b.number()))),
    zodS(z.union([z.string(), z.boolean(), z.number(), z.array(z.number())])),
    joiS(
      j.alternatives(
        j.string(),
        j.boolean(),
        j.number(),
        j.array().items(j.number())
      )
    ),
    valiS(v.union([v.string(), v.boolean(), v.number(), v.array(v.boolean())])),
    myzS(m.union([m.string(), m.boolean(), m.number(), m.array(m.number())])),
  ],
})

suite.addBench({
  name: 'Parse simple object',
  createValue: () => ({ a: true, b: 0 }),
  variants: [
    blzS(b.object({ a: b.boolean(), b: b.number() })),
    zodS(z.object({ a: z.boolean(), b: z.number() })),
    yupS(y.object({ a: y.boolean(), b: y.number() })),
    joiS(j.object({ a: j.boolean(), b: j.number() })),
    valiS(v.object({ a: v.boolean(), b: v.number() })),
    myzS(m.object({ a: m.boolean(), b: m.number() })),
  ],
})

suite.addBench({
  name: 'Parse complex object',
  createValue: () => ({
    numArray: Array.from({ length: Math.floor(Math.random() * 10) }).map(() =>
      Math.random()
    ),
    longStrArray: Array.from({
      length: 3 + Math.floor(Math.random() * 10),
    }).map(() => '*'.repeat(10)),
    nestedObj: {
      a: 'A',
      n: 100 + Math.random() * 200,
      u: undefined,
      nl: null,
      bNl: Math.random() > 0.5 ? Math.random() > 0.5 : null,
      objOpt: Math.random() > 0.5 ? {} : undefined,
    },
  }),
  variants: [
    blzS(
      b.object({
        numArray: b.array(b.number()).max(10),
        longStrArray: b.array(b.string().length(10)).min(3).max(13),
        nestedObj: b.object({
          a: b.string(),
          n: b.number().min(100).max(300),
          u: b.undefined(),
          nl: b.Null(),
          bNl: b.boolean().nullable(),
          objOpt: b.object({}).optional(),
        }),
      })
    ),
    zodS(
      z.object({
        numArray: z.array(z.number()).max(10),
        longStrArray: z.array(z.string().length(10)).min(3).max(13),
        nestedObj: z.object({
          a: z.string(),
          n: z.number().min(100).max(300),
          u: z.undefined(),
          nl: z.null(),
          bNl: z.boolean().nullable(),
          objOpt: z.object({}).optional(),
        }),
      })
    ),
    yupS(
      y.object({
        numArray: y.array(y.number()).max(10),
        longStrArray: y.array(y.string().length(10)).min(3).max(13),
        nestedObj: y.object({
          a: y.string(),
          n: y.number().min(100).max(300),
          u: y.mixed().optional(),
          nl: y.mixed().nullable(),
          bNl: y.boolean().nullable(),
          objOpt: y.object({}).optional(),
        }),
      })
    ),
    joiS(
      j.object({
        numArray: j.array().items(j.number()).max(10),
        longStrArray: j.array().items(j.string().length(10)).min(3).max(13),
        nestedObj: j.object({
          a: j.string(),
          n: j.number().min(100).max(300),
          u: j.optional(),
          nl: j.equal(null),
          bNl: j.boolean().allow(null),
          objOpt: j.object({}).optional(),
        }),
      })
    ),
    valiS(
      v.object({
        numArray: v.array(v.number(), [v.maxLength(10)]),
        longStrArray: v.array(v.string([v.length(10)]), [
          v.minLength(3),
          v.maxLength(13),
        ]),
        nestedObj: v.object({
          a: v.string(),
          n: v.number([v.minValue(100), v.maxValue(300)]),
          u: v.undefined_(),
          nl: v.null_(),
          bNl: v.nullable(v.boolean()),
          objOpt: v.optional(v.object({})),
        }),
      })
    ),
    myzS(
      m.object({
        numArray: m.array(m.number()).max(10),
        longStrArray: m.array(m.string().min(10).max(10)).min(3).max(13),
        nestedObj: m.object({
          a: m.string(),
          n: m.number().min(100).max(300),
          u: m.undefined(),
          nl: m.null(),
          bNl: m.boolean().nullable(),
          objOpt: m.object({}).optional(),
        }),
      })
    ),
  ],
})

await suite.run()
