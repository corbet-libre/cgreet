//! Deterministic locale-correct greeting and salutation helpers.
//!
//! `cgreet` owns every salutation rule: which honorifics and academic titles
//! render, how the surname is found, and which regions punctuate the German
//! salutation with a comma. The rules live as data in `tables/*.json` (see
//! `tables/README.md` for the schema and the normative token
//! normalization); `tests/vectors/*.json` is the executable contract every
//! language port runs. The Typst module in `typst/greet.typ` reads the
//! same canonical table; consumers share these helpers rather than mirror
//! salutation rules in their own renderers.
//!
//! Rules are sourced from the national correspondence norms (SN 010130 for
//! ch/li, DIN 5008 for de/at; at follows DIN since ÖNORM A 1080 was withdrawn
//! in 2018). Same input always yields the same output: no models, no I/O.

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

/// German correspondence table: honorifics, titles, filler tokens, comma
/// regions and the generic fallback, loaded from `tables/de.json` at
/// compile time so the library itself performs no I/O.
#[derive(serde::Deserialize)]
struct DeTable {
    honorifics: HashMap<String, String>,
    titles: HashMap<String, String>,
    sole_titles: Vec<String>,
    filler: HashSet<String>,
    comma_regions: Vec<String>,
    generic: String,
}

static DE: LazyLock<DeTable> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../tables/de.json")).expect("tables/de.json is valid")
});

/// Normalize one whitespace-separated token for table lookup: strip
/// leading/trailing `.`, lowercase ASCII A–Z only. Must stay in sync with
/// `tables/README.md` — every port implements this exact normalization.
fn norm(token: &str) -> String {
    token.trim_matches('.').to_ascii_lowercase()
}

/// German correspondence region. Only lowercase codes: ch, li, de, at.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Region {
    /// Switzerland (SN 010130): no comma after the salutation.
    Ch,
    /// Liechtenstein (renders Swiss-style): no comma after the salutation.
    Li,
    /// Germany (DIN 5008): trailing comma after the salutation.
    De,
    /// Austria (DIN-oriented): trailing comma after the salutation.
    At,
}

impl Region {
    /// Parse a lowercase region code. Returns `None` for anything else.
    #[must_use]
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "ch" => Some(Self::Ch),
            "li" => Some(Self::Li),
            "de" => Some(Self::De),
            "at" => Some(Self::At),
            _ => None,
        }
    }

    /// Lowercase region code, for table lookup.
    fn code(self) -> &'static str {
        match self {
            Self::Ch => "ch",
            Self::Li => "li",
            Self::De => "de",
            Self::At => "at",
        }
    }

    /// Whether the salutation carries a trailing comma (de/at only).
    #[must_use]
    pub fn uses_comma(self) -> bool {
        DE.comma_regions.iter().any(|region| region == self.code())
    }
}

/// Last whitespace-separated token of a recipient name for the salutation.
/// "Dr. Jane Doe" -> "Doe"; single-token and hyphenated names survive;
/// empty/whitespace yields "" for the generic greeting.
#[must_use]
pub fn salutation_last_name(name: &str) -> &str {
    name.split_whitespace().next_back().unwrap_or("")
}

/// Honorific of a recipient name for the German salutation: "frau" for
/// "Frau", "herr" for "Herr"/"Herrn", "" when unparsable. Abbreviations
/// ("Hr.", "Fr.") are rejected: the Anrede always uses "Herr" (never the
/// accusative "Herrn", which belongs only in the postal address) and never
/// abbreviates "Frau".
#[must_use]
pub fn salutation_honorific(name: &str) -> &'static str {
    let first = name.split_whitespace().next().unwrap_or("");
    DE.honorifics
        .get(norm(first).as_str())
        .map_or("", String::as_str)
}

/// Academic titles preserved in the German salutation: "Dr." stays
/// abbreviated, "Prof." normalises to the spelled-out "Professor",
/// "Dipl.-Ing." and "Mag." survive. Protocol keeps only the highest title,
/// so Professor suppresses Dr.
#[must_use]
pub fn salutation_titles(name: &str) -> Vec<&'static str> {
    let mut kept: Vec<&'static str> = Vec::new();
    for token in name.split_whitespace() {
        let Some(title) = DE.titles.get(norm(token).as_str()).map(String::as_str) else {
            continue;
        };
        if !kept.contains(&title) {
            kept.push(title);
        }
    }
    for title in &kept {
        if DE.sole_titles.iter().any(|sole| sole.as_str() == *title) {
            return vec![*title];
        }
    }
    kept
}

/// Surname for the German salutation: last significant token after dropping
/// the honorific, academic titles, and post-nominal grades.
#[must_use]
pub fn salutation_surname(name: &str) -> &str {
    name.split_whitespace()
        .rfind(|token| !DE.filler.contains(norm(token).as_str()))
        .unwrap_or("")
}

/// Locale-correct German salutation. Without a parsable honorific or surname
/// it falls back to the generic "Sehr geehrte Damen und Herren" so the letter
/// stays formally safe.
#[must_use]
pub fn de_salutation(name: &str, region: Region) -> String {
    let punct = if region.uses_comma() { "," } else { "" };
    let honorific = salutation_honorific(name);
    let surname = salutation_surname(name);
    if honorific.is_empty() || surname.is_empty() {
        return format!("{}{punct}", DE.generic);
    }
    let titles = salutation_titles(name);
    let title_part = if titles.is_empty() {
        String::new()
    } else {
        format!(" {}", titles.join(" "))
    };
    if honorific == "frau" {
        format!("Sehr geehrte Frau{title_part} {surname}{punct}")
    } else {
        format!("Sehr geehrter Herr{title_part} {surname}{punct}")
    }
}

