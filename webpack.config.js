const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === 'production',
      port: process.env.PORT && parseInt(process.env.PORT, 10) || 8000,
    },
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: "todomvc.js",
      webassemblyModuleFilename: "todomvc.wasm"
    },
    module: {
      rules: [
        {
          test: /\.css$/i,
          use: [
            'style-loader',
            'css-loader',
            'postcss-loader'
          ],
        },
      ],
    },
    plugins: [
      new CopyWebpackPlugin([
        { from: './static', to: distPath }
      ]),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      })
    ],
    watch: argv.mode !== 'production'
  };
};
