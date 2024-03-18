import colors from 'tailwindcss/colors';

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  darkMode: "class",
  theme: {
    colors: {
      "gifsc": "#32A041",
      "rifsc": "#C8191E",
      ...colors
    }
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
  plugins: [
    "cssnano"
  ],
}