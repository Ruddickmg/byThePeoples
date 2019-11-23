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
  devServer: {
    proxy: {
      '*': 'http://localhost:8080',
    },
    compress: true,
    historyApiFallback: true,
    writeToDisk:true,
    inline: true,
    hot: true,
    open: true,
    overlay: true,
    port: 8000,
    stats: {
      normal: true
    }
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