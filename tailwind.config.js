module.exports = (env, argv) => {
  return {
    purge: {
      enabled: argv.mode === 'production',
      content: [
        './static/**/*.html',
        './src/**/*.rs',
        './bootstrap.js',
      ],
    },
    theme: {
      extend: {
        colors: {
          background: '#f5f5f5',
        },
      },
    },
    variants: {},
    plugins: [],
  }
}
