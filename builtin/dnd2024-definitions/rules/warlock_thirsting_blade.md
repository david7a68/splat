---
splat_schema: 1
splat_id: 35ea33e2-3a90-5e75-bedb-69b616681d60
splat_kind: rule
splat_key: dnd2024:rules/warlock_thirsting_blade
splat_name: Thirsting Blade
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_thirsting_blade
  parameters:
    prerequisites:
      warlock_level_minimum: 5
      invocation: pact_of_the_blade
    extra_attack:
      weapon: pact_weapon
      attack_count: 2
      trigger: actor.attack_action
      own_turn_only: true
  implementation: dnd2024:rules_warlock_thirsting_blade
splat_contracts:
- id: dnd2024:rules_warlock_thirsting_blade
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    choice:
      type: choice
      required: true
    event:
      required: false
      type: event
  outputs:
  - effects
  - resource_updates
  events: []
  capabilities:
  - class_features
  - effects
  - resources
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 74
  engine_gaps: &id001
  - dnd2024:rules_warlock_thirsting_blade
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: f1afdb30e4b04381e20ce211bc6c3935a184f0b09487a5973395a1bf442ae2f3
  parent_name: Warlock
  line_span:
    first: 6353
    count: 7
splat_clauses:
- key: source_rules
  text: '# Thirsting Blade Prerequisite: Level 5+ Warlock, Pact of the Blade Invocation You gain the Extra
    Attack feature for your pact weapon only. With that feature, you can attack twice with the weapon
    instead of once when you take the Attack action on your turn.'
  sha256: 555d6e101c4fa552935c7dcb522100c436df6e6e753018d104e3693dd3c1ed7c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Thirsting Blade

<!-- SRD page 74 -->

Prerequisite: Level 5+ Warlock, Pact of the Blade\
Invocation\
You gain the Extra Attack feature for your pact\
weapon only. With that feature, you can attack\
twice with the weapon instead of once when you\
take the Attack action on your turn.\
