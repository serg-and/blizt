import native from './native'

export type ParseSafe<T> = { success: boolean } & (
  | { success: true; data: T }
  | { success: false; reason: string }
)

export const {
  BUndefined,
  BNull,
  BBoolean,
  BNumber,
  BString,
  BArray,
  BObject,
  BUnion,
  BTuple,
} = native

type BUndefined<T> = native.BUndefined<T>
type BNull<T> = native.BNull<T>
type BBoolean<T> = native.BBoolean<T>
type BNumber<T> = native.BNumber<T>
type BString<T> = native.BString<T>
type BArray<T> = native.BArray<T>
type BObject<T> = native.BObject<T>
type BUnion<T> = native.BUnion<T>
type BTuple<T> = native.BTuple<T>

export type BValue<T = unknown> =
  | BUndefined<T>
  | BNull<T>
  | BBoolean<T>
  | BNumber<T>
  | BString<T>
  | BArray<T>
  | BObject<T>
  | BUnion<T>
  | BTuple<T>

/**
 * Infer the `parse()` return type of a BValue schema
 */
export type InferParse<T extends BValue> = ReturnType<T['parse']>

/**
 * Infer the `parseSafe()` return type of a BValue schema
 */
export type InferParseSafe<T extends BValue> = ParseSafe<InferParse<T>>

type ArrayElement<ArrayType extends readonly unknown[]> =
  ArrayType extends readonly (infer ElementType)[] ? ElementType : never

type InferBObjectParseType<T extends Record<string, BValue>> = {
  [key in keyof T]: InferParse<T[key]>
}

type InferBTupleParseType<T extends readonly BValue[]> = {
  [key in keyof T]: InferParse<T[key]>
}

const undefined = native.BUndefined.default
const Null = native.BNull.default
const boolean = native.BBoolean.default
const number = native.BNumber.default
const string = native.BString.default
const array = <T extends BValue>(schema: T) =>
  native.BArray._fromWrapped(schema._toWrapped()) as BArray<InferParse<T>[]>
const object = <T extends Record<string, BValue>>(schema: T) =>
  native.BObject.new(
    Object.fromEntries(
      Object.entries(schema).map(([k, v]): [string, native.BWrapped] => [
        k,
        v._toWrapped(),
      ])
    )
  ) as BObject<InferBObjectParseType<T>>
const union = <T extends readonly BValue[]>(...schemas: T) =>
  native.BUnion._fromWrapped(
    schemas.map(schema => schema._toWrapped())
  ) as BUnion<InferParse<ArrayElement<T>>>
const tuple = <T extends readonly BValue[]>(...schemas: T) =>
  native.BTuple._fromWrapped(
    schemas.map(schema => schema._toWrapped())
  ) as BTuple<InferBTupleParseType<T>>

export default {
  undefined,
  Null,
  boolean,
  number,
  string,
  array,
  object,
  union,
  tuple,
}

// type NativeBValue =
//   | native.BUndefined
//   | native.BNull
//   | native.BBoolean
//   | native.BNumber
//   | native.BString
//   | native.BArray
//   | native.BObject
//   | native.BUnion

// type OverwriteMethods<T, Overwrite> = Omit<T, keyof Overwrite> & Overwrite

// // Correct all special method definitions
// type CorrectMethods<T extends NativeBValue, R> = OverwriteMethods<
//   T,
//   {
//     parse(value: unknown): R
//     parseSafe(value: unknown): ParseSafe<R>
//     optional(): CorrectMethods<T, R | undefined>
//     nullable(): CorrectMethods<T, R | null>
//     nullish(): CorrectMethods<T, R | null | undefined>
//     required(): CorrectMethods<T, Exclude<R, undefined>>
//     nonNullable(): CorrectMethods<T, Exclude<R, null>>
//   }
// >

