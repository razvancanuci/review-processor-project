import { defineRouter } from '#q-app/wrappers';
import {
  createMemoryHistory,
  createRouter,
  createWebHistory,
} from 'vue-router';
import routes from './routes';
import { validateToken } from 'src/utils/validate'

/*
 * If not building with SSR mode, you can
 * directly export the Router instantiation;
 *
 * The function below can be async too; either use
 * async/await or return a Promise which resolves
 * with the Router instance.
 */

export default defineRouter(function (/* { store, ssrContext } */) {
  const createHistory = process.env.SERVER
    ? createMemoryHistory
    :createWebHistory;

  const Router = createRouter({
    scrollBehavior: () => ({ left: 0, top: 0 }),
    routes,

    // Leave this as is and make changes in quasar.conf.js instead!
    // quasar.conf.js -> build -> vueRouterMode
    // quasar.conf.js -> build -> publicPath
    history: createHistory(process.env.VUE_ROUTER_BASE),
  });

  Router.beforeEach(async (to, _, next) => {
    if (to.name === 'Login') {
      localStorage.removeItem('auth_tkn');
      next();
      return;
    }

    if (to.name === 'Home' && !await validateToken()) {
      next({name: 'Login'});
      return;
    }

    next();
  })
  return Router;
});
