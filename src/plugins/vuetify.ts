// Styles
import '@mdi/font/css/materialdesignicons.css'
import 'vuetify/styles'

import { createVuetify } from 'vuetify'
import { aliases, mdi } from 'vuetify/iconsets/mdi'


// ref: https://m2.material.io/design/color/dark-theme.html#ui-application
// https://material-theme.com/docs/reference/color-palette/
const deepOcean = {
  dark: true,
  colors: {
    background: '#0F111A',
    surface: '#181A1F',
    primary: '#ffcb6b',
    'primary-darken-1': '#f78c6c',
    secondary: '#89ddff',
    'secondary-darken-1': '#717CB4',
    error: '#ff5370',
    info: '#89ddff',
    success: '#c3e88d',
    warning: '#f78c6c',
  },
}


export const vuetify = createVuetify({
    theme: {
      defaultTheme: 'deepOcean',
      themes: {
        deepOcean,
      },
    },
    // https://pictogrammers.com/library/mdi/
    icons: {
      defaultSet: 'mdi',
      aliases,
      sets: {
        mdi,
      },
    },
  });