// // Replaces return types of methods that return an instance of the native type with the corrected type
// // type CorrectReturnTypes<T, NativeValue extends NativeBValue> = {
// //   [key in keyof T]: T[key] extends (...args: any) => NativeValue
// //     ? (...args: Parameters<T[key]>) => T
// //     : T[key]
// // }

// // type CorrectBValueType<T extends NativeBValue, R> = CorrectReturnTypes<
// //   CorrectMethods<T, R>,
// //   T
// // >

// // export type BUndefined = CorrectBValueType<native.BUndefined, undefined>
// // export type BNull = CorrectBValueType<native.BNull, null>
// // export type BBoolean = CorrectBValueType<native.BBoolean, boolean>
// // export type BNumber = CorrectBValueType<native.BNumber, number>
// // export type BString = CorrectBValueType<native.BString, string>
// // export type BArray<T> = CorrectBValueType<native.BArray, T[]>
// // export type BUnion<T> = CorrectBValueType<native.BUnion, T>
// // export type BObject<T> = CorrectBValueType<native.BObject, T>

// export type BUndefined = CorrectMethods<native.BUndefined, undefined>
// export type BNull = CorrectMethods<native.BNull, null>
// export type BBoolean = CorrectMethods<native.BBoolean, boolean>
// export type BNumber = CorrectMethods<native.BNumber, number>
// export type BString = CorrectMethods<native.BString, string>
// export type BArray<T> = CorrectMethods<native.BArray, T[]>
// export type BObject<T> = CorrectMethods<
//   OverwriteMethods<
//     native.BObject,
//     {
//       merge<O>(object: BObject<O>): BObject<Omit<T, keyof O> & O>
//     }
//   >,
//   T
// >

// export type BUnion<T> = OverwriteMethods<
//   CorrectMethods<native.BUnion, T>,
//   { merge<U>(union: BUnion<U>): BUnion<T | U> }
// >

// export type BValue =
//   | BUndefined
//   | BNull
//   | BBoolean
//   | BNumber
//   | BString
//   | BArray<unknown>
//   | BObject<unknown>
//   | BUnion<unknown>

// export type ParseSafe<T> = { success: boolean } & (
//   | { success: true; data: T }
//   | { success: false; reason: string }
// )

// /**
//  * Infer the `parse()` return type of a BValue schema
//  */
// export type InferParse<T extends BValue> = ReturnType<T['parse']>

// /**
//  * Infer the `parseSafe()` return type of a BValue schema
//  */
// export type InferParseSafe<T extends BValue> = ParseSafe<InferParse<T>>

// type ArrayElement<ArrayType extends readonly unknown[]> =
//   ArrayType extends readonly (infer ElementType)[] ? ElementType : never

// type InferBObjectParseType<T extends Record<string, BValue>> = {
//   [key in keyof T]: InferParse<T[key]>
// }

// const undefined = native.BUndefined.default as () => BUndefined
// const Null = native.BNull.default as () => BNull
// const boolean = native.BBoolean.default as () => BBoolean
// const number = native.BNumber.default as () => BNumber
// const string = native.BString.default as () => BString
// const array = <T extends BValue>(schema: T) =>
//   native.BArray._fromWrapped(schema._toWrapped()) as BArray<InferParse<T>>
// const union = <T extends readonly BValue[]>(...schemas: T) =>
//   native.BUnion._fromWrapped(
//     schemas.map(schema => schema._toWrapped())
//   ) as BUnion<InferParse<ArrayElement<T>>>
// const object = <T extends Record<string, BValue>>(schema: T) =>
//   native.BObject.new(
//     Object.fromEntries(
//       Object.entries(schema).map(([k, v]): [string, native.BWrapped] => [
//         k,
//         v._toWrapped(),
//       ])
//     )
//   ) as BObject<InferBObjectParseType<T>>

// export default {
//   undefined,
//   Null,
//   boolean,
//   number,
//   string,
//   array,
//   union,
//   object,
// }
