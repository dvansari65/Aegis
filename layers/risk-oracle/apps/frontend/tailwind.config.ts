import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        skyglass: {
          white: "#FFFFFF",
          mist: "#EAF6FF",
          ice: "#C1E5FF",
          blue: "#9CD5FF",
          deep: "#6AB0E3",
          ink: "#102A3A",
          muted: "#557487",
        },
        navy: {
          900: "#102A3A",
          800: "#17394D",
          700: "#245B7F",
          600: "#3B86B8",
          500: "#6AB0E3",
        },
        silver: {
          DEFAULT: "#102A3A",
          dim: "rgba(16,42,58,0.72)",
          muted: "rgba(16,42,58,0.48)",
        },
        lime: {
          DEFAULT: "#6AB0E3",
          dim: "#3B86B8",
          soft: "rgba(156,213,255,0.32)",
          border: "rgba(106,176,227,0.34)",
        },
        status: {
          ok: "#3B86B8",
          warn: "#D89A2B",
          error: "#ef4444",
        },
        cream: {
          DEFAULT: "#102A3A",
          dim: "rgba(16,42,58,0.72)",
          muted: "rgba(16,42,58,0.48)",
        },
        blue: {
          300: "#9CD5FF",
          500: "#6AB0E3",
          700: "#3B86B8",
          900: "#102A3A",
        },
      },
      fontFamily: {
        sans: ["var(--font-inter)", "Inter", "Segoe UI", "Helvetica Neue", "Arial", "sans-serif"],
      },
    },
  },
  plugins: [],
};

export default config;
