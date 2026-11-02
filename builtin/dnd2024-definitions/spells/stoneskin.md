---
splat_schema: 1
splat_id: 5c5919d9-67ca-523e-9a4d-d49534a1c746
splat_kind: spell
splat_key: dnd2024:spells/stoneskin
splat_name: Stoneskin
splat_origin:
  key: dnd2024:reference/spell/stoneskin
splat_sources:
- title: SRD 5.2.1
  locator: pp. 165
  url: https://www.dndbeyond.com/srd
---

# Stoneskin

## Source text

<!-- SRD page 165 -->

Level 4 Transmutation (Druid, Ranger, Sorcerer, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (diamond dust worth 100+ GP, which the spell consumes)\
Duration: Concentration, up to 1 hour

Until the spell ends, one willing creature you touch has Resistance to Bludgeoning, Piercing, and Slashing damage.

Original layout: [[../../srd-5.2.1/pages/165|page 165]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 4 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["druid", "ranger", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 hour ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` diamond dust worth 100+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/stoneskin ` |
| ` implementation ` | ` dnd2024:spell_stoneskin ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` willing ` | ` true ` |
| ` touch ` | ` true ` |

#### Resistance

<!-- splat:values path=splat_rule/parameters/resistance -->

| Field | Value |
| --- | --- |
| ` types ` | ` ["bludgeoning", "piercing", "slashing"] ` |
| ` duration ` | ` spell_duration ` |
| ` magical_damage_included ` | ` true ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: hour
```
