import { Configuration } from 'webpack';
const merge = require('webpack-merge');
const HotModuleReplacement = require('webpack-hot-middleware');
const baseConfig = require('./webpack.common');
const path = require('path');
const publicPath = '../../src/public';

export const developmentServerConfig: Configuration = merge(baseConfig, {
  mode: 'development',
  devtool: 'source-map',
  output: {
    path: path.resolve(__dirname, `${publicPath}/scripts`),
    publicPath:'/public',
    filename: 'main.js',
  },
  module: {
    rules: [
      {
        test: /\.(css|scss|sass)$/,
        options: {
          sourceMap: true,
        }
      }
    ]
  },
  plugins: [
    new HotModuleReplacement(),
  ],
  resolve: {
    alias: {
      'vue$': 'vue/dist/vue.runtime.js',
    },
  },
});

export default developmentServerConfig;
