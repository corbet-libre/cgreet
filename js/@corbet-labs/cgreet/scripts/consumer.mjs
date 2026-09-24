// Copy next to a fresh installation to test the actual npm tarball.
import { createRequire } from 'node:module';
import { verify } from './verify-api.mjs';
verify(await import('@corbet-labs/cgreet'));
verify(createRequire(import.meta.url)('@corbet-labs/cgreet'));
verify(await import('@corbet-labs/cgreet/browser'));
console.log('cgreet: installed ESM, CommonJS, and browser exports passed');
