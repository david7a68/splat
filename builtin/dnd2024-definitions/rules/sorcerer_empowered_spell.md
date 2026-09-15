---
splat_schema: 1
splat_id: f560dd00-d196-560f-8de0-5edf974c6b13
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_empowered_spell
splat_name: Empowered Spell
splat_origin:
  key: dnd2024:reference/pages/067
splat_sources:
- title: SRD 5.2.1
  locator: pp. 67
  url: https://www.dndbeyond.com/srd
---

# Empowered Spell

## Source text

<!-- SRD page 67 -->

Cost: 1 Sorcery Point\
When you roll damage for a spell, you can spend 1\
Sorcery Point to reroll a number of the damage dice\
up to your Charisma modifier (minimum of one),\
and you must use the new rolls.\
You can use Empowered Spell even if you’ve al-\
ready used a different Metamagic option during the\
casting of the spell.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sorcerer_empowered_spell ` |
| ` implementation ` | ` dnd2024:rules_sorcerer_empowered_spell ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` spell_damage_roll ` |
| ` compatible_with_other_metamagic ` | ` true ` |

#### Cost

<!-- splat:values path=splat_rule/parameters/cost -->

| Field | Value |
| --- | --- |
| ` sorcery_points ` | ` 1 ` |

#### Reroll dice

<!-- splat:yaml path=splat_rule/parameters/reroll_dice -->

```yaml
selection: caster_choice
maximum:
  op: max
  args:
  - 1
  - var: subject.charisma_modifier
use_new_results: true
```
