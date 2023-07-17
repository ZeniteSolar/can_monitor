import { createApp } from "vue"
import App from "./App.vue"
import "./styles.css"
const app = createApp(App);

import { vuetify } from './plugins/vuetify'
app.use(vuetify);

app.mount("#app");
