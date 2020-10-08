module.exports = {
  future: {
    removeDeprecatedGapUtilities: true,
    purgeLayersByDefault: true,
  },
  purge: {
    content: [
      './static/index.html',
      './static/style.css',
      './src/**/*.rs',
      './bootstrap.js',
    ],
  },
  theme: {
    extend: {
      colors: {
        brand: {
          'pale-blue':  'var(--color-brand-pale-blue)',
          'blue':  'var(--color-brand-blue)',
          'dark-blue':  'var(--color-brand-dark-blue)',
          'red':        'var(--color-brand-red)',
          'bright-red': 'var(--color-brand-bright-red)',
        }
      },
    },
  },
};
