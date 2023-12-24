import './assets/finished.css'

import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import PrimeVue from 'primevue/config';
import Button from "primevue/button";
import Image from "primevue/image";
import TabMenu from 'primevue/tabmenu';
import Card from 'primevue/card';
import Divider from 'primevue/divider';
import 'primeicons/primeicons.css'
import Tailwind from 'primevue/passthrough/tailwind';
import { usePassThrough } from "primevue/passthrough";
const app = createApp(App)


app.component('Button', Button);
app.component('TabMenu', TabMenu);
app.component('Image',Image);
app.component('Card',Card);
app.component('Divider',Divider);
app.use(router)
const theme = usePassThrough(
    Tailwind,
    {
        tabmenu: {
            class: ['flex'],
            menu: {
                class: ['text-black'],
                
            },
            icon: {
                class: ['text-black']
            },
            label: {
                class: ['text-black']
            },
            action: {
                style: {
                    'background-color': 'transparent'
                }
            }
        },
        card: {
            root: {
                class: ['bg-white','shadow-2xl','border-solid border-4 slate-950','rounded-md']
            },
            subtitle: {
                class: ["text-black"]
            }
        }
    },
)
app.use(PrimeVue,{pt: theme});

app.mount('#app')
