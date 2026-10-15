# @dotrino/theme

El tema del ecosistema Dotrino, «Cool & Cozy», en una sola hoja: las tipografías
(Quicksand para el texto y Comfortaa para la marca, empaquetadas: nada se pide a Google
Fonts), los colores en claro y en oscuro, la geometría, unos estilos base y las
variables que leen los componentes compartidos (`<dotrino-topbar>`, `<dotrino-support>`,
`<dotrino-profile>`).

Toda app del ecosistema lo usa (CONVENCIONES §5). Así un cambio del tema es una versión
de este paquete y no una edición por app.

## Uso (Vite)

```bash
npm i @dotrino/theme
```

```js
// main.js, antes del CSS propio de la app
import '@dotrino/theme/theme.css'
```

Vite copia las fuentes al build. El CSS de la app usa los tokens:

```css
.card { background: var(--dt-surface); border: 1px solid var(--dt-line-soft); border-radius: var(--dt-radius); box-shadow: var(--dt-shadow); }
.btn.primary { background: var(--dt-accent); color: var(--dt-accent-ink); }
```

## Uso (sin build)

```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@dotrino/theme@0.1/src/theme.css">
```

## Claro y oscuro

La hoja sigue al sistema del usuario. Para fijarlo, `<html data-theme="light">` (o
`dark`). **El usuario elige con el sol/luna del topbar**: `<dotrino-topbar theme>`
(`@dotrino/topbar` ≥ 0.16.0), que ya usa esto por debajo. Para hacerlo a mano:

```js
import { applyTheme, setTheme, getTheme, resolvedTheme } from '@dotrino/theme'

applyTheme()        // al arrancar: aplica lo que eligió
setTheme('dark')    // 'auto' | 'light' | 'dark'; se recuerda en localStorage
```

Cada color se declara una vez con `light-dark()`, así que no hay dos bloques que se
desincronicen. Todos los pares de texto sobre fondo pasan 4.5:1 (WCAG AA).

**Al cambiar el esquema en caliente, Chromium puede dejar un elemento sin repintar**: el
valor calculado ya es el nuevo, pero en pantalla queda un color intermedio hasta que algo
le cambia el estilo (visto en iatosvg con la emulación de Playwright, 2026-10-14; una
carga limpia en cada modo sale bien). Si lo ves en una prueba, recarga antes de dar el
tema por roto. Y no pongas `transition` sobre un color `--dt-*`: no arregla nada y
alarga el intermedio.

## Tokens

| | |
|---|---|
| Superficies | `--dt-bg`, `--dt-bg-2`, `--dt-surface`, `--dt-surface-2`, `--dt-surface-3`, `--dt-line`, `--dt-line-soft`, `--dt-shadow` |
| Texto | `--dt-text`, `--dt-text-dim`, `--dt-text-faint` |
| Acento | `--dt-accent`, `--dt-accent-press`, `--dt-accent-ink`, `--dt-accent-soft`, `--dt-accent-soft-ink`, `--dt-accent-tint` |
| Secundarios | `--dt-mint`, `--dt-mint-soft`, `--dt-violet`, `--dt-violet-soft`, `--dt-gold` |
| Estado | `--dt-ok`, `--dt-busy`, `--dt-error`, `--dt-error-soft`, `--dt-error-ink` |
| Tipografía | `--dt-font`, `--dt-font-brand`, `--dt-font-mono` |
| Geometría | `--dt-radius-sm`, `--dt-radius`, `--dt-radius-lg`, `--dt-radius-pill` |

Para el manifest y el `<meta name="theme-color">`: `THEME_COLOR` (`#00658c`) y
`BACKGROUND_COLOR.light` / `.dark`.

## Desarrollo

```bash
npm test
```

Se publica desde CI con un tag `vX.Y.Z` (`release.yml`).
