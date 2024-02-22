import * as fs from 'fs';
import * as points from './config/points.mjs';
import blake2b from 'blake2b';
import WebSocket from 'ws';

/******************************************************************************
 *
 * CONFIGURATION
 *
 *****************************************************************************/

const since = points.aikenAlphaLaunch;
const until = null;
const OGMIOS_HOST = process.env['OGMIOS_HOST'];
const NATIVE_SCRIPTS = new Set(JSON.parse(fs.readFileSync(`./data/native_scripts.json`)));

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

  let n = null;
  let m = null;

  const tip = (until || JSON.parse(data).result.tip).slot;

  client.on('message', (data) => {
    const result = JSON.parse(data).result;

    if (result.direction === 'forward') {
      n += 1;

      if (n % 50000 === 0 && n > m) {
        const progress = 100 * (result.block.slot - since.slot) / (tip - since.slot);
        console.error(`At slot ${result.block.slot} (${progress.toFixed(2)}%)`);
        m = n;
      }

      result.block.transactions.forEach(tx => {
        // Gather scripts from outputs & witnesses
        const scripts = tx.outputs.reduce((acc, out) => {
          if (out.script && out.script.language !== 'native') {
            const tag = Number.parseInt(out.script.language.slice(-1), 10);
            const msg = Buffer.concat([
              Buffer.from([tag]),
              Buffer.from(out.script.cbor, 'hex'),
            ]);
            const digest = blake2b(28).update(msg).digest('hex');
            acc[digest] = out.script;
          } else if (out.script && out.script.language === 'native') {
            const msg = Buffer.concat([
              Buffer.from([0]),
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
          if (language !== 'native') {
            console.log(`${digest},${cbor}`);
          } else {
            console.error(`"${digest}",`);
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
