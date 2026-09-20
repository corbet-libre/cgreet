// This module also runs in a real browser with no Node shims.
export function verify(api) {
    const check = (actual, expected) => {
        if (JSON.stringify(actual) !== JSON.stringify(expected)) {
            throw new Error(`${JSON.stringify(actual)} !== ${JSON.stringify(expected)}`);
        }
    };
    check(api.deSalutation('Frau Dr. Müller', 'ch'), 'Sehr geehrte Frau Dr. Müller');
    check(api.parseRegion('CH'), null);
}
