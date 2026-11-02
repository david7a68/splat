---
splat_schema: 1
splat_id: d2717406-1b0f-509c-a13e-75424455d68c
splat_kind: spell
splat_key: dnd2024:spells/protection_from_poison
splat_name: Protection from Poison
splat_origin:
  key: dnd2024:reference/spell/protection_from_poison
splat_sources:
- title: SRD 5.2.1
  locator: pp. 157
  url: https://www.dndbeyond.com/srd
---

# Protection from Poison

## Source text

<!-- SRD page 157 -->

Level 2 Abjuration (Cleric, Druid, Paladin, Ranger)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: 1 hour

You touch a creature and end the Poisoned condition on it. For the duration, the target has Advantage on saving throws to avoid or end the Poisoned condition, and it has Resistance to Poison damage.

Original layout: [[../../srd-5.2.1/pages/157|page 157]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["cleric", "druid", "paladin", "ranger"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 hour ` |
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
| ` mechanic ` | ` dnd2024:spells/protection_from_poison ` |
| ` implementation ` | ` dnd2024:spell_protection_from_poison ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 3600 ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` touch ` | ` true ` |

#### Immediate

<!-- splat:values path=splat_rule/parameters/immediate -->

| Field | Value |
| --- | --- |
| ` end_condition ` | ` poisoned ` |

#### Benefits

<!-- splat:yaml path=splat_rule/parameters/benefits -->

```yaml
save_advantage:
  condition: poisoned
  purposes:
  - avoid
  - end
damage_resistance: poison
```
