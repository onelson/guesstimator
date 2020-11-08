// In CI or during docker builds we need to rely on whatever the schema looks
// like on disk instead of trying to contact a live graphql endpoint.

module.exports = {
  client: {
    service: {
      name: 'phi',
      localSchemaFile: './schema.graphql',
    },
  },
};
