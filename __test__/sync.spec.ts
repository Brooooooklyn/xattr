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

test('should set an attribute', (t) => {
  const {
    context: { path },
  } = t
  t.notThrows(() => {
    xattr.setAttributeSync(path, attribute0, payload0)
    xattr.setAttributeSync(path, attribute1, payload1)
  })
})

test('should get an attribute', (t) => {
  const {
    context: { path },
  } = t
  const val = xattr.getAttributeSync(path, attribute0)
  t.true(Buffer.isBuffer(val))
  t.is(val!.toString(), payload0)
})

test('should list the attributes', (t) => {
  const {
    context: { path },
  } = t
  const val = xattr.listAttributesSync(path)
  t.true(val.includes(attribute0))
  t.true(val.includes(attribute1))
})

test('should remove the attribute', (t) => {
  const {
    context: { path },
  } = t
  t.notThrows(() => {
    xattr.removeAttributeSync(path, attribute0)
    xattr.removeAttributeSync(path, attribute1)
  })
  t.is(xattr.getAttributeSync(path, attribute0), null)
  t.is(xattr.getAttributeSync(path, attribute1), null)
})
