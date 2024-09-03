import * as fs from 'fs';
import blake2b from 'blake2b';
import WebSocket from 'ws';
import { bech32 } from 'bech32';
import {
  NATIVE_SCRIPTS,
  OGMIOS_HOST,
  REFERENCE_SCRIPTS,
  SINCE,
  UNTIL,
  VALIDATORS,
} from './config.mjs';

const KIND = {
  AIKEN: "aiken",
  PLUTARCH: "plutarch",
  OPSHIN: "opshin",
  PLUTUSTX: "plutus-tx",
  PLUTS: "plu-ts",
  HELIOS: "helios",
  MARLOWE: "marlowe",
  NATIVE: "native",
};

const LOGO = {
  [KIND.AIKEN]: "https://raw.githubusercontent.com/aiken-lang/site/main/public/cardano-smart-contract-frameworks/aiken.png",
  [KIND.PLUTARCH]: "https://raw.githubusercontent.com/aiken-lang/site/main/public/cardano-smart-contract-frameworks/plutarch.png",
  [KIND.OPSHIN]: "https://raw.githubusercontent.com/aiken-lang/site/main/public/cardano-smart-contract-frameworks/opshin.png",
  [KIND.PLUTUSTX]: "https://raw.githubusercontent.com/aiken-lang/site/main/public/cardano-smart-contract-frameworks/plutus-tx.png",
  [KIND.PLUTS]: "https://raw.githubusercontent.com/aiken-lang/site/main/public/cardano-smart-contract-frameworks/plu-ts.png",
  [KIND.HELIOS]: "https://raw.githubusercontent.com/aiken-lang/site/main/public/cardano-smart-contract-frameworks/helios.png",
  [KIND.MARLOWE]: "https://raw.githubusercontent.com/aiken-lang/site/main/public/cardano-smart-contract-frameworks/marlowe.png",
};

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
  client.rpc('findIntersection', { points: [SINCE] });
});

let unknowns = new Set();

let interactions = {
  [KIND.AIKEN]: 0,
  [KIND.PLUTARCH]: 0,
  [KIND.OPSHIN]: 0,
  [KIND.HELIOS]: 0,
  [KIND.MARLOWE]: 0,
  [KIND.PLUTUSTX]: 0,
  [KIND.PLUTS]: 0,
  [KIND.NATIVE]: 0,
  total: 0,
};

let transactions = {
  ...interactions
};

let timeline = {
  [KIND.AIKEN]: [],
  [KIND.PLUTARCH]: [],
  [KIND.OPSHIN]: [],
  [KIND.HELIOS]: [],
  [KIND.MARLOWE]: [],
  [KIND.PLUTUSTX]: [],
  [KIND.PLUTS]: [],
  [KIND.NATIVE]: [],
  epochs: [],
};

let previousInteractions = { total: 0 };

let previousTransactions = { total: 0 };

let previousEpoch = null;

