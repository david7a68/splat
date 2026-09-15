---
splat_schema: 1
splat_id: 9c19c257-7d4f-5754-aed2-0b8870ad2861
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_resistances_and_vulnerabilities
splat_name: Resistances and Vulnerabilities
splat_origin:
  key: dnd2024:reference/pages/255
splat_sources:
- title: SRD 5.2.1
  locator: pp. 255
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_resistances_and_vulnerabilities
  parameters:
    damage_modifiers_source: stat_block_resistance_and_vulnerability_entries
    damage_rule:
      key: dnd2024:rules/damage_and_healing_resistance_and_vulnerability
    combat_tactics_guidance:
      special_limited_abilities: as_early_and_often_as_possible
      multiattack: unless_more_powerful_ability_used
      bonus_reaction_legendary_actions: as_often_as_possible
  implementation: dnd2024:rules_parts_of_a_stat_block_resistances_and_vulnerabilities
splat_contracts:
- id: dnd2024:rules_parts_of_a_stat_block_resistances_and_vulnerabilities
  inputs:
    monster:
      type: participant
      required: true
    damage:
      type: resolution
      required: true
  outputs:
  - damage_modifiers
  - tactics
  events: []
  capabilities:
  - damage
  - monster_actions
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 255
  engine_gaps: &id001
  - dnd2024:rules_parts_of_a_stat_block_resistances_and_vulnerabilities
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: a8d25d73cfb4dcd45066d646ec7e7a62af399fabea98cd9e4427d3a9bf1d78fb
  parent_name: Parts of a Stat Block
  line_span:
    first: 11006
    count: 20
splat_clauses:
- key: source_rules
  text: '# Resistances and Vulnerabilities These entries list a monster’s Resistances and Vulnerabilities,
    if any. See “Playing the Game” for more information. Running a MonsteR To ensure a monster acts in
    accordance with its Challenge Rating, follow these rules during combat: Special Abilities. If the
    monster has a special ability that deals a lot of damage but has a limited number of uses, such as
    a recharging breath weapon or a spell it can cast only once per day, have it use that special ability
    as quickly and as often as possible. Multiattack. If the monster has Multiattack, have it use Multiattack
    on any of its turns in which it’s not using one of its more powerful abilities. Bonus Actions, Reactions,
    Legendary Actions. If the monster has Bonus Actions, Reactions, or Legendary Actions in its stat block,
    make sure it uses them as often as it can.'
  sha256: 58ebf604edd32160c5aefd3ab7bb54ac067b26b0da053d0cca6566cbd1fc1216
  status: contract_specified
splat_requires_mechanics: *id001
---

# Resistances and Vulnerabilities

<!-- SRD page 255 -->

These entries list a monster’s Resistances and Vul-\
nerabilities, if any. See “Playing the Game” for more\
information.\
Running a MonsteR\
To ensure a monster acts in accordance with its\
Challenge Rating, follow these rules during combat:\
Special Abilities. If the monster has a special abil-\
ity that deals a lot of damage but has a limited\
number of uses, such as a recharging breath\
weapon or a spell it can cast only once per day,\
have it use that special ability as quickly and as\
often as possible.\
Multiattack. If the monster has Multiattack, have\
it use Multiattack on any of its turns in which\
it’s not using one of its more powerful abilities.\
Bonus Actions, Reactions, Legendary Actions. If\
the monster has Bonus Actions, Reactions, or\
Legendary Actions in its stat block, make sure it\
uses them as often as it can.\
