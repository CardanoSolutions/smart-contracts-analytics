import WebSocket from 'ws';
import { bech32 } from 'bech32';
import * as fs from 'fs';

const NETWORK = process.env['NETWORK'];
const OGMIOS_HOST = process.env['OGMIOS_HOST'];
const POINTS = JSON.parse(fs.readFileSync('config/starting-points.json'));

const AIKEN_VALIDATORS = JSON
  .parse(fs.readFileSync(`./data/aiken_validators.json`))
  .reduce((obj, v) => { obj[v] = true; return obj }, {});

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
  let total = 0;
  let previousTotal = total;
  let n = 0;

  const tip = JSON.parse(data).result.tip.slot;

  client.on('message', (data) => {
    const result = JSON.parse(data).result;

    if (result.direction === 'forward') {
      result.block.transactions.forEach(tx => {
        // Show some progress at regular intervals
        if (total % 1000 === 0 && total > previousTotal) {
          previousTotal = total;
          console.log(`Total: ${total}, Aiken's: ${n} (at ${result.block.slot})`);
        }

        const hasCollateral = (tx.collaterals || []).length > 0;

        if (hasCollateral || tx.outputs.some(isScriptAddress)) {
          total += 1;
          if (hasAikenValidator(tx.mint) || hasAikenValidator(tx.scripts) || tx.outputs.some(useAikenValidator)) {
            n += 1;
          }
        }
      });

      if (result.block.slot >= tip) {
        console.log(`Total: ${total}, Aiken's: ${n} (at ${result.block.slot})`);
        process.exit(0);
      }
    }

    client.rpc('nextBlock');
  });


  // Fill in the initial queue to leverage pipelining.
  for (let i = 0; i < 100; i += 1) { client.rpc('nextBlock'); }
});

function isScriptAddress({ address }) {
  return withShelleyAddress(address, false, (bytes) => {
    const addrType = bytes.slice(0, 1).toString('hex')[0];
    return addrType === '1' || addrType === '3' || addrType === '5' || addrType === '7';
  });
}

function hasAikenValidator(scripts = {}) {
  return Object.keys(scripts).some(k => AIKEN_VALIDATORS[k]);
}

function useAikenValidator({ address }) {
  return withShelleyAddress(address, false, (bytes) => {
    const payment_part = bytes.slice(1, 29).toString('hex');
    const delegation_part = bytes.slice(29).toString('hex');
    return AIKEN_VALIDATORS[payment_part] || AIKEN_VALIDATORS[delegation_part];
  });
}

// Small helper to deal with intermittent Byron addresses that aren't
// bech32-encoded and can't take part in smart-contract transactions anyway.
function withShelleyAddress(address, empty, cb) {
  if (!address.startsWith('addr')) { return empty; }
  return cb(Buffer.from(bech32.fromWords(bech32.decode(address, 999).words)));
}