client.once('message', (data) => {
  const tip = (UNTIL || JSON.parse(data).result.tip).slot;

  client.on('message', (data) => {
    const result = JSON.parse(data).result;

    if (result.direction === 'forward') {
      if (previousEpoch === null) {
        previousEpoch = getEpoch(result.block.slot);
      }

      result.block.transactions.forEach(tx => {
        // Show some progress at regular intervals
        if (transactions.total % 5000 === 0 && transactions.total > previousTransactions.total) {
          notify(result.block);
        }

        const hasCollateral = (tx.collaterals || []).length > 0;

        if (hasCollateral || tx.outputs.some(isScriptAddress)) {
          let sources = { ...(tx.mint || {}), ...(tx.scripts || {}), ...(tx.withdrawals || {}) };

          const before = { ...interactions };

          countWitnesses(sources, interactions);
          countUsage(tx.outputs, interactions);
          countReferences(tx.references, interactions);

          markTransaction(before, interactions, transactions);
        }
      });

      if (result.block.slot >= tip) {
        previousEpoch = Number.NEGATIVE_INFINITY;
        notify(result.block);
        fs.writeFileSync(`./unknowns-${SINCE.slot}:${tip}.json`, JSON.stringify(Array.from(unknowns), null, 2));
        console.log(`${unknowns.size} unknown script hashes?`);
        console.log('\n');
        const epochs = (new Array(timeline.epochs.length)).fill(0).map((_, ix) => ix + getEpoch(SINCE.slot));
        console.log(['framework', 'logo'].concat(epochs).join(', '));
        for (let lang in timeline) {
          if (lang === 'epochs' || lang === 'native') { continue; }
          console.log([lang, LOGO[lang]].concat(timeline[lang]).join(', '));
        }
        process.exit(0);
      }
    }

    client.rpc('nextBlock');
  });

  function notify({ slot }) {
    delete transactions['_'];

    const currentEpoch = getEpoch(slot)
    if (currentEpoch > previousEpoch) {
      previousEpoch = currentEpoch;

      for (let lang in transactions) {
        if (lang === 'total') { continue; }
        timeline[lang].push(transactions[lang] - (previousTransactions[lang] ?? 0));
      }

      timeline.epochs.push(currentEpoch);

      previousTransactions = { ...transactions };
      previousInteractions = { ...interactions };
    }

    transactions['_'] = transactions.total - countFilter(transactions, k => k != 'total');

    const total = transactions.total - transactions[KIND.NATIVE];

    const p = (source) => Math.round(10000 * source / total) / 100;

    const display = (kind, title) => {
      return {
        score: transactions[kind],
        text: [
          title.padEnd(10, ' '),
          String(transactions[kind]).padStart(12, ' '),
          `(${p(transactions[kind]).toFixed(2).padStart(5, ' ')}%)`,
        ].join(' '),
      };
    };

    const summary = [
      display(KIND.PLUTUSTX, 'Plutus-tx:'),
      display(KIND.PLUTARCH, 'Plutarch:'),
      display(KIND.AIKEN, 'Aiken:'),
      display(KIND.HELIOS, 'Helios:'),
      display(KIND.MARLOWE, 'Marlowe:'),
      display(KIND.OPSHIN, 'OpShin:'),
      display(KIND.PLUTS, 'Plu-ts:'),
    ] .sort((a, b) => b.score - a.score)
      .concat([
        display('_', 'Unsure:'),
        { text: ''.padStart(32, '=') },
        { text: `Total:${String(total).padStart(17, ' ')}` },
      ])
      .map(({ text }) => text)
      .join('\n  ');

    const completion = Math.floor(16 * (slot - SINCE.slot) / (tip - SINCE.slot));
    const progress = '>'.padStart(completion + 1, '=').padEnd(17, ' ');

    console.log(`(at ${slot}) [${progress}]\n${''.padStart(22, ' ')}TRANSACTIONS\n  ${''.padStart(32, '=')}\n  ${summary}\n\n`);
  }

  // Fill in the initial queue to leverage pipelining.
  for (let i = 0; i < 100; i += 1) { client.rpc('nextBlock'); }
});

function isScriptAddress({ address }) {
  return withShelleyAddress(address, false, (bytes) => {
    const addrType = bytes.slice(0, 1).toString('hex')[0];
    return addrType === '1' || addrType === '2' || addrType === '3' || addrType === '5' || addrType === '7' || addrType === '15';
  });
}

function isNativeScript({ address }) {
  return withShelleyAddress(address, false, (bytes) => {
    const payment_part = bytes.slice(1, 29).toString('hex');
    const delegation_part = bytes.slice(29).toString('hex');
    return NATIVE_SCRIPTS.has(payment_part) || NATIVE_SCRIPTS.has(delegation_part);
  });
}

function countWitnesses(scripts = {}, n) {
  return Object.keys(scripts).reduce((acc, k) => {
    const h = k.startsWith("stake")
      ? Buffer.from(bech32.fromWords(bech32.decode(k, 999).words)).toString('hex').substr(2)
      : k;

    const found = VALIDATORS.get(h);

    if (found) {
      acc[found] += 1;
    } else if (NATIVE_SCRIPTS.has(h)) {
      acc[KIND.NATIVE] += 1;
    } else {
      unknowns.add(h);
    }

    acc.total += 1;

    return acc;
  }, n);
}

