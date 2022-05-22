import { createMemoryHistory, createRouter } from 'vue-router'
import MyPacks from '@/pages/MyPacks.vue'
import Settings from '@/pages/Settings.vue'

const routes = [
  {
    name: "MyPacks",
    path: '/',
    component: MyPacks,
  },
  {
    name: "BrowsePacks",
    path: '/browse',
    component: () => import(/* webpackChunkName: 'browse-packs' */ '@/pages/BrowsePacks.vue')
  },
  {
    name: "Settings",
    path: '/settings',
    component: Settings
  },
  {
    name: "About",
    path: '/about',
    component: () => import(/* webpackChunkName: 'about' */ '@/pages/About.vue')
  }
]

const router = createRouter({
  history: createMemoryHistory(process.env.BASE_URL),
  routes,
  linkExactActiveClass: "is-active"
})

export default router
