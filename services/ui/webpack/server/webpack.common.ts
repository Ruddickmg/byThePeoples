import { Configuration } from 'webpack';

const nodeExternals = require('webpack-node-externals');
const VueSSRServerPlugin = require('vue-server-renderer/server-plugin');
const merge = require('webpack-merge');
const baseConfig = require('../webpack.config');

export const commonServerConfig: Configuration = merge(baseConfig, {
  entry: ['./src/vue/server-entry.ts'],
  output: {
    libraryTarget: 'commonjs2',
  },
  target: 'node',
  externals: nodeExternals({
    // do not externalize dependencies that need to be processed by webpack.
    // you can add more file types here e.g. raw *.vue files
    // you should also whitelist deps that modifies `global` (e.g. polyfills)
    whitelist: /\.css$/
  }),
  plugins: [
    new VueSSRServerPlugin(),
  ],
});

export default commonServerConfig;
