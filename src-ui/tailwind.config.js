/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    colors: {
      primary: "#fff",
      accent: "#0071df",
      neutral: "#bdbdbd",
      error: "#de3341",
      "base-100": "#121212",
      "base-200": "#202020",
      "base-300": "#303030",
      "base-content": "#fff",
    },
    extend: {
      ringWidth: {
        10: "10px",
      },
      animation: {
        flash: "flash",
      },
      keyframes: {
        flash: {
          "0%": { scale: "90%" },
          "100%": { scale: "100%" },
        },
      },
    },
  },
  plugins: [],
  daisyui: {},
};
