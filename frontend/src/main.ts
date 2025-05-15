import { createApp } from "vue"
import App from "./App.vue"

// Vuetify
import { createVuetify } from 'vuetify'
import { md3 } from 'vuetify/blueprints'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import { aliases, mdi } from 'vuetify/iconsets/mdi'
import 'vuetify/styles'

// helper to read CSS vars from :root in App.vue
function getVar(name: string): string { 
  return getComputedStyle(document.documentElement) 
    .getPropertyValue(name).trim(); 
} 

const ZeniteTheme = {
  dark: false,
  colors: {
    background: getVar('--zenite-background'),
    'on-background': getVar('--zenite-on-background'),
    surface: getVar('--zenite-surface'),
    primary: getVar('--zenite-primary'),
    'primary-darken-1': getVar('--zenite-primary-darken-1'),
    secondary: getVar('--zenite-secondary'),
    'secondary-darken-1': getVar('--zenite-secondary-darken-1'),
    terciary: getVar('--zenite-terciary'),
    error: getVar('--zenite-error'),
    info: getVar('--zenite-info'),
    success: getVar('--zenite-success'),
    warning: getVar('--zenite-warning'),
  }
} 

export const vuetify = createVuetify({
  components,
  directives,
  blueprint: md3,
  theme: {
    defaultTheme: 'ZeniteTheme',
    themes: {
      ZeniteTheme,
    },
  },
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: {
      mdi,
    },
  },
})

createApp(App).use(vuetify).mount('#app')
