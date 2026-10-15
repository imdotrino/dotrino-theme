import { test } from 'node:test'
import assert from 'node:assert/strict'
import { readFileSync, existsSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'

const here = dirname(fileURLToPath(import.meta.url))
const css = readFileSync(join(here, '../src/theme.css'), 'utf8')

test('every var() used in the sheet is declared in it', () => {
  const declared = new Set([...css.matchAll(/^\s*(--[\w-]+)\s*:/gm)].map(m => m[1]))
  const used = new Set([...css.matchAll(/var\((--[\w-]+)/g)].map(m => m[1]))
  const missing = [...used].filter(v => !declared.has(v))
  assert.deepEqual(missing, [])
})

test('every color token carries a light and a dark value', () => {
  // Un token de color sin `light-dark()` se vería igual en los dos modos.
  const colors = [...css.matchAll(/^\s*(--dt-(?:bg|surface|line|text|accent|mint|violet|gold|busy|error)[\w-]*)\s*:\s*([^;]+);/gm)]
  assert.ok(colors.length > 20)
  for (const [, name, value] of colors) {
    const v = value.trim()
    if (v.startsWith('var(') || v.startsWith('color-mix(') || v.includes('var(--dt-shadow-color)')) continue
    assert.match(v, /^light-dark\(/, `${name} has a single value: ${value}`)
  }
})

test('the font files the sheet points to are in the package', () => {
  const files = [...css.matchAll(/url\('\.\.\/fonts\/([\w.-]+)'\)/g)].map(m => m[1])
  assert.equal(files.length, 4)
  for (const f of files) assert.ok(existsSync(join(here, '../fonts', f)), f)
  const pkg = JSON.parse(readFileSync(join(here, '../package.json'), 'utf8'))
  assert.ok(pkg.files.includes('fonts'))
})

test('setTheme writes data-theme and remembers it; auto clears both', async () => {
  const attrs = {}
  const stored = {}
  globalThis.document = { documentElement: {
    setAttribute: (k, v) => { attrs[k] = v },
    removeAttribute: (k) => { delete attrs[k] },
    getAttribute: (k) => attrs[k] ?? null
  } }
  globalThis.localStorage = {
    getItem: k => stored[k] ?? null,
    setItem: (k, v) => { stored[k] = v },
    removeItem: k => { delete stored[k] }
  }
  globalThis.matchMedia = () => ({ matches: true })
  const t = await import('../src/index.js')

  assert.equal(t.getTheme(), 'auto')
  t.setTheme('dark')
  assert.equal(attrs['data-theme'], 'dark')
  assert.equal(t.getTheme(), 'dark')
  assert.equal(t.resolvedTheme(), 'dark')
  t.setTheme('auto')
  assert.equal(attrs['data-theme'], undefined)
  assert.equal(stored['dotrino-theme'], undefined)
  assert.equal(t.resolvedTheme(), 'dark')   // lo dice el sistema (matchMedia)
  assert.throws(() => t.setTheme('sepia'), e => e.code === 'theme-bad-mode')
  stored['dotrino-theme'] = 'garbage'
  assert.equal(t.getTheme(), 'auto')
})
