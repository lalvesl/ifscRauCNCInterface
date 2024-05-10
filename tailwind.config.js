import colors from "tailwindcss/colors";

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  darkMode: "class",
  theme: {
    colors: {
      gifsc: "#32A041",
      rifsc: "#C8191E",
      ifsc: {
        g: {
          1: "#23702D", //0.7
          2: "#288034", //0.8
          3: "#2A8537", //0.85
          4: "#32A041", //1
        },
      },
      ...colors,
    },
  },
  safelist: [
    "bg-gifsc",
    "text-gifsc",
    "bg-rifsc",
    "text-rifsc",
    "hover:bg-gifsc",
    "hover:text-gifsc",
    "hover:bg-rifsc",
    "hover:text-rifsc",
  ],
  plugins: ["cssnano"],
};
