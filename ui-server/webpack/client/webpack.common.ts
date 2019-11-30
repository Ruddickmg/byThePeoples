import { Configuration } from 'webpack';
const VueSSRClientPlugin = require('vue-server-renderer/client-plugin');
const merge = require('webpack-merge');
const baseConfig = require('../webpack.config');

export const commonClientConfig: Configuration = merge(baseConfig, {
  entry:  ['./src/vue/client-entry.ts'],
  optimization: {
    splitChunks: {
      name: "manifest",
      minChunks: Infinity
    },
  },
  plugins: [
    new VueSSRClientPlugin()
  ],
});

export default commonClientConfig;
