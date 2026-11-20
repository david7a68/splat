//! A player character's inspiration, as the campaign's rules define it.
//!
//! The engine has no inspiration rule of its own. It runs the inspiration contracts below,
//! each with the parameters its rule entry gives, and a campaign gets its rule from the
//! packages it uses. No ruleset is the default: a campaign with one inspiration rule uses it,
//! a campaign with several picks one under [`CHOICE`], and a campaign with none has none.
use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;

/// The key in `splat_rule_choices` that picks one inspiration rule from several.
pub const CHOICE: &str = "splat:inspiration";

/// SRD 5.2.1, Heroic Inspiration: reroll one die immediately after rolling it.
pub const REROLL_CONTRACT: &str = "dnd2024:glossary_heroic_inspiration";
/// SRD 5.1, Inspiration: advantage on an attack roll, a saving throw or an ability check.
pub const ADVANTAGE_CONTRACT: &str = "dnd2014:inspiration";

/// What expending inspiration does.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Inspiration {
    /// Reroll one die immediately after rolling it, and use the new roll.
    Reroll,
    /// Advantage on a d20 test, chosen before the roll.
    Advantage,
}

/// The d20 tests the advantage contract may name.
const D20_TESTS: [&str; 3] = ["attack_roll", "saving_throw", "ability_check"];

fn text(value: Option<&Value>) -> Option<&str> {
    value.and_then(Value::as_str).map(str::trim)
}

fn number(value: Option<&Value>) -> Option<i64> {
    match value {
        Some(Value::Number(n)) => n.as_i64(),
        Some(Value::String(s)) => s.trim().parse().ok(),
        _ => None,
    }
}

fn flag(value: Option<&Value>) -> Option<bool> {
    match value {
        Some(Value::Bool(b)) => Some(*b),
        Some(Value::String(s)) => s.trim().parse().ok(),
        _ => None,
    }
}

impl Inspiration {
    /// The inspiration a rule entry's `splat_rule` value defines. `None` when the rule is not
    /// an inspiration contract; an error when it is one with parameters this engine cannot run.
    pub fn from_rule(rule: &Value) -> Option<Result<Self, String>> {
        let contract = text(rule.get("implementation"))?;
        if contract != REROLL_CONTRACT && contract != ADVANTAGE_CONTRACT {
            return None;
        }
        Some(Self::read(contract, rule.get("parameters")))
    }

    fn read(contract: &str, parameters: Option<&Value>) -> Result<Self, String> {
        let parameters = parameters.ok_or("The inspiration rule has no parameters.")?;
        if text(parameters.get("eligible")).is_some_and(|who| who != "player_character") {
            return Err("Only inspiration for player characters is supported.".into());
        }
        let expend = parameters
            .get("expend")
            .ok_or("The inspiration rule does not say how it is expended.")?;
        let timing = text(expend.get("timing"));
        if contract == REROLL_CONTRACT {
            if timing != Some("immediately_after_rolling_any_die")
                || number(expend.get("reroll_count")) != Some(1)
                || flag(expend.get("must_use_new_result")) != Some(true)
            {
                return Err("Only a reroll of one die, using the new roll, is supported.".into());
            }
            return Ok(Self::Reroll);
        }
        let tests: Vec<&str> = expend
            .get("applies_to")
            .and_then(Value::as_sequence)
            .map(|tests| tests.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default();
        if timing != Some("when_making_d20_test")
            || text(expend.get("grants")) != Some("advantage")
            || tests.is_empty()
            || tests.iter().any(|test| !D20_TESTS.contains(test))
        {
            return Err(
                "Only advantage on attack rolls, saving throws or ability checks is supported."
                    .into(),
            );
        }
        Ok(Self::Advantage)
    }

    /// What the DM does with it, for a rule's line in settings.
    pub fn effect(self) -> &'static str {
        match self {
            Self::Reroll => "reroll one die after the roll",
            Self::Advantage => "advantage on a d20 test",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::Entry;

    fn rule(yaml: &str) -> Value {
        serde_yaml_ng::from_str(yaml).unwrap()
    }

    #[test]
    fn the_srd_521_rule_rerolls_one_die() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../builtin/dnd2024-definitions/glossary/heroic_inspiration.md"
        );
        let text = std::fs::read_to_string(path).unwrap();
        let entry = Entry::parse("heroic_inspiration.md", &text).unwrap();
        assert_eq!(
            Inspiration::from_rule(&entry.fields["splat_rule"]),
            Some(Ok(Inspiration::Reroll))
        );
    }

    #[test]
    fn an_srd_51_rule_gives_advantage() {
        let srd51 = rule(
            "implementation: dnd2014:inspiration\nparameters:\n  eligible: player_character\n  capacity: 1\n  expend:\n    timing: when_making_d20_test\n    grants: advantage\n    applies_to: [attack_roll, saving_throw, ability_check]\n",
        );
        assert_eq!(
            Inspiration::from_rule(&srd51),
            Some(Ok(Inspiration::Advantage))
        );
    }

    #[test]
    fn other_rules_are_not_inspiration_and_bad_parameters_are_refused() {
        assert_eq!(
            Inspiration::from_rule(&rule("implementation: splat:manual_initiative")),
            None
        );
        for yaml in [
            "implementation: dnd2024:glossary_heroic_inspiration\nparameters: {expend: {timing: immediately_after_rolling_any_die, reroll_count: 2, must_use_new_result: true}}",
            "implementation: dnd2014:inspiration\nparameters: {expend: {timing: when_making_d20_test, grants: bonus_d6, applies_to: [attack_roll]}}",
            "implementation: dnd2014:inspiration\nparameters: {eligible: anyone, expend: {timing: when_making_d20_test, grants: advantage, applies_to: [attack_roll]}}",
            "implementation: dnd2014:inspiration\nparameters: {expend: {timing: at_dawn}}",
            "implementation: dnd2014:inspiration",
        ] {
            assert!(
                matches!(Inspiration::from_rule(&rule(yaml)), Some(Err(_))),
                "{yaml}"
            );
        }
    }
}
