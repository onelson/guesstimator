const { createProxyMiddleware } = require('http-proxy-middleware');

module.exports = function(app) {
  app.use(
    '/gql',
    createProxyMiddleware({
          target: 'http://localhost:7878',
          ws: true,
    })
  );
};

