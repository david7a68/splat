---
splat_schema: 1
splat_id: c60032dc-0ab4-5660-afd7-386b0c6fc57d
splat_kind: spell
splat_key: dnd2024:spells/resistance
splat_name: Resistance
splat_origin:
  key: dnd2024:reference/spell/resistance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 158
  url: https://www.dndbeyond.com/srd
---

# Resistance

## Source text

<!-- SRD page 158 -->

Abjuration Cantrip (Cleric, Druid)\
Casting Time: Action\
Range: Touch\
Component: V, S\
Duration: Concentration, up to 1 minute\
You touch a willing creature and choose a damage\
type: Acid, Bludgeoning, Cold, Fire, Lightning, Ne-\
crotic, Piercing, Poison, Radiant, Slashing, or Thun-\
der. When the creature takes damage of the chosen\
type before the spell ends, the creature reduces the\
total damage taken by 1d4. A creature can benefit\
from this spell only once per turn.\

Original layout: [[../../srd-5.2.1/pages/158|page 158]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["cleric", "druid"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/resistance ` |
| ` implementation ` | ` dnd2024:spell_resistance ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_type_choice ` | ` ["acid", "bludgeoning", "cold", "fire", "lightning", "necrotic", "piercing", "poison", "radiant", "slashing", "thunder"] ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` willing ` | ` true ` |
| ` touch ` | ` true ` |

#### Reduction

<!-- splat:values path=splat_rule/parameters/reduction -->

| Field | Value |
| --- | --- |
| ` event ` | ` target.takes_chosen_damage_type ` |
| ` total_damage_reduced_by ` | ` 1d4 ` |
| ` max_benefits_per_creature_per_turn ` | ` 1 ` |
| ` duration ` | ` spell_duration ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: minute
```
