import { createApp } from "vue";
import "./styles.css";
import '@mdi/font/css/materialdesignicons.css'
import App from "./App.vue";

import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

const vuetify = createVuetify({
    theme: {
        defaultTheme: 'dark'
    },
    components,
    directives,
})

createApp(App).use(vuetify).mount("#app");
