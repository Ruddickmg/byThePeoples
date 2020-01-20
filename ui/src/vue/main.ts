import Vue from 'vue'
import { createRouter } from './router';
import { Store } from 'vuex';
import { createStore } from './store';
import { VueRouter } from 'vue-router/types/router';
import { sync } from 'vuex-router-sync';
import home from './components/home/index.vue';
import Router from 'vue-router';
import Vuex from 'vuex';

Vue.use(Vuex);
Vue.use(Router);

export interface Context {
  url: string;
  rendered?: (fn: () => void) => void;
  state?: any;
}

export interface App {
  app: Vue,
  router: VueRouter,
  store: Store<any>,
}

export const createApp = ({ url }: Context): App => {
  const router = createRouter();
  const store = createStore();

  sync(store, router);

  const app = new Vue({
    render: (h: any): any => h(home),
    template: '<h1>hello world</h1>',
    router,
    store,
    data: {
      url,
    },
  });
  return { app, router, store };
};