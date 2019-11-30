import { Configuration } from 'webpack';
const VueLoaderPlugin = require('vue-loader/lib/plugin');

export const baseConfig: Configuration = {
  entry: ['@babel/polyfill'],
  resolve: {
    extensions: ['.js', '.ts', '.vue', '.json'],
  },
  module: {
    rules: [
      {
        test: /\.(ts|tsx)?$/,
        loader: 'ts-loader',
        exclude: /node_modules/,
        options: {
          appendTsSuffixTo: [/\.vue$/],
        }
      },
      {
        test: /\.(js|jsx)?$/,
        loader: 'babel-loader',
        exclude: /node_modules/,
        options: {
          presets: ['@babel/env', '@babel/typescript']
        }
      },
      {
        test: /\.vue$/,
        loader: 'vue-loader',
        exclude: /node_modules/,
        // options: {
        //   cacheBusting: true,
        // },
      },
      {
        test: /\.(png|jpg|gif|svg)$/,
        loader: 'file-loader',
        options: {
          name: '[name].[ext]?[hash]'
        }
      },
      {
        test: /\.(css|scss|sass)$/,
        use: [
          'vue-style-loader',
          'css-loader',
          'sass-loader',
        ]
      }
    ]
  },
  plugins: [
    new VueLoaderPlugin(),
  ],
};

export default baseConfig;
