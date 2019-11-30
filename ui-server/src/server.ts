import express, { Request, Response, Router } from 'express';
import fs from 'fs';
import { BundleRenderer, createBundleRenderer } from 'vue-server-renderer';
import clientManifest from './public/scripts/vue-ssr-client-manifest.json';
import serverBundle from './public/scripts/vue-ssr-server-bundle.json';
import { ALL_ROUTES, HOME_PAGE } from './constants/routes';
import { NOT_FOUND, SERVER_ERROR } from './constants/errorCodes';
import { PORT } from './constants/connections';
import { PRODUCTION } from './constants/environment';
import { hotModuleReplacement } from './utils/hmr';

const templatePath = 'src/templates/main.html';

const ui = Router();
const title = 'byThePeoples';
const server = express();

interface RenderUpdates {
  [property: string]: any,
  template?: any;
  clientManifest?: any;
  bundle?: any;
}

const { updateData, createRenderer } = (() => {
  const mainPageTemplate = fs.readFileSync(templatePath);
  let renderData: RenderUpdates = {
    template: mainPageTemplate.toString(),
    bundle: serverBundle,
    clientManifest,
  };
  return {
    updateData(update: RenderUpdates) {
      renderData = {
        ...renderData,
        ...update,
      }
    },
    createRenderer() {
      const { bundle, ...data } = renderData;
      return createBundleRenderer(bundle, {
        runInNewContext: false,
        ...data,
      })
    },
  };
})();


const router = async (req: Request, res: Response): Promise<void> => {
  const context = { url: req.url, title };
  const renderer = createRenderer();
  try {
    res.end(await renderer.renderToString(context));
  } catch (error) {
    error.code === NOT_FOUND
      ? res.status(NOT_FOUND)
      : res.status(SERVER_ERROR);
    if (process.env.NODE_ENV !== PRODUCTION) {
      console.error(error);
      res.send({ ...error });
    } else {
      res.end();
    }
  }
};

server.use(hotModuleReplacement(templatePath, updateData));
server.use('/public', express.static(`${__dirname}/public/scripts`));
ui.get(ALL_ROUTES, router);
server.use(HOME_PAGE, ui);
server.listen(PORT, (): void => console.log(`listening @ port ${PORT}`));
