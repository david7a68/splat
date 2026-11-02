---
splat_schema: 1
splat_id: bbdf55ef-a5be-5853-b290-8f40317602d9
splat_kind: spell
splat_key: dnd2024:spells/heroism
splat_name: Heroism
splat_origin:
  key: dnd2024:reference/spell/heroism
splat_sources:
- title: SRD 5.2.1
  locator: pp. 140
  url: https://www.dndbeyond.com/srd
---

# Heroism

## Source text

<!-- SRD page 140 -->

Level 1 Enchantment (Bard, Paladin)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Concentration, up to 1 minute

A willing creature you touch is imbued with bravery. Until the spell ends, the creature is immune to the Frightened condition and gains Temporary Hit Points equal to your spellcasting ability modifier at the start of each of its turns.

Using a Higher-Level Spell Slot. You can target one additional creature for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/140|page 140]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["bard", "paladin"] ` |
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
| ` mechanic ` | ` dnd2024:spells/heroism ` |
| ` implementation ` | ` dnd2024:spell_heroism ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition_immunity ` | ` ["frightened"] ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` willing ` | ` true ` |
| ` touch ` | ` true ` |

#### Temporary hp

<!-- splat:yaml path=splat_rule/parameters/temporary_hp -->

```yaml
event: turn.start
subject: target
amount:
  binding: spellcasting_modifier
```

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 1 ` |
| ` additional_targets ` | ` 1 ` |
