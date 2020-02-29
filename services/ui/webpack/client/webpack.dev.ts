import { Configuration } from 'webpack';
const merge = require('webpack-merge');
const baseConfig = require('./webpack.common');

export const developmentClientConfig: Configuration = merge(baseConfig, {
  mode: 'development',
  devtool: 'source-map',
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
  resolve: {
    alias: {
      'vue$': 'vue/dist/vue.runtime.js',
    },
  },
});

export default developmentClientConfig;
