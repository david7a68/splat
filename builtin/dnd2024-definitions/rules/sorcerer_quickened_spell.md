---
splat_schema: 1
splat_id: acc8a3a9-1c08-5c1c-8569-ebb1df44a31c
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_quickened_spell
splat_name: Quickened Spell
splat_origin:
  key: dnd2024:reference/pages/067
splat_sources:
- title: SRD 5.2.1
  locator: pp. 67
  url: https://www.dndbeyond.com/srd
---

# Quickened Spell

## Source text

<!-- SRD page 67 -->

Cost: 2 Sorcery Points\
When you cast a spell that has a casting time of an\
action, you can spend 2 Sorcery Points to change\
the casting time to a Bonus Action for this casting.\
You can’t modify a spell in this way if you’ve already\
cast a level 1+ spell on the current turn, nor can you\
cast a level 1+ spell on this turn after modifying a\
spell in this way.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sorcerer_quickened_spell ` |
| ` implementation ` | ` dnd2024:rules_sorcerer_quickened_spell ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` casting_time_for_this_cast ` | ` bonus_action ` |
| ` forbid_later_level_1_plus_spell_cast_this_turn ` | ` true ` |

#### Cost

<!-- splat:values path=splat_rule/parameters/cost -->

| Field | Value |
| --- | --- |
| ` sorcery_points ` | ` 2 ` |

#### Requires

<!-- splat:values path=splat_rule/parameters/requires -->

| Field | Value |
| --- | --- |
| ` spell_casting_time ` | ` action ` |
| ` level_1_plus_spell_cast_this_turn ` | ` false ` |
