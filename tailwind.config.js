/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./docs/**/*.html"],
  theme: {
    extend: {
      colors: {
        dxorange: "#E96020",
        dxblue: "#00A8D6",
        ghmetal: "#24292f",
        ghdarkmetal: "#161b22",
        ideblack: "#0e1116",
      },
      fontFamily: {
        sans: ["Inter var", "sans-serif"],
      },
      boxShadow: {
        "3xl": "0 35px 60px -1ww5px rgba(0, 0, 0, 0.5)",
        cutesy: "0px 0px 40px -5px rgba(255, 255, 255, 0.2)",
        pop: "0px 0px 10px -2px rgba(0, 0, 0, 0.1)",
      },
      keyframes: {
        fadein: {
          from: { opacity: "0" },
          to: { opacity: "1" },
        },
      },
      animation: {
        "fadein-medium": "fadein 500ms ease-in-out forwards",
      },
    },
  },
  plugins: [],
};
