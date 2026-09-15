---
splat_schema: 1
splat_id: 860e2c08-259a-50f4-8a34-90f43d087ea7
splat_kind: rule
splat_key: dnd2024:rules/weapons_thrown
splat_name: Thrown
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_thrown
  parameters:
    ranged_attack_allowed: true
    draw_as_part_of_attack: true
    melee_thrown_ability_modifier: same_as_melee_attack_for_attack_and_damage
  implementation: dnd2024:rules_weapons_thrown
splat_contracts:
- id: dnd2024:rules_weapons_thrown
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    event:
      type: event
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - action_options
  - effects
  - resource_updates
  events: []
  capabilities:
  - action_economy
  - movement
  - combat
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 90
  engine_gaps: &id001
  - dnd2024:rules_weapons_thrown
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 0dadb94bb4f55bf4a93843f889e5ebc9ee6f4dd45363c6135e0219700558e6fd
  parent_name: Weapons
  line_span:
    first: 7746
    count: 7
splat_clauses:
- key: source_rules
  text: '# Thrown If a weapon has the Thrown property, you can throw the weapon to make a ranged attack,
    and you can draw that weapon as part of the attack. If the weapon is a Melee weapon, use the same
    ability modifier for the attack and damage rolls that you use for a melee attack with that weapon.'
  sha256: 0e94da29b22b2087a67571b0bc6c2dd422751e7a6bfb68915ea2aa8b2a84185d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Thrown

<!-- SRD page 90 -->

If a weapon has the Thrown property, you can\
throw the weapon to make a ranged attack, and\
you can draw that weapon as part of the attack. If\
the weapon is a Melee weapon, use the same ability\
modifier for the attack and damage rolls that you\
use for a melee attack with that weapon.\