/// Non-blocking advisory when the recipient name is missing: the letter
/// still renders with the generic salutation, but a tailored opportunity
/// should name a person. Returns `None` when a last name is available.
#[must_use]
pub fn recipient_salutation_warning(location: &str, name: &str) -> Option<String> {
    if salutation_last_name(name).is_empty() {
        Some(format!(
            "{location}: job.cl_recipient.name is empty; using generic salutation (provide a name for tailored opportunities)"
        ))
    } else {
        None
    }
}

/// Non-blocking advisory for German records whose recipient name carries no
/// parsable Herr/Frau honorific: the letter falls back to the generic
/// salutation, so a human should supply the full address form (e.g.
/// "Frau Dr. Müller"). Returns `None` for empty names (already covered by
/// [`recipient_salutation_warning`]) and for complete names.
#[must_use]
pub fn de_honorific_warning(location: &str, name: &str) -> Option<String> {
    if salutation_last_name(name).is_empty() {
        return None;
    }
    if salutation_honorific(name).is_empty() || salutation_surname(name).is_empty() {
        Some(format!(
            "{location}: job.cl_recipient.name has no parsable Herr/Frau honorific; using generic salutation (provide e.g. \"Frau Dr. Müller\" for tailored opportunities)"
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tables_schema() {
        // Every key in the table is already normalized, so lookups agree
        // across languages implementing the documented normalization.
        for key in DE.honorifics.keys() {
            assert_eq!(key, &norm(key), "honorific key not normalized: {key:?}");
        }
        for value in DE.honorifics.values() {
            assert!(
                value == "frau" || value == "herr",
                "honorific value must be frau/herr: {value:?}"
            );
        }
        for key in DE.titles.keys() {
            assert_eq!(key, &norm(key), "title key not normalized: {key:?}");
        }
        for value in DE.titles.values() {
            assert!(!value.is_empty(), "title display form must not be empty");
        }
        for sole in &DE.sole_titles {
            assert!(
                DE.titles.values().any(|display| display == sole),
                "sole title must be a known display form: {sole:?}"
            );
        }
        for token in &DE.filler {
            assert_eq!(
                token,
                &norm(token),
                "filler token not normalized: {token:?}"
            );
        }
        for region in &DE.comma_regions {
            assert!(
                Region::parse(region).is_some(),
                "comma region must be a known code: {region:?}"
            );
        }
        assert!(!DE.generic.is_empty(), "generic fallback must not be empty");
    }

    fn run_vector(file: &std::path::Path, vector: &serde_json::Value) {
        let name = vector["name"].as_str().unwrap_or("<unnamed>");
        let context = format!("{} :: {name}", file.display());
        let input = vector["input"].as_str().expect("vector needs input");
        let actual: serde_json::Value = match vector["fn"].as_str().unwrap_or("") {
            "region_parse" => Region::parse(input).map_or(serde_json::Value::Null, |region| {
                serde_json::Value::String(region.code().to_owned())
            }),
            "region_uses_comma" => {
                let region = Region::parse(input).expect("uses_comma vector needs a known region");
                serde_json::Value::Bool(region.uses_comma())
            }
            "salutation_last_name" => {
                serde_json::Value::String(salutation_last_name(input).to_owned())
            }
            "salutation_honorific" => {
                serde_json::Value::String(salutation_honorific(input).to_owned())
            }
            "salutation_surname" => serde_json::Value::String(salutation_surname(input).to_owned()),
            "salutation_titles" => serde_json::Value::Array(
                salutation_titles(input)
                    .iter()
                    .map(|title| serde_json::Value::String((*title).to_owned()))
                    .collect(),
            ),
            "de_salutation" => {
                let code = vector["region"]
                    .as_str()
                    .expect("de_salutation vector needs region");
                let region =
                    Region::parse(code).expect("de_salutation vector needs a known region");
                serde_json::Value::String(de_salutation(input, region))
            }
            "recipient_salutation_warning" => {
                let location = vector["location"]
                    .as_str()
                    .expect("warning vector needs location");
                recipient_salutation_warning(location, input)
                    .map_or(serde_json::Value::Null, serde_json::Value::String)
            }
            "de_honorific_warning" => {
                let location = vector["location"]
                    .as_str()
                    .expect("warning vector needs location");
                de_honorific_warning(location, input)
                    .map_or(serde_json::Value::Null, serde_json::Value::String)
            }
            other => panic!("{context}: unknown fn {other:?}"),
        };
        let expected = vector
            .get("expected")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        assert_eq!(actual, expected, "{context}");
    }

    #[test]
    fn token_normalization_strips_dots_and_folds_ascii_only() {
        assert_eq!(norm("...Dr..."), "dr");
        assert_eq!(norm("Frau."), "frau");
        assert_eq!(norm("FRAU"), "frau");
        assert_eq!(norm("MÜLLER"), "mÜller");
        assert!(Region::parse("De").is_none());
        assert!(Region::parse("ch ").is_none());
    }

    #[test]
    fn conformance_vectors() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/vectors");
        let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&dir)
            .expect("tests/vectors exists")
            .map(|entry| entry.expect("readable entry").path())
            .collect();
        files.sort();
        assert!(!files.is_empty(), "no vector files in tests/vectors");
        let mut count = 0;
        for file in &files {
            let raw = std::fs::read_to_string(file).expect("vector file is readable");
            let vectors: Vec<serde_json::Value> =
                serde_json::from_str(&raw).expect("vector file is valid JSON");
            for vector in &vectors {
                run_vector(file, vector);
                count += 1;
            }
        }
        assert!(count > 0, "no vectors ran");
    }
}
