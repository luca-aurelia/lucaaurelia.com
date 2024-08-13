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
        cerulean: "rgba(169, 232, 252, 1)", // Sky blue.
        cloud: "rgba(245, 245, 245, 0)", // Fully transparent white.
        twilight: "rgba(96, 76, 126, 1)", // A dusky purple.
        midnight: "hsl(270.35deg 21.52% 14.21%)", // An even darker, less saturated purple.
        // midnight: 'rgba(38,38,38,0)' // Fully transparent black.
      },
    },
  },

  plugins: [],
};

module.exports = config;
