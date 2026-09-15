---
splat_schema: 1
splat_id: 25f83e04-8d94-5821-bd9f-8febd5436a55
splat_kind: spell
splat_key: dnd2024:spells/guidance
splat_name: Guidance
splat_origin:
  key: dnd2024:reference/spell/guidance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 138
  url: https://www.dndbeyond.com/srd
---

# Guidance

## Source text

<!-- SRD page 138 -->

Divination Cantrip (Cleric, Druid)\
Casting Time: Action\
Range: Touch\
Component: V, S\
Duration: Concentration, up to 1 minute\
You touch a willing creature and choose a skill. Until\
the spell ends, the creature adds 1d4 to any ability\
check using the chosen skill.\

Original layout: [[../../srd-5.2.1/pages/138|page 138]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
| ` school ` | ` divination ` |
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
| ` mechanic ` | ` dnd2024:spells/guidance ` |
| ` implementation ` | ` dnd2024:spell_guidance ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` willing ` | ` true ` |
| ` touch ` | ` true ` |

#### Skill choice

<!-- splat:values path=splat_rule/parameters/skill_choice -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` selection ` | ` skill ` |

#### Bonus

<!-- splat:yaml path=splat_rule/parameters/bonus -->

```yaml
roll: ability_check
requires_skill: chosen
amount:
  dice:
    count: 1
    sides: 4
    add: 0
all_matching_checks: true
```
