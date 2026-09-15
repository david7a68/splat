---
splat_schema: 1
splat_id: 47b49534-499b-5993-b5f5-1094037db414
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_attack_notation
splat_name: Attack Notation
splat_origin:
  key: dnd2024:reference/pages/256
splat_sources:
- title: SRD 5.2.1
  locator: pp. 256
  url: https://www.dndbeyond.com/srd
---

# Attack Notation

## Source text

<!-- SRD page 256 -->

The entry for a monster’s attack identifies whether\
the attack is a melee or a ranged attack and then\
provides the attack roll’s bonus, its reach or range,\
and what happens on a hit. An attack is against one\
target unless its entry says otherwise. For details\
on different kinds of attacks, see “Playing the Game”\
and “Spells.”\
Hit. Any damage dealt or other effects that occur\
as a result of an attack hitting a target are described\
after the “Hit:” notation.\
Miss. If an attack has an effect that occurs on a\
miss, that information follows the “Miss:” notation.\
Hit or Miss. If an attack has an effect that occurs\
regardless of whether it hits or misses its target,\
that information follows the “Hit or Miss:” notation.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_attack_notation ` |
| ` implementation ` | ` dnd2024:rules_parts_of_a_stat_block_attack_notation ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attack_fields ` | ` ["melee_or_ranged", "attack_bonus", "reach_or_range", "hit_effects"] ` |
| ` default_targets ` | ` 1 ` |
| ` specific_target_count_overrides ` | ` true ` |

#### Notation branches

<!-- splat:values path=splat_rule/parameters/notation_branches -->

| Field | Value |
| --- | --- |
| ` hit ` | ` only_on_hit ` |
| ` miss ` | ` only_on_miss ` |
| ` hit_or_miss ` | ` always_after_attack_roll ` |
