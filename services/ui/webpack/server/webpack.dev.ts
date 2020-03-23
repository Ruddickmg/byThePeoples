import { Configuration } from 'webpack';
const merge = require('webpack-merge');
const baseConfig = require('./webpack.common');

export const developmentServerConfig: Configuration = merge(baseConfig, {
  mode: 'development',
  devtool: 'source-map',
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
  resolve: {
    alias: {
      'vue$': 'vue/dist/vue.runtime.js',
    },
  },
});

export default developmentServerConfig;
