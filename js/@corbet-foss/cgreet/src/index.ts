/**
 * Deterministic locale-correct greeting and salutation helpers.
 *
 * Pure TypeScript port of the cgreet Rust crate: zero dependencies, zero
 * Node APIs, synchronous, no I/O — runs in browsers, edge runtimes, and
 * Node alike. Behavior is defined by `tables/*.json` at the repository
 * root; `tests/vectors/*.json` is the shared conformance suite. A change
 * here without a matching vector is a bug.
 */
import { DE_TABLE } from './generated/tables.ts';

export type Region = 'ch' | 'li' | 'de' | 'at';
export type Honorific = 'frau' | 'herr' | '';

const REGIONS: readonly Region[] = ['ch', 'li', 'de', 'at'];

/**
 * Whitespace split matching Rust `split_whitespace` (Unicode White_Space)
 * exactly — JavaScript `\s` differs on U+0085 and U+FEFF, so the set is
 * explicit. See `tables/README.md`.
 */
const WS = new Set([
    '\t',
    '\n',
    '\x0B',
    '\x0C',
    '\r',
    ' ',
    '\u0085',
    '\u00A0',
    '\u1680',
    '\u2000',
    '\u2001',
    '\u2002',
    '\u2003',
    '\u2004',
    '\u2005',
    '\u2006',
    '\u2007',
    '\u2008',
    '\u2009',
    '\u200A',
    '\u2028',
    '\u2029',
    '\u202F',
    '\u205F',
    '\u3000',
]);

function splitWhitespace(text: string): string[] {
    const tokens: string[] = [];
    let current = '';
    for (const char of text) {
        if (WS.has(char)) {
            if (current !== '') {
                tokens.push(current);
                current = '';
            }
        } else {
            current += char;
        }
    }
    if (current !== '') tokens.push(current);
    return tokens;
}

/**
 * Normalize one token for table lookup: strip leading/trailing `.`,
 * lowercase ASCII A–Z only (never Unicode-aware lowercasing).
 */
function norm(token: string): string {
    return token.replace(/^\.+|\.+$/g, '').replace(/[A-Z]/g, (c) => c.toLowerCase());
}

const filler = new Set<string>(DE_TABLE.filler);
const commaRegions = new Set<string>(DE_TABLE.comma_regions);

/** Parse a lowercase region code. Returns `null` for anything else. */
export function parseRegion(code: string): Region | null {
    return (REGIONS as readonly string[]).includes(code) ? (code as Region) : null;
}

/** Whether the salutation carries a trailing comma (de/at only). */
export function regionUsesComma(region: Region): boolean {
    return commaRegions.has(region);
}

/**
 * Last whitespace-separated token of a recipient name for the salutation.
 * Empty/whitespace yields "" for the generic greeting.
 */
export function salutationLastName(name: string): string {
    const tokens = splitWhitespace(name);
    return tokens.length > 0 ? tokens[tokens.length - 1] : '';
}

/**
 * Honorific of a recipient name: "frau" for "Frau", "herr" for
 * "Herr"/"Herrn", "" when unparsable. Abbreviations ("Hr.", "Fr.") are
 * rejected.
 */
export function salutationHonorific(name: string): Honorific {
    const first = splitWhitespace(name)[0] ?? '';
    const hit = DE_TABLE.honorifics[norm(first)];
    return hit === 'frau' || hit === 'herr' ? hit : '';
}

/**
 * Academic titles preserved in the salutation. Protocol keeps only the
 * highest title, so Professor suppresses Dr.
 */
export function salutationTitles(name: string): string[] {
    const kept: string[] = [];
    for (const token of splitWhitespace(name)) {
        const title = DE_TABLE.titles[norm(token)];
        if (title !== undefined && !kept.includes(title)) kept.push(title);
    }
    for (const title of kept) {
        if (DE_TABLE.sole_titles.includes(title)) return [title];
    }
    return kept;
}

/**
 * Surname for the salutation: last significant token after dropping the
 * honorific, academic titles, and post-nominal grades.
 */
export function salutationSurname(name: string): string {
    const tokens = splitWhitespace(name);
    for (let index = tokens.length - 1; index >= 0; index--) {
        if (!filler.has(norm(tokens[index]))) return tokens[index];
    }
    return '';
}

/**
 * Locale-correct German salutation. Without a parsable honorific or surname
 * it falls back to the generic greeting so the letter stays formally safe.
 */
export function deSalutation(name: string, region: Region): string {
    const punct = regionUsesComma(region) ? ',' : '';
    const honorific = salutationHonorific(name);
    const surname = salutationSurname(name);
    if (honorific === '' || surname === '') return `${DE_TABLE.generic}${punct}`;
    const titles = salutationTitles(name);
    const titlePart = titles.length > 0 ? ` ${titles.join(' ')}` : '';
    if (honorific === 'frau') return `Sehr geehrte Frau${titlePart} ${surname}${punct}`;
    return `Sehr geehrter Herr${titlePart} ${surname}${punct}`;
}

/**
 * Non-blocking advisory when the recipient name is missing. Returns `null`
 * when a last name is available.
 */
export function recipientSalutationWarning(location: string, name: string): string | null {
    if (salutationLastName(name) === '') {
        return `${location}: job.cl_recipient.name is empty; using generic salutation (provide a name for tailored opportunities)`;
    }
    return null;
}

/**
 * Non-blocking advisory for German records with no parsable Herr/Frau
 * honorific. Returns `null` for empty names (covered by
 * `recipientSalutationWarning`) and for complete names.
 */
export function deHonorificWarning(location: string, name: string): string | null {
    if (salutationLastName(name) === '') return null;
    if (salutationHonorific(name) === '' || salutationSurname(name) === '') {
        return `${location}: job.cl_recipient.name has no parsable Herr/Frau honorific; using generic salutation (provide e.g. "Frau Dr. Müller" for tailored opportunities)`;
    }
    return null;
}
