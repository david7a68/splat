---
splat_schema: 1
splat_id: ae6847a4-e9e0-513b-b60d-6ea97b144ca6
splat_kind: spell
splat_key: dnd2024:spells/protection_from_energy
splat_name: Protection from Energy
splat_origin:
  key: dnd2024:reference/spell/protection_from_energy
splat_sources:
- title: SRD 5.2.1
  locator: pp. 157
  url: https://www.dndbeyond.com/srd
---

# Protection from Energy

## Source text

<!-- SRD page 157 -->

Level 3 Abjuration (Cleric, Druid, Ranger, Sorcerer,\
Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Concentration, up to 1 hour\
For the duration, the willing creature you touch has\
Resistance to one damage type of your choice: Acid,\
Cold, Fire, Lightning, or Thunder.\

Original layout: [[../../srd-5.2.1/pages/157|page 157]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["cleric", "druid", "ranger", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 hour ` |
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
| ` mechanic ` | ` dnd2024:spells/protection_from_energy ` |
| ` implementation ` | ` dnd2024:spell_protection_from_energy ` |

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
| ` choose_count ` | ` 1 ` |
| ` damage_types ` | ` ["acid", "cold", "fire", "lightning", "thunder"] ` |
| ` duration ` | ` spell_duration ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: hour
```
