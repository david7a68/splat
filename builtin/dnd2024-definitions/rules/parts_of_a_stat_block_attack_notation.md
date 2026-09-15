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
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_attack_notation
  parameters:
    attack_fields:
    - melee_or_ranged
    - attack_bonus
    - reach_or_range
    - hit_effects
    default_targets: 1
    specific_target_count_overrides: true
    notation_branches:
      hit: only_on_hit
      miss: only_on_miss
      hit_or_miss: always_after_attack_roll
  implementation: dnd2024:rules_parts_of_a_stat_block_attack_notation
splat_contracts:
- id: dnd2024:rules_parts_of_a_stat_block_attack_notation
  inputs:
    attack:
      type: definition
      required: true
    resolution:
      type: resolution
      required: true
  outputs:
  - attack_effects
  events: []
  capabilities:
  - monster_attacks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 256
  engine_gaps: &id001
  - dnd2024:rules_parts_of_a_stat_block_attack_notation
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 92925a7f9d71279bd3f975ce6b61a2ecb661a12668e007ad6eaa86f661a5aadf
  parent_name: Parts of a Stat Block
  line_span:
    first: 11132
    count: 16
splat_clauses:
- key: source_rules
  text: '# Attack Notation The entry for a monster’s attack identifies whether the attack is a melee or
    a ranged attack and then provides the attack roll’s bonus, its reach or range, and what happens on
    a hit. An attack is against one target unless its entry says otherwise. For details on different kinds
    of attacks, see “Playing the Game” and “Spells.” Hit. Any damage dealt or other effects that occur
    as a result of an attack hitting a target are described after the “Hit:” notation. Miss. If an attack
    has an effect that occurs on a miss, that information follows the “Miss:” notation. Hit or Miss. If
    an attack has an effect that occurs regardless of whether it hits or misses its target, that information
    follows the “Hit or Miss:” notation.'
  sha256: 9547fbf966f65cafd45b5cc0345498ec48ab7a34ef506289f0a92177a1936577
  status: contract_specified
splat_requires_mechanics: *id001
---

# Attack Notation

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