function countReferences(references = [], n) {
  return references.reduce((acc, { transaction, index }) => {
    const ref = REFERENCE_SCRIPTS.get(`${transaction.id}#${index}`);
    const found = VALIDATORS.get(ref);

    if (found) {
      acc[found] += 1;
    } else if (ref && NATIVE_SCRIPTS.has(ref)) {
      acc[KIND.NATIVE] += 1;
    } else {
      unknowns.add(ref);
    }

    acc.total += 1;

    return acc;
  }, n);
}

function countUsage(outs, n) {
  return outs.reduce((acc, { address, script }) => {
    withShelleyAddress(address, false, (bytes) => {
      const addrType = bytes.slice(0, 1).toString('hex')[0];

      const payment_part = bytes.slice(1, 29).toString('hex');
      const payment_part_kind = VALIDATORS.get(payment_part);
      if (payment_part_kind) {
        acc[payment_part_kind] += 1;
        acc.total += 1;
      } else if (NATIVE_SCRIPTS.has(payment_part)) {
        acc.total += 1;
        acc[KIND.NATIVE] += 1;
      } else if (addrType === '1' || addrType === '3' || addrType === '5' || addrType === '7' || addrType === '15') {
        acc.total += 1;
        unknowns.add(payment_part);
      }

      const delegation_part = bytes.slice(29).toString('hex');
      const delegation_part_kind = VALIDATORS.get(delegation_part);
      if (delegation_part_kind) {
        acc[delegation_part_kind] += 1;
        acc.total += 1;
      } else if (NATIVE_SCRIPTS.has(delegation_part)) {
        acc.total += 1;
        acc[KIND.NATIVE] += 1;
      } else if (addrType === '2' || addrType === '3') {
        acc.total += 1;
        unknowns.add(delegation_part);
      }

      const inline_script = script ? blake2b(28).update(preimage(script)).digest('hex') : null;
      const inline_script_kind = VALIDATORS.get(inline_script);
      if (inline_script_kind) {
        acc[inline_script_kind] += 1;
        acc.total += 1;
      } else if (NATIVE_SCRIPTS.has(inline_script)) {
        acc.total += 1;
        acc[KIND.NATIVE] += 1;
      } else if (inline_script) {
        acc.total += 1;
        unknowns.add(inline_script);
      }
    });

    return acc;
  }, n);
}

function markTransaction(before, after, n) {
  Object.keys(KIND).forEach(k => {
    const delta = after[KIND[k]] - before[KIND[k]];
    if (delta > 0) {
      n.total += 1;
      n[KIND[k]] += 1;
    }
  });
}

function preimage({ language, cbor }) {
  switch (language) {
    case "plutus:v1":
      return Buffer.concat([Buffer.from([1]), Buffer.from(cbor, 'hex')]);
    case "plutus:v2":
      return Buffer.concat([Buffer.from([2]), Buffer.from(cbor, 'hex')]);
    case "plutus:v3":
      return Buffer.concat([Buffer.from([3]), Buffer.from(cbor, 'hex')]);
    default:
      return Buffer.from("");
  }
}

// Small helper to deal with intermittent Byron addresses that aren't
// bech32-encoded and can't take part in smart-contract transactions anyway.
function withShelleyAddress(address, empty, cb) {
  if (!address.startsWith('addr')) { return empty; }
  return cb(Buffer.from(bech32.fromWords(bech32.decode(address, 999).words)));
}

function countFilter(obj, predicate) {
  return Object.keys(obj).reduce((acc, k) => {
    if (!predicate(k)) {
      return acc;
    }

    return acc + obj[k];
  }, 0);
}

function getEpoch(slot) {
  const FIRST_SHELLEY_EPOCH = 208;
  const FIRST_SHELLEY_SLOT = 4492800;
  const EPOCH_LENGTH = 432000;
  return FIRST_SHELLEY_EPOCH + Math.floor((slot - FIRST_SHELLEY_SLOT) / EPOCH_LENGTH);
}
