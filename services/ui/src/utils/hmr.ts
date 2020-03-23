// import fs from 'fs';
// import path from 'path';
// import webpack from 'webpack';
// import chokidar from 'chokidar';
// import { fs as memoryFileSystem } from 'memfs';
// import webpackDevMiddleware from 'webpack-dev-middleware';
// import webpackHotMiddleware from 'webpack-hot-middleware';
// import { developmentClientConfig } from '../../webpack/client/webpack.dev';
// import { developmentServerConfig } from '../../webpack/server/webpack.dev';
// import {
//   CLIENT_MANIFEST,
//   FINISHED_COMPILING,
//   HEART_BEAT,
//   SERVER_BUNDLE,
// } from '../constants/hmr';
// import { NextFunction, Request, Response } from 'express';
//
// const clientOutput = developmentClientConfig.output || {};
// const serverOutput = developmentServerConfig.output || {};
//
// developmentClientConfig.entry = ['webpack-hot-middleware/client', ...(developmentClientConfig.entry as string[])];
// developmentClientConfig.output = {
//   ...clientOutput,
//   filename: '[name].js',
// };
//
// const readFile = (fs: any, file: string) => {
//   try {
//     return JSON.parse(fs.readFileSync(path.join(clientOutput.path as string, file), 'utf-8'));
//   } catch (e) {
//     console.error(e);
//   }
// };
//
// const updateManifest = (middleWare: any, renderer: any): any => ({ errors, warnings, ...stats }: any): any => {
//   console.log('stats', stats);
//   errors.forEach(console.error);
//   warnings.forEach(console.warn);
//   if (!errors.length) {
//     try {
//       renderer({
//         clientManifest: readFile(middleWare.fileSystem, CLIENT_MANIFEST),
//       });
//     } catch (e) {
//       console.error(e);
//     }
//   }
// };
//
// const updateServerBundle = (fileSystem: any, update: any): any => (error: Error, { errors }: any): void => {
//    if (error) {
//      throw error;
//    }
//    if (!errors.length) {
//      try {
//        update({
//          bundle: readFile(fileSystem, SERVER_BUNDLE),
//        });
//      } catch (e) {
//        console.error(e);
//      }
//    }
// };
//
// const updateTemplate = (path: string, renderer: any): any => (): void => {
//   try {
//     renderer({
//       template: fs.readFileSync(path, 'utf-8'),
//     });
//   } catch (e) {
//     console.error(e);
//   }
// };
//
// export const hotModuleReplacement = (templatePath: string, renderer: any) => {
//   const clientCompiler = webpack(developmentClientConfig);
//   const serverCompiler = webpack(developmentServerConfig);
//   const devMiddleware = webpackDevMiddleware(clientCompiler);
//   const hotMiddleware = webpackHotMiddleware(clientCompiler, {
//     ...serverOutput,
//     heartbeat: HEART_BEAT,
//   });
//   chokidar.watch(templatePath).on('change', updateTemplate(templatePath, renderer));
//   serverCompiler.fileSystem = memoryFileSystem;
//   serverCompiler.watch({}, updateServerBundle(devMiddleware, renderer));
//   clientCompiler.hooks.afterCompile.tap(FINISHED_COMPILING, updateManifest(clientCompiler, renderer));
//   return (req: Request, res: Response, next: NextFunction): void => {
//     hotMiddleware(req, res, next);
//     devMiddleware(req, res, next);
//   };
// };
//
// export default hotModuleReplacement;
