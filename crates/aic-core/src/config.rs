//! Game configuration: which modules, which variant of each, and every
//! tunable parameter. A config is plain data (JSON); the engine never
//! hard-codes a tunable number.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::hash::fnv1a64;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// A label for humans. Not part of the config hash.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Number of countries (= seats = megacorps): any count with a layout in
    /// `content/map.json`, 1-4 today.
    pub countries: u8,
    /// Module instances, at most one per module id.
    pub modules: Vec<ModuleConfig>,
    /// The phase sequence of one round, as `"<module>.<phase>"`. Empty means
    /// every module's phases in module order, then phase-declaration order.
    #[serde(default)]
    pub round: Vec<String>,
    /// The content overlay: `<table>.<id>.<path>=<JSON>` entries, applied in
    /// order to the embedded content files (see [`crate::content::overlaid`]).
    /// Part of the config, so a record replays with the content it ran and
    /// the config hash names it. An experiment lever, not a rules option.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub content: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RawModuleConfig")]
pub struct ModuleConfig {
    pub module: String,
    pub variant: String,
    /// Every parameter the variant declares must be given; unknown names
    /// are errors.
    pub params: BTreeMap<String, ParamValue>,
    /// Card ids this module uses, for modules that play cards.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cards: Vec<String>,
}

/// A module config as read, its parameters still JSON values, so a bad value
/// is reported with its key (022: serde's untagged-enum error named none).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawModuleConfig {
    module: String,
    variant: String,
    #[serde(default)]
    params: BTreeMap<String, serde_json::Value>,
    #[serde(default)]
    cards: Vec<String>,
}

impl TryFrom<RawModuleConfig> for ModuleConfig {
    type Error = String;

    fn try_from(r: RawModuleConfig) -> Result<Self, String> {
        let params = r
            .params
            .iter()
            .map(|(k, v)| {
                let p = ParamValue::from_json(v).map_err(|e| format!("{}.{k}: {e}", r.module))?;
                Ok((k.clone(), p))
            })
            .collect::<Result<_, String>>()?;
        Ok(ModuleConfig {
            module: r.module,
            variant: r.variant,
            params,
            cards: r.cards,
        })
    }
}

/// A tunable value. Integers only: every number must be printable on a card
/// or a board track. A list is a table (by difficulty, stage,
/// board, seat, country or site kind) or a list of thresholds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum ParamValue {
    Int(i64),
    List(Vec<i64>),
}

impl ParamValue {
    /// An integer or a list of integers; anything else (a string, a
    /// decimal, null) is an error that shows the value.
    pub fn from_json(v: &serde_json::Value) -> Result<ParamValue, String> {
        let parsed = match v {
            serde_json::Value::Array(xs) => xs
                .iter()
                .map(|x| x.as_i64())
                .collect::<Option<Vec<i64>>>()
                .map(ParamValue::List),
            _ => v.as_i64().map(ParamValue::Int),
        };
        parsed.ok_or_else(|| format!("expected an integer or a list of integers, got {v}"))
    }

    /// Parses `3` or `1,2,3` (a trailing comma forces a one-element list).
    pub fn parse(s: &str) -> Result<ParamValue, String> {
        let s = s.trim();
        let num = |t: &str| {
            t.trim()
                .parse::<i64>()
                .map_err(|_| format!("not an integer: {t:?}"))
        };
        if s.contains(',') {
            s.split(',')
                .filter(|t| !t.trim().is_empty())
                .map(num)
                .collect::<Result<_, _>>()
                .map(ParamValue::List)
        } else {
            num(s).map(ParamValue::Int)
        }
    }
}

impl std::fmt::Display for ParamValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParamValue::Int(v) => write!(f, "{v}"),
            ParamValue::List(v) => {
                let parts: Vec<String> = v.iter().map(|x| x.to_string()).collect();
                write!(f, "[{}]", parts.join(","))
            }
        }
    }
}

