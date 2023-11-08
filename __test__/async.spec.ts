import crypto from 'node:crypto'
import fs from 'node:fs/promises'

import ava, { type TestFn } from 'ava'
import temp from 'fs-temp'

import * as xattr from '../index.js'

const attribute0 = 'user.linusu.test'
const attribute1 = 'user.linusu.secondary'
const payload0 = crypto.randomBytes(24).toString('hex')
const payload1 = crypto.randomBytes(24).toString('hex')

const test = ava as TestFn<{
  path: string
}>

test.beforeEach((t) => {
  t.context.path = temp.writeFileSync('')
  xattr.setAttributeSync(t.context.path, attribute0, payload0)
  xattr.setAttributeSync(t.context.path, attribute1, payload1)
})

test.afterEach(async (t) => {
  const { path } = t.context
  await fs.unlink(path)
})

test('should set an attribute', async (t) => {
  const {
    context: { path },
  } = t
  await t.notThrowsAsync(async () => {
    await xattr.setAttribute(path, attribute0, payload0)
    await xattr.setAttribute(path, attribute1, payload1)
  })
})

test('should get an attribute', async (t) => {
  const {
    context: { path },
  } = t
  const val = await xattr.getAttribute(path, attribute0)
  t.true(Buffer.isBuffer(val))
  t.is(val!.toString(), payload0)
})

test('should list the attributes', async (t) => {
  const {
    context: { path },
  } = t
  const val = await xattr.listAttributes(path)
  t.true(val.includes(attribute0))
  t.true(val.includes(attribute1))
})

test('should remove the attribute', async (t) => {
  const {
    context: { path },
  } = t
  await t.notThrowsAsync(async () => {
    await xattr.removeAttribute(path, attribute0)
    await xattr.removeAttribute(path, attribute1)
  })
  t.is(await xattr.getAttribute(path, attribute0), null)
  t.is(await xattr.getAttribute(path, attribute1), null)
})
