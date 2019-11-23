import express, { Request, Response, Router } from 'express';
import fs from 'fs';
import { createBundleRenderer } from 'vue-server-renderer';
import clientManifest from './public/scripts/vue-ssr-client-manifest.json';
import serverBundle from './public/scripts/vue-ssr-server-bundle.json';
import { ALL_ROUTES, HOME_PAGE } from './constants/routes';
import { NOT_FOUND, SERVER_ERROR } from './constants/errorCodes';
import { PORT } from './constants/connections';
import { PRODUCTION } from './constants/environment';

const ui = Router();
const title = 'byThePeoples';
const mainPageTemplate = fs.readFileSync('src/templates/main.html');
const server = express();
const renderer = createBundleRenderer(serverBundle, {
  runInNewContext: false,
  template: mainPageTemplate.toString(),
  clientManifest,
});

const router = async (req: Request, res: Response): Promise<void> => {
  const context = { url: req.url, title };
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

server.use('/public', express.static(`${__dirname}/public/scripts`));
ui.get(ALL_ROUTES, router);
server.use(HOME_PAGE, ui);
server.listen(PORT, (): void => console.log(`listening @ port ${PORT}`));
