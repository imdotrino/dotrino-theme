/**
 * @dotrino/theme — elegir claro, oscuro o seguir al sistema.
 *
 * La hoja (`theme.css`) ya sigue al sistema sola. Esto es para la app que deja al
 * usuario elegir: fija `data-theme` en <html> y guarda la elección. Es una preferencia
 * de interfaz, así que va en localStorage (CONVENCIONES §4), no en el store.
 */

/** Para el manifest y el <meta name="theme-color"> de cada app. */
export const THEME_COLOR = '#00658c'
export const BACKGROUND_COLOR = { light: '#f7fafc', dark: '#0f1416' }

export const MODES = ['auto', 'light', 'dark']
const KEY = 'dotrino-theme'

/** La elección guardada; sin elección (o guardada mal), `auto`. */
export function getTheme () {
  try {
    const v = localStorage.getItem(KEY)
    return MODES.includes(v) ? v : 'auto'
  } catch {
    return 'auto'   // modo privado o almacenamiento bloqueado: se sigue al sistema
  }
}

/** Aplica `mode` ahora y lo recuerda. Lanza `theme-bad-mode` si no es uno de MODES. */
export function setTheme (mode) {
  if (!MODES.includes(mode)) {
    const e = new Error(`theme: unknown mode "${mode}"`)
    e.code = 'theme-bad-mode'
    throw e
  }
  const root = document.documentElement
  if (mode === 'auto') root.removeAttribute('data-theme')
  else root.setAttribute('data-theme', mode)
  try {
    if (mode === 'auto') localStorage.removeItem(KEY)
    else localStorage.setItem(KEY, mode)
  } catch { /* sin almacenamiento, vale solo para esta carga */ }
  return mode
}

/** Al arrancar: aplica lo guardado, antes del primer pintado si se llama en <head>. */
export function applyTheme () {
  return setTheme(getTheme())
}

/** Lo que se está viendo: `light` o `dark`, resuelto `auto` contra el sistema. */
export function resolvedTheme () {
  const fixed = document.documentElement.getAttribute('data-theme')
  if (fixed === 'light' || fixed === 'dark') return fixed
  return matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
}
