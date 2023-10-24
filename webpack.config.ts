import * as webpack from "webpack";
import * as path from "path";
import HtmlWebpackPlugin from "html-webpack-plugin";
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";

const config: webpack.Configuration = {
  target: "web",
  entry: "./index.ts",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bundle.js",
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: "ts-loader",
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({template: "./index.html"}),
    new WasmPackPlugin({crateDirectory: path.resolve(__dirname, "app")}),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
  resolve: {
    extensions: ['.ts', '.js'],
  },
  mode: "production",
}

export default config;
