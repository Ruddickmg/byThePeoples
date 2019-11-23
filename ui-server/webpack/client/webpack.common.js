const VueSSRClientPlugin = require('vue-server-renderer/client-plugin');
const merge = require('webpack-merge');
const baseConfig = require('../webpack.config');

module.exports = merge(baseConfig, {
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
