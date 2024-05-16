const Fastify = require('fastify');

const fastify = Fastify({
  logger: true
});

fastify.get('/health', async (request, reply) => {
    return { hello: 'world' }
});

const start = async () => {
    try {
        await fastify.listen(8080);
        console.log(`server listening on ${fastify.server.address().port}`)
    } catch (err) {
        console.error(err)
        process.exit(1)
    }
}

start()