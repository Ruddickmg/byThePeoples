const merge = require('webpack-merge');
const baseConfig = require('./webpack.common');
const path = require('path');
const publicPath = '../../src/public';

module.exports = merge(baseConfig, {
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
  resolve: {
    alias: {
      'vue$': 'vue/dist/vue.runtime.js',
    },
  },
});