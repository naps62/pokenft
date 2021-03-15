const { override, addPostcssPlugins, addWebpackModuleRule } = require("customize-cra");

module.exports = override(
  addWebpackModuleRule({
    test: /\.mjs$/,
    include: /node_modules/,
    type: "javascript/auto"
  }),
  addPostcssPlugins([
        require('tailwindcss'),
        require('autoprefixer'),
  ])
);

// module.exports = function override(webpackConfig) {
//   webpackConfig.module.rules.push({
//     test: /\.mjs$/,
//     include: /node_modules/,
//     type: "javascript/auto"
//   });

//   webpackConfig.style = {
//     postcss: {
//       plugins: [
//         require('tailwindcss'),
//         require('autoprefixer'),
//       ]
//     }
//   };

//   return webpackConfig;
// }
