module.exports = {
  purge: {
    mode: "all",
    content: [
      "../src/**/*.rs",
      "../index.html",
      "../src/**/*.html",
      "../src/**/*.css",
    ],
  },
  darkMode: 'class', // <= 'media' or 'class'
  theme: {},
  variants: {},
  plugins: [],
};
