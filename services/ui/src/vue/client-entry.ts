import { createApp } from './main';
import { BROWSER_STORE } from '../constants/environment';

const { app, router, store } = createApp({ url: '/' });

if ((window as any)[BROWSER_STORE]) {
  store.replaceState((window as any)[BROWSER_STORE])
}

router.onReady(() => {
  app.$mount('#app');
});
