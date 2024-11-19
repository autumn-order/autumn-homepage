import daisyui from "daisyui";

/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  daisyui: {
    themes: [
      {
        "autumn-light": {
          primary: "#9a3412",
          secondary: "#7f1d1d",
          accent: "#1c1917",
          neutral: "#000000",
          "base-100": "#ffffff",
          info: "#0ea5e9",
          success: "#22c55e",
          warning: "#eab308",
          error: "#ef4444",
        },
      },
    ],
  },
  plugins: [daisyui],
};