impl Config {
    pub fn from_json(json: &str) -> Result<Config, String> {
        serde_json::from_str(json).map_err(|e| format!("bad config JSON: {e}"))
    }

    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).expect("config serializes")
    }

    pub fn module(&self, id: &str) -> Option<&ModuleConfig> {
        self.modules.iter().find(|m| m.module == id)
    }

    fn module_mut(&mut self, id: &str) -> Result<&mut ModuleConfig, String> {
        match self.modules.iter().position(|m| m.module == id) {
            Some(i) => Ok(&mut self.modules[i]),
            None => {
                // 020: name the modules there are, as a config file's typo does.
                let have: Vec<&str> = self.modules.iter().map(|m| m.module.as_str()).collect();
                Err(format!(
                    "config has no module {id:?} (it has: {})",
                    have.join(", ")
                ))
            }
        }
    }

    /// Applies one override. Used by sweeps; the result is still validated
    /// when rules are built from it.
    ///
    /// - `countries=N`
    /// - `<module>.<param>=<value>`, value `3` or `1,2,3`; an empty value
    ///   unsets the parameter
    /// - `<module>.variant=<name>` (`variant` is reserved, never a parameter
    ///   name): switches variant and drops the parameters the new variant
    ///   does not take, so only its new parameters need setting
    /// - `<module>=off`: removes the module and its phases from the round
    /// - `<table>.<id>.<path>=<JSON>` (two dots or more, so never a parameter):
    ///   a content overlay entry, as `cards.factory.cost=4`, replacing any
    ///   entry of the same key; an empty value drops the entry
    pub fn apply_override(&mut self, spec: &str) -> Result<(), String> {
        let (key, value) = spec
            .split_once('=')
            .ok_or_else(|| format!("override {spec:?}: expected key=value"))?;
        let (key, value) = (key.trim(), value.trim());
        if crate::content::is_overlay_key(key) {
            let entry = format!("{key}={value}");
            let at = self
                .content
                .iter()
                .position(|e| e.split_once('=').is_some_and(|(k, _)| k == key));
            match (at, value.is_empty()) {
                (Some(i), true) => {
                    self.content.remove(i);
                }
                (Some(i), false) => self.content[i] = entry,
                (None, true) => {}
                (None, false) => self.content.push(entry),
            }
            return Ok(());
        }
        if key == "countries" {
            self.countries = value
                .parse()
                .map_err(|_| format!("countries: not a number: {value:?}"))?;
            return Ok(());
        }
        let Some((module, param)) = key.split_once('.') else {
            if value != "off" {
                return Err(format!(
                    "override {spec:?}: expected <module>.<param>=<value> or <module>=off"
                ));
            }
            self.module_mut(key)?;
            let prefix = format!("{key}.");
            let round: Vec<String> = self
                .round
                .iter()
                .filter_map(|entry| {
                    let parts: Vec<&str> = entry
                        .split('+')
                        .filter(|p| !p.starts_with(&prefix))
                        .collect();
                    (!parts.is_empty()).then(|| parts.join("+"))
                })
                .collect();
            // An empty round means "every module's phases"; never turn an
            // explicit round into that by accident.
            if round.is_empty() && !self.round.is_empty() {
                return Err(format!("removing {key} leaves the round with no phases"));
            }
            self.modules.retain(|m| m.module != key);
            self.round = round;
            return Ok(());
        };
        let m = self.module_mut(module)?;
        if param == "variant" {
            m.variant = value.to_string();
            let def =
                crate::modules::find(module).ok_or_else(|| format!("unknown module {module:?}"))?;
            if let Some(v) = def.variants.iter().find(|v| v.name == value) {
                m.params.retain(|k, _| v.params.iter().any(|p| p.name == k));
                if !v.uses_cards {
                    m.cards.clear();
                }
            }
        } else if value.is_empty() {
            m.params.remove(param);
        } else {
            let parsed = ParamValue::parse(value).map_err(|e| format!("{module}.{param}: {e}"))?;
            m.params.insert(param.to_string(), parsed);
        }
        Ok(())
    }

    /// Stable hash of the config's content (not its `name`), as 16 hex
    /// digits. Meaningful on a *resolved* config (`Rules::config`), where
    /// `round` is filled in and lists are normalized, so equal games always
    /// hash equal.
    pub fn hash(&self) -> String {
        let mut c = self.clone();
        c.name.clear();
        let json = serde_json::to_string(&c).expect("config serializes");
        format!("{:016x}", fnv1a64(json.as_bytes()))
    }

    /// Human-readable differences from `other`, one per line item, for
    /// side-by-side reports. Compares module by module.
    pub fn diff(&self, other: &Config) -> Vec<String> {
        let mut out = Vec::new();
        if self.countries != other.countries {
            out.push(format!(
                "countries: {} -> {}",
                self.countries, other.countries
            ));
        }
        for m in &self.modules {
            let Some(o) = other.module(&m.module) else {
                out.push(format!("{}: only in A", m.module));
                continue;
            };
            if m.variant != o.variant {
                out.push(format!(
                    "{}.variant: {} -> {}",
                    m.module, m.variant, o.variant
                ));
            }
            let keys: std::collections::BTreeSet<&String> =
                m.params.keys().chain(o.params.keys()).collect();
            for k in keys {
                match (m.params.get(k), o.params.get(k)) {
                    (Some(a), Some(b)) if a == b => {}
                    (a, b) => out.push(format!("{}.{k}: {} -> {}", m.module, show(a), show(b))),
                }
            }
            if m.cards != o.cards {
                out.push(format!(
                    "{}.cards: {:?} -> {:?}",
                    m.module, m.cards, o.cards
                ));
            }
        }
        for o in &other.modules {
            if self.module(&o.module).is_none() {
                out.push(format!("{}: only in B", o.module));
            }
        }
        if self.round != other.round {
            out.push(format!("round: {:?} -> {:?}", self.round, other.round));
        }
        if self.content != other.content {
            out.push(format!("content: {:?} -> {:?}", self.content, other.content));
        }
        out
    }
}

fn show(v: Option<&ParamValue>) -> String {
    v.map_or_else(|| "(unset)".to_string(), |v| v.to_string())
}
