import type { RouteRecordRaw } from 'vue-router';
import LoginPage from 'pages/LoginPage.vue'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Home',
    component: () => import('pages/FormPage.vue')
  },
  {
    path:'/login',
    name:'Login',
    component: LoginPage
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/ErrorNotFound.vue'),
  },
];

export default routes;
