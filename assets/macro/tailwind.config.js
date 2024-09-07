const path = require("path");

// We were running into some funny business with relative paths which is why we use
// __dirname here.
const anyHtmlRustOrCssFileInTheWorkspace = path.resolve(
  __dirname,
  "..",
  "..",
  "**",
  "*.{html,rs,css}",
);

const content = [
  anyHtmlRustOrCssFileInTheWorkspace,

  // Exclude any files in a `target` directory
  "!**/target/**/*",
];

console.log(`Using ${content} as the content path.`);

/** @type {import('tailwindcss').Config}*/
const config = {
  content,

  theme: {
    extend: {
      fontSize: {},
      spacing: {},
      colors: {
        // midnight: 'rgba(38,38,38,0)' // Fully transparent black.
      },
    },
  },

  plugins: [],
};

module.exports = config;
