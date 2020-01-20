import merge from 'webpack-merge';
import OptimizeCSSAssetsPlugin from 'optimize-css-assets-webpack-plugin';
import { Configuration } from 'webpack';
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const baseConfig = require('./webpack.common');

export const productionServerConfig: Configuration = merge(baseConfig, {
  mode: 'production',
  optimization: {
    splitChunks: {
      cacheGroups: {
        commons: {
          test: /[\\/]node_modules[\\/]/,
          name: "vendor",
          chunks: "all",
        },
      },
    },
  },
  module: {
    rules:  [
      {
        test: /\.vue$/,
        loader: 'vue-loader',
        options: {
          extractCSS: true
        }
      },
    ],
  },
  resolve: {
    alias: {
      'vue$': 'vue/dist/vue.runtime.min.js',
    },
  },
  plugins: [
    new OptimizeCSSAssetsPlugin({
      cssProcessorPluginOptions: {
        preset: [
          'default',
          { discardComments: { removeAll: true } }
        ],
      }
    }),
    new MiniCssExtractPlugin({
      filename: 'common.[chunkhash].css',
    }),
  ],
});

export default productionServerConfig;
