import { createApp } from 'vue'
import App from './App.vue'
import { createRouter, createWebHashHistory } from 'vue-router'
import Main from './views/Main.vue'
import Settings from './views/Settings.vue'
import './assets/css/style.css'
import './assets/css/scroll.css'

const routes = [
  { path: '/', component: Main },
  { path: '/settings', component: Settings },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

createApp(App).use(router).mount('#app')
