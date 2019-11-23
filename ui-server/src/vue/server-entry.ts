import { Context, createApp } from './main';
import Vue from 'vue';
import { NOT_FOUND } from '../constants/errorCodes';

export const serverEntryPoint = (context: Context): Promise<Vue> => new Promise(async (
  resolve,
  reject,
): Promise<void> => {
  const { app, router, store } = createApp(context);
  await router.push(context.url);
  await router.onReady(async (): Promise<void> => {
    context.rendered && context.rendered((): void => {
      context.state = store.state;
    });
    const matchedComponents = router.getMatchedComponents();
    if (!matchedComponents.length) {
      return reject({ code: NOT_FOUND });
    }
    resolve(app)
  }, reject);
});

export default serverEntryPoint;