# `@napi-rs/xattr`

![https://github.com/Brooooooklyn/xattr/actions](https://github.com/Brooooooklyn/xattr/workflows/CI/badge.svg)

> It provides support for manipulating extended attributes (xattrs) on modern Unix filesystems.

# Usage

```ts
export function getAttribute(path: string, name: string): Promise<Buffer | null>
export function getAttributeSync(path: string, name: string): Buffer | null
export function setAttribute(path: string, name: string, value: Buffer | string): Promise<void>
export function setAttributeSync(path: string, name: string, value: Buffer | string): void
export function removeAttributeSync(path: string, name: string): void
export function removeAttribute(path: string, name: string): Promise<void>
export function listAttributes(path: string): Promise<Array<string>>
export function listAttributesSync(path: string): Array<string>
```
