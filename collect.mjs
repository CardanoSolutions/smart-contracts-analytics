import * as fs from 'fs';
import WebSocket from 'ws';
import blake2b from 'blake2b';

const NETWORK = process.env['NETWORK'];
const OGMIOS_HOST = process.env['OGMIOS_HOST'];
const POINTS = JSON.parse(fs.readFileSync('config/starting-points.json'));

const client = new WebSocket(OGMIOS_HOST);

// Strawman JSON-RPC 2.0 client
client.rpc = function rpc(method, params = {}) {
  this.send(JSON.stringify({
    jsonrpc: '2.0',
    method,
    params,
  }));
};

client.on('open', () => {
  const point = POINTS[NETWORK.toLowerCase()];

  if (!point) {
    throw new Error(`No configuration for NETWORK: ${NETWORK}`);
  }

  // Intersection points correspond roughly to the beginning of April when
  // Aiken's alpha release came out.
  client.rpc('findIntersection', { points: [point] });
});

client.once('message', (data) => {
  let store = new Set();

  const tip = JSON.parse(data).result.tip.slot;

  client.on('message', (data) => {
    const result = JSON.parse(data).result;

    if (result.direction === 'forward') {
      result.block.transactions.forEach(tx => {
        // Gather scripts from outputs & witnesses
        const scripts = tx.outputs.reduce((acc, out) => {
          if (out.script && out.script.language === 'plutus:v2') {
            const msg = Buffer.concat([
              Buffer.from([2]),
              Buffer.from(out.script.cbor, 'hex'),
            ]);
            const digest = blake2b(28).update(msg).digest('hex');
            acc[digest] = out.script;
          }
          return acc;
        }, tx.scripts || {});

        // Print out possible candidates, with their digest.
        Object.keys(scripts).forEach(digest => {
          if (store.has(digest)) { return; }
          store.add(digest);
          const { language, cbor } = scripts[digest];
          if (language === 'plutus:v2') {
            console.log(`${digest},${cbor}`);
          }
        });
      });

      if (result.block.slot >= tip) {
        process.exit(0);
      }

      client.rpc('nextBlock');
    }
  });

  // Fill in the initial queue to leverage pipelining.
  for (let i = 0; i < 100; i += 1) { client.rpc('nextBlock'); }
});
