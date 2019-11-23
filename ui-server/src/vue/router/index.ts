import Router from 'vue-router'

const login = () => import('../components/login/index.vue');
const home = () => import('../components/home/index.vue');

export function createRouter (): Router {
  return new Router({
    mode: 'history',
    routes: [
      { path: '/', component: home, }
    ],
  } as any)
}