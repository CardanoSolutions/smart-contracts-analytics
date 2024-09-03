import * as fs from 'fs';
import * as points from './config/points.mjs';

export const SINCE = points.?;

export const UNTIL = null;

export const OGMIOS_HOST = 'ws://127.0.0.1:1337';

export const NATIVE_SCRIPTS = new Set(JSON.parse(fs.readFileSync(
  `./data/native_scripts.json`
)));

export const VALIDATORS = new Map(JSON.parse(fs.readFileSync(
  `./data/validators.json`
)));

export const REFERENCE_SCRIPTS = new Map(JSON.parse(fs.readFileSync(
  `./data/reference_scripts.json`
)));
