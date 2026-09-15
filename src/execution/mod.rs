//! A bounded execution proof for schema-1 attacks, saves, healing and consumables.
//! No package code is executed. Unsupported fields fail compilation.
pub mod builtin;
pub mod dice;
pub mod model;
#[cfg(test)]
mod tests;
use dice::{Dice, Random};
use model::*;
use serde::Serialize;
use serde_yaml_ng::Value;
use std::collections::BTreeSet;
use uuid::Uuid;

pub const ENGINE_ID: &str = "splat:execution-proof-v1";

#[derive(Clone, Debug, Serialize)]
pub struct Actor {
    pub id: Uuid,
    pub definition: String,
    pub name: String,
    pub hp: i64,
    pub max_hp: i64,
    pub temporary_hp: i64,
    pub pools: Map<i64>,
    pub action_available: bool,
    pub bonus_action_available: bool,
    pub slot_expended: bool,
    pub spells: BTreeSet<String>,
    pub spellcasting: Option<Spellcasting>,
    pub items: Map<ItemInstance>,
}
#[derive(Clone, Debug, Serialize)]
pub struct Spellcasting {
    pub actor_level: u8,
    pub modifier: i64,
    pub save_dc: i64,
}
#[derive(Clone, Debug, Serialize)]
pub struct ItemInstance {
    pub id: Uuid,
    pub definition: String,
    pub quantity: i64,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Event {
    pub sequence: usize,
    pub use_id: usize,
    pub actor: String,
    pub target: Option<String>,
    pub kind: String,
    pub detail: serde_json::Value,
}
#[derive(Clone)]
pub struct Engine {
    pub definitions: Map<Definition>,
    pub actors: Map<Actor>,
    pub events: Vec<Event>,
    pub current: Option<String>,
    random: Random,
    next_use: usize,
}

/// Explicit encounter facts supplied by the controller. None means unknown,
/// never "probably true". These facts avoid adding maps or guessing geometry.
#[derive(Clone, Debug)]
pub struct TargetFacts {
    pub range: Option<RangeBand>,
    pub seen: Option<bool>,
    pub clear_path: Option<bool>,
    pub attack_advantage: Option<bool>,
    pub attack_disadvantage: Option<bool>,
    pub save_advantage: Option<i8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RangeBand {
    Normal,
    Long,
    OutOfRange,
}
impl TargetFacts {
    pub fn clear() -> Self {
        Self {
            range: Some(RangeBand::Normal),
            seen: Some(true),
            clear_path: Some(true),
            attack_advantage: Some(false),
            attack_disadvantage: Some(false),
            save_advantage: Some(0),
        }
    }
}
#[derive(Clone, Debug)]
pub struct AreaFacts {
    pub members: Vec<String>,
    pub origin_in_range: Option<bool>,
    pub origin_clear_path: Option<bool>,
}
#[derive(Clone, Debug)]
pub struct Use {
    pub actor: String,
    pub definition: String,
    pub action: String,
    pub targets: Map<Vec<String>>,
    pub bindings: Map<Value>,
    pub facts: Map<TargetFacts>,
    pub areas: Map<AreaFacts>,
    pub source_item: Option<String>,
    pub can_speak: Option<bool>,
    pub free_hands: Option<i64>,
    pub item_accessible: Option<bool>,
}
impl Use {
    pub fn new(actor: &str, definition: &str, action: &str, targets: &[&str]) -> Self {
        Self {
            actor: actor.into(),
            definition: definition.into(),
            action: action.into(),
            targets: Map::from([(
                "chosen".into(),
                targets.iter().map(|s| s.to_string()).collect(),
            )]),
            bindings: Map::new(),
            facts: Map::new(),
            areas: Map::new(),
            source_item: None,
            can_speak: None,
            free_hands: None,
            item_accessible: None,
        }
    }
}

impl Engine {
    pub fn new(entries: &[crate::content::Entry], random: Random) -> Result<Self, String> {
        let mut definitions = Map::new();
        for e in entries {
            if ["creature", "spell", "item"].contains(&e.kind.as_str()) {
                let d = compile(e)?;
                let key = e
                    .key
                    .clone()
                    .ok_or("Executable definition requires a key")?;
                if definitions.insert(key, d).is_some() {
                    return Err("Duplicate executable key".into());
                }
            }
        }
        Ok(Self {
            definitions,
            actors: Map::new(),
            events: Vec::new(),
            current: None,
            random,
            next_use: 0,
        })
    }
    pub fn add_actor(&mut self, key: &str, definition: &str) -> Result<(), String> {
        if self.actors.contains_key(key) {
            return Err("Duplicate actor placement".into());
        }
        let d = self.definitions.get(definition).ok_or("Unknown creature")?;
        if d.creature.is_none() {
            return Err("Participant must be a creature".into());
        }
        let hp = d.stats["max_hp"].value;
        self.actors.insert(
            key.into(),
            Actor {
                id: Uuid::new_v4(),
                definition: definition.into(),
                name: key.into(),
                hp,
                max_hp: hp,
                temporary_hp: 0,
                pools: d
                    .pools
                    .iter()
                    .map(|(key, p)| (key.clone(), p.initial.unwrap_or(p.max)))
                    .collect(),
                action_available: false,
                bonus_action_available: false,
                slot_expended: false,
                spells: BTreeSet::new(),
                spellcasting: None,
                items: Map::new(),
            },
        );
        Ok(())
    }
    pub fn give_item(
        &mut self,
        actor: &str,
        key: &str,
        definition: &str,
        quantity: i64,
    ) -> Result<(), String> {
        if quantity <= 0
            || self
                .definitions
                .get(definition)
                .is_none_or(|d| d.item.is_none())
        {
            return Err("Invalid item placement".into());
        }
        let a = self.actors.get_mut(actor).ok_or("Unknown actor")?;
        if a.items.contains_key(key) {
            return Err("Duplicate item placement".into());
        }
        a.items.insert(
            key.into(),
            ItemInstance {
                id: Uuid::new_v4(),
                definition: definition.into(),
                quantity,
            },
        );
        Ok(())
    }
    pub fn start_turn(&mut self, actor: &str) -> Result<(), String> {
        let a = self.actors.get_mut(actor).ok_or("Unknown actor")?;
        if a.hp == 0 {
            return Err("Dead actor cannot start a turn".into());
        }
        a.action_available = true;
        a.bonus_action_available = true;
        a.slot_expended = false;
        self.current = Some(actor.into());
        self.log(0, actor, None, "turn_start", serde_json::json!({}));
        Ok(())
    }
    fn log(
        &mut self,
        use_id: usize,
        actor: &str,
        target: Option<&str>,
        kind: &str,
        detail: serde_json::Value,
    ) {
        self.events.push(Event {
            sequence: self.events.len() + 1,
            use_id,
            actor: actor.into(),
            target: target.map(str::to_owned),
            kind: kind.into(),
            detail,
        });
    }
    /// Requests are checked on a private candidate. Content errors, missing facts or
    /// unavailable choices leave both state and random stream untouched. A resolved
    /// miss/successful save still commits costs. Interruptions are unsupported.
    pub fn use_action(&mut self, request: Use) -> Result<(), String> {
        let mut candidate = self.clone();
        candidate.resolve(request)?;
        *self = candidate;
        Ok(())
    }
    fn resolve(&mut self, mut request: Use) -> Result<(), String> {
        let definition = self
            .definitions
            .get(&request.definition)
            .ok_or("Unknown executable definition")?
            .clone();
        let action = definition
            .actions
            .get(&request.action)
            .ok_or("Unknown action")?
            .clone();
        let actor = self.actors.get(&request.actor).ok_or("Unknown actor")?;
        if self.current.as_deref() != Some(&request.actor) || actor.hp == 0 {
            return Err("Actor is not active on its turn".into());
        }
        let budget = if action.activation.economy == "action" {
            actor.action_available
        } else {
            actor.bonus_action_available
        };
        if !budget {
            return Err("Action economy is spent".into());
        }
        if definition.creature.is_some() && actor.definition != request.definition {
            return Err("Actor does not have this creature action".into());
        }
        if definition.spell.is_some() && !actor.spells.contains(&request.definition) {
            return Err("Actor has no grant for this spell".into());
        }
        if definition.item.is_some() {
            let item = request
                .source_item
                .as_ref()
                .and_then(|key| actor.items.get(key))
                .ok_or("Missing source item instance")?;
            if item.definition != request.definition || item.quantity <= 0 {
                return Err("Item is unavailable".into());
            }
        }
        if definition.spell.is_some() {
            let grant = actor
                .spellcasting
                .as_ref()
                .ok_or("Missing spellcasting grant values")?;
            if !(1..=20).contains(&grant.actor_level) {
                return Err("Unsupported actor level".into());
            }
            let tier = [5, 11, 17]
                .into_iter()
                .filter(|level| grant.actor_level >= *level)
                .count() as i64;
            for (name, value) in [
                ("cantrip_tier", tier),
                ("save_dc", grant.save_dc),
                ("spellcasting_modifier", grant.modifier),
            ] {
                if definition.parameters.contains_key(name) {
                    if request
                        .bindings
                        .get(name)
                        .is_some_and(|v| v.as_i64() != Some(value))
                    {
                        return Err(format!(
                            "Binding {name} conflicts with the spellcasting grant"
                        ));
                    }
                    request.bindings.insert(name.into(), value.into());
                }
            }
        }
        for key in request.bindings.keys() {
            if !definition.parameters.contains_key(key) {
                return Err(format!("Unknown supplied binding {key}"));
            }
        }
        for (key, param) in &definition.parameters {
            if !request.bindings.contains_key(key)
                && let Some(default) = &param.default
            {
                request.bindings.insert(key.clone(), default.clone());
            }
            match request.bindings.get(key) {
                None if param.required => return Err(format!("Missing binding {key}")),
                Some(v) if param.kind == "integer" => {
                    let n = v.as_i64().ok_or("Expected integer binding")?;
                    if param.minimum.is_some_and(|m| n < m) || param.maximum.is_some_and(|m| n > m)
                    {
                        return Err(format!("Out-of-range binding {key}"));
                    }
                }
                Some(v)
                    if param.kind == "actor_resource"
                        && !v.as_str().is_some_and(|s| actor.pools.contains_key(s)) =>
                {
                    return Err("Missing bound actor resource".into());
                }
                _ => (),
            }
        }
        if let Some(spell) = &definition.spell {
            if spell.components.iter().any(|s| s == "verbal") && request.can_speak != Some(true) {
                return Err("Verbal component unavailable or unknown".into());
            }
            if spell.components.iter().any(|s| s == "somatic")
                && request.free_hands.is_none_or(|n| n < 1)
            {
                return Err("Somatic component unavailable or unknown".into());
            }
            if spell.level > 0 {
                let cost = action
                    .costs
                    .iter()
                    .find(|c| c.at == "dnd2024:spell_slot_expenditure")
                    .ok_or("Leveled spell needs slot method")?;
                let pool = resource_name(cost.resource.as_ref().unwrap(), &request.bindings)?;
                let pooldef = self.definitions[&actor.definition]
                    .pools
                    .get(&pool)
                    .ok_or("Unknown slot pool definition")?;
                let level = request
                    .bindings
                    .get("cast_level")
                    .and_then(Value::as_i64)
                    .ok_or("Missing cast level")?;
                if pooldef.kind.as_deref() != Some("spell_slot")
                    || pooldef.level != Some(level)
                    || level < spell.level
                {
                    return Err("Slot level does not match cast level".into());
                }
                if actor.slot_expended {
                    return Err("Only one spell slot may be expended per turn".into());
                }
            }
        }
        if request.targets.keys().collect::<BTreeSet<_>>()
            != action.targets.keys().collect::<BTreeSet<_>>()
        {
            return Err("Target binding mismatch".into());
        }
        for (key, selector) in &action.targets {
            let targets = &request.targets[key];
            let unique: BTreeSet<_> = targets.iter().collect();
            if unique.len() != targets.len()
                || targets.len() < selector.count.min
                || targets.len() > selector.count.max
            {
                return Err("Invalid or duplicate target selection".into());
            }
            if let Some(area) = &selector.assumption {
                let members = request
                    .areas
                    .get(area)
                    .ok_or("Missing complete area membership assumption")?;
                if members.origin_in_range != Some(true) || members.origin_clear_path != Some(true)
                {
                    return Err("Area origin range or clear path is unavailable or unknown".into());
                }
                if members.members.iter().collect::<BTreeSet<_>>() != unique
                    || members.members.len() != unique.len()
                {
                    return Err("Area must include every affected creature exactly once".into());
                }
            }
            for target in targets {
                let target_actor = self.actors.get(target).ok_or("Unknown target")?;
                if target_actor.hp == 0 {
                    return Err("Dead targets are outside this proof's legal target set".into());
                }
                if target == &request.actor && !selector.allow_self {
                    return Err("Self target disallowed".into());
                }
                let facts = request.facts.get(target).ok_or("Missing target facts")?;
                if selector.assumption.is_none()
                    && (!(matches!(facts.range, Some(RangeBand::Normal))
                        || (facts.range == Some(RangeBand::Long) && selector.range.long.is_some()))
                        || facts.clear_path != Some(true))
                {
                    return Err("Range or clear path unavailable or unknown".into());
                }
                if selector.visibility.as_deref() == Some("seen") && facts.seen != Some(true) {
                    return Err("Visibility unavailable or unknown".into());
                }
            }
        }
        for r in &action.requirements {
            match r.predicate.as_str() {
                "accessible" if request.item_accessible != Some(true) => {
                    return Err("Item access unavailable or unknown".into());
                }
                "free_hands" if request.free_hands.is_none_or(|n| n < r.minimum.unwrap()) => {
                    return Err("Free hand requirement not met".into());
                }
                _ => (),
            }
        }
        for cost in &action.costs {
            if let Some(resource) = &cost.resource {
                let pool = resource_name(resource, &request.bindings)?;
                if actor.pools.get(&pool).copied().unwrap_or(0) < cost.amount.unwrap() {
                    return Err("Insufficient resource".into());
                }
            }
            if let Some(quantity) = cost.quantity {
                let item = request
                    .source_item
                    .as_ref()
                    .and_then(|key| actor.items.get(key))
                    .ok_or("Missing item cost source")?;
                if item.quantity < quantity {
                    return Err("Insufficient item quantity".into());
                }
            }
        }
        // Evaluate dynamic dice before commitment. Every action-supplied roll is bounded.
        let mut dice = Map::new();
        for (name, r) in &action.rolls {
            let expression = if let Some(text) = &r.expression {
                text.clone()
            } else {
                let d = r.dice.as_ref().unwrap();
                let count = eval(&d.count, &request.bindings)?;
                if !(1..=1000).contains(&count) {
                    return Err("Dynamic dice count is out of bounds".into());
                }
                format!("{count}d{}{:+}", d.sides, eval(&d.add, &request.bindings)?)
            };
            let modifier = r
                .modifier
                .as_ref()
                .map(|v| eval(v, &request.bindings))
                .transpose()?
                .unwrap_or(0);
            dice.insert(name.clone(), (Dice::parse(&expression)?, modifier));
        }
        self.next_use += 1;
        let use_id = self.next_use;
        let a = self.actors.get_mut(&request.actor).unwrap();
        if action.activation.economy == "action" {
            a.action_available = false
        } else {
            a.bonus_action_available = false
        }
        self.log(use_id,&request.actor,None,"commit",serde_json::json!({"definition":request.definition,"action":request.action,"budget":action.activation.economy}));
        for cost in &action.costs {
            if let Some(resource) = &cost.resource {
                let pool = resource_name(resource, &request.bindings)?;
                let amount = cost.amount.unwrap();
                let a = self.actors.get_mut(&request.actor).unwrap();
                *a.pools.get_mut(&pool).unwrap() -= amount;
                a.slot_expended = true;
                self.log(
                    use_id,
                    &request.actor,
                    None,
                    "resource_expended",
                    serde_json::json!({"pool":pool,"amount":amount}),
                );
            }
            if let Some(quantity) = cost.quantity {
                let key = request.source_item.as_ref().unwrap();
                self.actors
                    .get_mut(&request.actor)
                    .unwrap()
                    .items
                    .get_mut(key)
                    .unwrap()
                    .quantity -= quantity;
                self.log(
                    use_id,
                    &request.actor,
                    None,
                    "quantity_expended",
                    serde_json::json!({"item":key,"amount":quantity}),
                );
            }
        }
        let mut context = Resolution {
            request: &request,
            action: &action,
            dice: &dice,
            cache: Map::new(),
            use_id,
            operations: 0,
            attack_index: 0,
        };
        self.steps(&action.steps, &mut context)?;
        Ok(())
    }
    fn steps(&mut self, steps: &[Step], ctx: &mut Resolution<'_>) -> Result<(), String> {
        for step in steps {
            ctx.operations += 1;
            if ctx.operations > 1000 {
                return Err("Resolution operation limit reached".into());
            }
            if step.operation == "splat:repeat" {
                let count = eval(step.count.as_ref().unwrap(), &ctx.request.bindings)?;
                if !(0..=100).contains(&count) {
                    return Err("Repeat count out of bounds".into());
                }
                for _ in 0..count {
                    self.steps(step.steps.as_ref().unwrap(), ctx)?;
                }
                continue;
            }
            for target in &ctx.request.targets[step.target.as_ref().unwrap()] {
                if self.actors[target].hp == 0 {
                    continue;
                } // A previous Multiattack resolution can kill it.
                match step.operation.as_str() {
                    "dnd2024:attack" => {
                        ctx.attack_index += 1;
                        let facts = &ctx.request.facts[target];
                        let has_advantage = facts
                            .attack_advantage
                            .ok_or("Attack advantage state is unknown")?;
                        let has_disadvantage = facts
                            .attack_disadvantage
                            .ok_or("Attack disadvantage state is unknown")?
                            || facts.range == Some(RangeBand::Long);
                        let advantage = i8::from(has_advantage) - i8::from(has_disadvantage);
                        let roll_name = step.roll.as_ref().unwrap();
                        let (_, modifier) = &ctx.dice[roll_name];
                        let natural = self.d20(advantage)?;
                        let total = natural
                            .checked_add(*modifier)
                            .ok_or("Attack total overflow")?;
                        let ac =
                            self.definitions[&self.actors[target].definition].stats["armor_class"]
                                .value;
                        let critical = natural == 20;
                        let hit = natural != 1 && (critical || total >= ac);
                        self.log(ctx.use_id,&ctx.request.actor,Some(target),"attack",serde_json::json!({"step":step.key,"natural":natural,"total":total,"armor_class":ac,"hit":hit,"critical":critical,"advantage":advantage}));
                        if hit {
                            self.damage(
                                &step.on_hit.as_ref().unwrap().damage,
                                target,
                                critical,
                                1,
                                1,
                                ctx,
                            )?;
                        }
                    }
                    "dnd2024:save_damage" => {
                        // A use-scoped area roll is evaluated once and reused for every target.
                        let dc = eval(step.dc.as_ref().unwrap(), &ctx.request.bindings)?;
                        let advantage = ctx.request.facts[target]
                            .save_advantage
                            .ok_or("Saving throw advantage state is unknown")?;
                        let ability = step.ability.as_ref().unwrap();
                        let d = &self.definitions[&self.actors[target].definition];
                        let modifier = d
                            .stats
                            .get(&format!("save_{ability}"))
                            .map(|s| s.value)
                            .unwrap_or_else(|| (d.stats[ability].value - 10).div_euclid(2));
                        let natural = self.d20(advantage)?;
                        let total = natural.checked_add(modifier).ok_or("Save total overflow")?;
                        let success = total >= dc;
                        self.log(ctx.use_id,&ctx.request.actor,Some(target),"save",serde_json::json!({"step":step.key,"ability":ability,"natural":natural,"total":total,"dc":dc,"success":success}));
                        let scale = &step.success.as_ref().unwrap().damage_scale;
                        self.damage(
                            step.damage.as_ref().unwrap(),
                            target,
                            false,
                            if success { scale.numerator } else { 1 },
                            if success { scale.denominator } else { 1 },
                            ctx,
                        )?;
                    }
                    "dnd2024:heal" => {
                        let amount = self
                            .amount(step.amount.as_ref().unwrap(), target, false, ctx)?
                            .max(0);
                        let a = self.actors.get_mut(target).unwrap();
                        let restored = amount.min(a.max_hp - a.hp);
                        a.hp += restored;
                        self.log(
                            ctx.use_id,
                            &ctx.request.actor,
                            Some(target),
                            "healing",
                            serde_json::json!({"rolled":amount,"restored":restored}),
                        );
                    }
                    _ => return Err("Unregistered operation".into()),
                }
            }
        }
        Ok(())
    }
    fn d20(&mut self, advantage: i8) -> Result<i64, String> {
        if ![-1, 0, 1].contains(&advantage) {
            return Err("Advantage must be -1, 0 or 1".into());
        }
        let first = i64::from(self.random.die(20)?);
        if advantage == 0 {
            return Ok(first);
        }
        let second = i64::from(self.random.die(20)?);
        Ok(if advantage > 0 {
            first.max(second)
        } else {
            first.min(second)
        })
    }
    fn amount(
        &mut self,
        value: &Value,
        target: &str,
        critical: bool,
        ctx: &mut Resolution<'_>,
    ) -> Result<i64, String> {
        if let Some(name) = value
            .as_mapping()
            .and_then(|m| m.get("roll"))
            .and_then(Value::as_str)
        {
            let def = &ctx.action.rolls[name];
            let scope = match def.scope.as_str() {
                "use" => "use".into(),
                "target" => format!("target/{target}"),
                "attack" => format!("attack/{}", ctx.attack_index),
                _ => return Err("Unknown roll scope".into()),
            };
            let cache_key = format!("{name}/{scope}/{critical}");
            if let Some(value) = ctx.cache.get(&cache_key) {
                return Ok(*value);
            }
            let (dice, modifier) = &ctx.dice[name];
            let roll = dice.roll(&mut self.random, critical)?;
            let total = roll.total.checked_add(*modifier).ok_or("Roll overflow")?;
            self.log(ctx.use_id,&ctx.request.actor,Some(target),"roll",serde_json::json!({"name":name,"scope":scope,"dice":roll.dice,"total":total,"critical":critical}));
            ctx.cache.insert(cache_key, total);
            Ok(total)
        } else if let Some(map) = value.as_mapping()
            && let Some(operator) = map.get("op").and_then(Value::as_str)
        {
            let args = map
                .get("args")
                .and_then(Value::as_sequence)
                .ok_or("Missing expression args")?;
            if args.len() != 2 {
                return Err("Invalid expression arity".into());
            }
            let left = self.amount(&args[0], target, critical, ctx)?;
            let right = self.amount(&args[1], target, critical, ctx)?;
            match operator {
                "add" => left.checked_add(right),
                "subtract" => left.checked_sub(right),
                "multiply" => left.checked_mul(right),
                _ => None,
            }
            .ok_or_else(|| "Unsupported or overflowing amount expression".into())
        } else {
            eval(value, &ctx.request.bindings)
        }
    }
    fn damage(
        &mut self,
        components: &[Damage],
        target: &str,
        critical: bool,
        numerator: i64,
        denominator: i64,
        ctx: &mut Resolution<'_>,
    ) -> Result<(), String> {
        let d = self.definitions[&self.actors[target].definition].clone();
        let mut affinities = Affinities::default();
        for t in d
            .traits
            .values()
            .filter(|t| t.rule == "dnd2024:damage_affinities")
        {
            let a: Affinities =
                serde_yaml_ng::from_value(t.parameters.clone()).map_err(|e| e.to_string())?;
            affinities.immunities.extend(a.immunities);
            affinities.resistances.extend(a.resistances);
            affinities.vulnerabilities.extend(a.vulnerabilities);
        }
        let mut grouped = Map::<i64>::new();
        for component in components {
            let amount = self
                .amount(&component.amount, target, critical, ctx)?
                .max(0);
            let total = grouped.entry(component.kind.clone()).or_default();
            *total = total.checked_add(amount).ok_or("Damage overflow")?;
        }
        let mut total = 0_i64;
        let mut details = Vec::new();
        for (kind, raw) in grouped {
            let scaled = raw.checked_mul(numerator).ok_or("Damage overflow")? / denominator;
            let mut adjusted = scaled;
            if affinities.immunities.contains(&kind) {
                adjusted = 0
            } else {
                if affinities.resistances.contains(&kind) {
                    adjusted /= 2;
                }
                if affinities.vulnerabilities.contains(&kind) {
                    adjusted = adjusted.checked_mul(2).ok_or("Damage overflow")?;
                }
            }
            total = total.checked_add(adjusted).ok_or("Damage overflow")?;
            details.push(
                serde_json::json!({"type":kind,"raw":raw,"scaled":scaled,"adjusted":adjusted}),
            );
        }
        let a = self.actors.get_mut(target).unwrap();
        let absorbed = total.min(a.temporary_hp);
        a.temporary_hp -= absorbed;
        let lost = (total - absorbed).min(a.hp);
        a.hp -= lost;
        let hp = a.hp;
        self.log(ctx.use_id,&ctx.request.actor,Some(target),"damage",serde_json::json!({"components":details,"total":total,"temporary_hp_absorbed":absorbed,"hp_lost":lost,"hp":hp,"dead":hp==0}));
        Ok(())
    }
}
struct Resolution<'a> {
    request: &'a Use,
    action: &'a Action,
    dice: &'a Map<(Dice, i64)>,
    cache: Map<i64>,
    use_id: usize,
    operations: usize,
    attack_index: usize,
}
fn resource_name(value: &Value, bindings: &Map<Value>) -> Result<String, String> {
    let name = value
        .as_mapping()
        .and_then(|m| m.get("binding"))
        .and_then(Value::as_str)
        .ok_or("Resource selector must be a binding")?;
    bindings
        .get(name)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| "Missing resource binding".into())
}
fn eval(value: &Value, bindings: &Map<Value>) -> Result<i64, String> {
    if let Some(n) = value.as_i64() {
        return Ok(n);
    }
    let m = value.as_mapping().ok_or("Invalid expression")?;
    if let Some(name) = m.get("binding").and_then(Value::as_str) {
        return bindings
            .get(name)
            .and_then(Value::as_i64)
            .ok_or_else(|| format!("Missing numeric binding {name}"));
    }
    let args = m
        .get("args")
        .and_then(Value::as_sequence)
        .ok_or("Missing expression args")?;
    if args.len() != 2 {
        return Err("Invalid expression arity".into());
    }
    let left = eval(&args[0], bindings)?;
    let right = eval(&args[1], bindings)?;
    match m.get("op").and_then(Value::as_str) {
        Some("add") => left.checked_add(right),
        Some("subtract") => left.checked_sub(right),
        Some("multiply") => left.checked_mul(right),
        _ => return Err("Unsupported expression".into()),
    }
    .ok_or_else(|| "Expression overflow".into())
}
