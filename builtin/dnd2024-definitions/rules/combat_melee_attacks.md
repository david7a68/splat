---
splat_schema: 1
splat_id: 312d9860-b8b9-5e10-bb12-46abc16bcdc7
splat_kind: rule
splat_key: dnd2024:rules/combat_melee_attacks
splat_name: Melee Attacks
splat_origin:
  key: dnd2024:reference/pages/015
splat_sources:
- title: SRD 5.2.1
  locator: pp. 15
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_melee_attacks
  parameters:
    target_distance_maximum: attacker_reach
    attack_kind: melee
  implementation: dnd2024:rules_combat_melee_attacks
splat_contracts:
- id: dnd2024:rules_combat_melee_attacks
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
  - 15
  engine_gaps: &id001
  - dnd2024:rules_combat_melee_attacks
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: a3ea52add86a853d9229b4a0c0870b2ec7cc0e3090d3d86437ceeb01a05a6133
  parent_name: Combat
  line_span:
    first: 1030
    count: 6
splat_clauses:
- key: source_rules
  text: '# Melee Attacks A melee attack allows you to attack a target within your reach. A melee attack
    typically uses a handheld weapon or an Unarmed Strike. Many monsters make melee attacks with claws,
    teeth, or other body parts. A few spells also involve melee attacks.'
  sha256: 509d71ba218a36a0cb69628f5a3e81f5cf6ddd3663e2e8a8c9d1264b0c704740
  status: contract_specified
splat_requires_mechanics: *id001
---

# Melee Attacks

<!-- SRD page 15 -->

A melee attack allows you to attack a target within\
your reach. A melee attack typically uses a hand-\
held weapon or an Unarmed Strike. Many monsters\
make melee attacks with claws, teeth, or other body\
parts. A few spells also involve melee attacks.\
