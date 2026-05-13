import type { Config } from "tailwindcss";
import typography from "@tailwindcss/typography";

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
      typography: ({ theme }: { theme: (path: string) => string }) => ({
        DEFAULT: {
          css: {
            color: theme('colors.skyglass.muted'),
            a: {
              color: theme('colors.skyglass.blue'),
              '&:hover': {
                color: theme('colors.skyglass.deep'),
              },
            },
            h1: {
              color: theme('colors.skyglass.ink'),
            },
            h2: {
              color: theme('colors.skyglass.ink'),
            },
            h3: {
              color: theme('colors.skyglass.ink'),
            },
            h4: {
              color: theme('colors.skyglass.ink'),
            },
            strong: {
              color: theme('colors.skyglass.ink'),
            },
            code: {
              color: theme('colors.skyglass.blue'),
              backgroundColor: theme('colors.navy.900'),
              padding: '0.25rem',
              borderRadius: '0.25rem',
              fontWeight: '400',
            },
            'code::before': {
              content: '""',
            },
            'code::after': {
              content: '""',
            },
            pre: {
              backgroundColor: theme('colors.navy.900'),
              color: theme('colors.skyglass.mist'),
            },
            blockquote: {
              borderLeftColor: theme('colors.skyglass.blue'),
              color: theme('colors.skyglass.muted'),
            },
          },
        },
        skyglass: {
          css: {
            '--tw-prose-body': theme('colors.silver.dim'),
            '--tw-prose-headings': theme('colors.skyglass.ink'),
            '--tw-prose-lead': theme('colors.silver.dim'),
            '--tw-prose-links': theme('colors.navy.700'),
            '--tw-prose-bold': theme('colors.skyglass.ink'),
            '--tw-prose-counters': theme('colors.silver.muted'),
            '--tw-prose-bullets': theme('colors.silver.muted'),
            '--tw-prose-hr': 'rgba(106,176,227,0.28)',
            '--tw-prose-quotes': theme('colors.skyglass.ink'),
            '--tw-prose-quote-borders': 'rgba(106,176,227,0.34)',
            '--tw-prose-captions': theme('colors.silver.muted'),
            '--tw-prose-code': theme('colors.navy.700'),
            '--tw-prose-pre-code': theme('colors.skyglass.mist'),
            '--tw-prose-pre-bg': theme('colors.navy.900'),
            '--tw-prose-th-borders': 'rgba(106,176,227,0.28)',
            '--tw-prose-td-borders': 'rgba(106,176,227,0.18)',
            a: {
              textDecoration: 'none',
              fontWeight: '600',
              '&:hover': { color: theme('colors.navy.600') },
            },
            code: {
              backgroundColor: 'rgba(234,246,255,0.9)',
              border: '1px solid rgba(106,176,227,0.22)',
              borderRadius: '0.375rem',
              padding: '0.15rem 0.35rem',
              fontWeight: '500',
            },
            'code::before': { content: '""' },
            'code::after': { content: '""' },
            pre: {
              borderRadius: '0.75rem',
              border: '1px solid rgba(106,176,227,0.24)',
            },
            blockquote: {
              borderLeftWidth: '3px',
              fontStyle: 'normal',
            },
            h1: { letterSpacing: '-0.02em' },
            h2: { letterSpacing: '-0.02em' },
          },
        },
      }),
    },
  },
  plugins: [
    typography,
  ],
};

export default config;
