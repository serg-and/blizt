import c, { type ChalkInstance } from 'chalk'
import Table from 'cli-table'

type Bench = {
  name: string
  createValue: (index: number) => unknown
  variants: {
    name: string
    schema: { parse: (value: unknown) => unknown }
  }[]
  warmup?: false | number
  count?: number
}

export class BenchSuite {
  private benchmarks: Bench[]
  private warmup: number | false = 2
  private count: number = 1000

  constructor(config?: { warmup?: number | false; count?: number }) {
    this.benchmarks = []
    if (!config) return
    if (config.warmup) this.warmup = config.warmup
    if (config.count) this.count = config.count
  }

  public addBench(bench: Bench) {
    this.benchmarks.push(bench)
  }

  public run() {
    for (const b of this.benchmarks) this.runBench(b)
  }

  private runBench(bench: Bench) {
    const warmup = bench.warmup ?? this.warmup
    const count = bench.count ?? this.count

    console.log()
    this.table(
      c.whiteBright(`> ${bench.name}`),
      c.gray(`warmup: ${c.yellowBright(warmup)} runs: ${c.yellow(count)}`),
      '-'
    )

    if (warmup) {
      const values = Array.from({ length: warmup }).map((_, i) =>
        bench.createValue(i)
      )
      for (const variant of bench.variants) {
        for (const value of values) variant.schema.parse(value)
      }
    }

    const values = Array.from({ length: count }).map((_, i) =>
      bench.createValue(i)
    )
    for (const variant of bench.variants) {
      Bun.gc(true)

      const t0 = Bun.nanoseconds()
      for (const value of values) variant.schema.parse(value)
      const ns = Bun.nanoseconds() - t0

      this.table(
        c.white(variant.name),
        `${c.green((ns / count).toFixed(0))} ${c.gray('ns/iter')}`
      )
    }
  }

  private table(left: string, right: string, bottom = '') {
    console.log(
      new Table({
        chars: {
          top: '',
          'top-mid': '',
          'top-left': '',
          'top-right': '',
          bottom: bottom,
          'bottom-mid': '',
          'bottom-left': '',
          'bottom-right': '',
          left: '',
          'left-mid': '',
          mid: '',
          'mid-mid': '',
          right: '',
          'right-mid': '',
          middle: ' ',
        },
        head: [left, right],
        colWidths: [40, 30],
        colAligns: ['left', 'right'],
      }).toString()
    )
  }
}
