module.exports = {
  future: {
    removeDeprecatedGapUtilities: true,
    purgeLayersByDefault: true,
  },
  purge: {
    content: [
      './public/index.html',
      './styles/main.css',
      './src/**/*.{ts,tsx,js,jsx}',
    ],
  },
  theme: {
    extend: {
      colors: {
        brand: {
          'pale-blue': 'var(--color-brand-pale-blue)',
          blue: 'var(--color-brand-blue)',
          'dark-blue': 'var(--color-brand-dark-blue)',
          red: 'var(--color-brand-red)',
          'bright-red': 'var(--color-brand-bright-red)',
        },
      },
    },
  },
};
