const OptimizeCSSAssetsPlugin = require('optimize-css-assets-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const merge = require('webpack-merge');
const baseConfig = require('./webpack.common');
const path = require('path');
const publicPath = '../../dist/public';

export const productionClientConfig = merge(baseConfig, {
  mode: 'production',
  output: {
    path: path.resolve(__dirname, `${publicPath}/scripts`),
    publicPath:'/public',
    filename: 'main.js',
  },
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
    new MiniCssExtractPlugin({ filename: 'common.[chunkhash].css' }),
    new OptimizeCSSAssetsPlugin({
      cssProcessorPluginOptions: {
        preset: [
          'default',
          { discardComments: { removeAll: true } }
        ],
      }
    }),
  ],
});

export default productionClientConfig;
