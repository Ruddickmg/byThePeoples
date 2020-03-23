import { Configuration } from 'webpack';
const VueLoaderPlugin = require('vue-loader/lib/plugin');
const nodeExternals = require('webpack-node-externals');
const path = require('path');

const publicPath = '../src/public';

export const baseConfig: Configuration = {
  entry: ['@babel/polyfill'],
  resolve: {
    extensions: ['.js', '.ts', '.vue', '.json'],
  },
  output: {
    path: path.resolve(__dirname, `${publicPath}/scripts`),
    libraryTarget: "commonjs2",
    publicPath:'/public',
    filename: 'main.js',
  },
  externals: nodeExternals({
    // do not externalize dependencies that need to be processed by webpack.
    // you can add more file types here e.g. raw *.vue files
    // you should also whitelist deps that modifies `global` (e.g. polyfills)
    whitelist: /\.css$/
  }),
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
          name: '[name].[ext]?[authentication]'
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
