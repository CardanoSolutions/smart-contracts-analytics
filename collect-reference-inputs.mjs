import * as fs from 'fs';
import WebSocket from 'ws';
import blake2b from 'blake2b';
import * as points from './config/points.mjs';

/******************************************************************************
 *
 * CONFIGURATION
 *
 *****************************************************************************/

const since = points.beginningOfBabbage;
const until = null;
const OGMIOS_HOST = process.env['OGMIOS_HOST'];

/*****************************************************************************/

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
  client.rpc('findIntersection', { points: [since] });
});

client.once('message', (data) => {
  let store = new Set();

  let n = 0;
  let m = null;
  let separator = '[';

  const tip = (until || JSON.parse(data).result.tip).slot;

  client.on('message', (data) => {
    const result = JSON.parse(data).result;

    if (result.direction === 'forward') {
      n += 1;

      if (n % 10000 === 0 && n > m) {
        const progress = 100 * (result.block.slot - since.slot) / (tip - since.slot);
        console.error(`At slot ${result.block.slot} (${progress.toFixed(2)}%)`);
        m = n;
      }

      result.block.transactions.forEach(tx => {
        tx.outputs.forEach((out, ix) => {
          if (out.script && out.script.language !== 'native') {
            const tag = Number.parseInt(out.script.language.slice(-1), 10);
            const msg = Buffer.concat([
              Buffer.from([tag]),
              Buffer.from(out.script.cbor, 'hex'),
            ]);
            const digest = blake2b(28).update(msg).digest('hex');
            console.log(`${separator} [ "${tx.id}#${ix}", "${digest}" ]\n`)
            separator = ','
          }
        });
      });

      if (result.block.slot >= tip) {
        console.log(separator === '[' ? '[]' : ']');
        process.exit(0);
      }

      client.rpc('nextBlock');
    }
  });

  // Fill in the initial queue to leverage pipelining.
  for (let i = 0; i < 100; i += 1) { client.rpc('nextBlock'); }
});
