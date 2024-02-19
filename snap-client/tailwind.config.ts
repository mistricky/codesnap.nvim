import type { Config } from "tailwindcss";

export default {
  content: ["./src/**/*.{js,jsx,ts,tsx}"],
  plugins: [require("daisyui"), require("@tailwindcss/typography")],
  daisyui: {
    themes: ["dark"],
  },
  theme: {
    fontFamily: {
      caskaydiacove: "CaskaydiaCove Nerd Font",
    },
    extend: {
      colors: {
        "one-dark-base": "#282C34",
        "border-color": "#545F64",
      },
    },
  },
} satisfies Config;
