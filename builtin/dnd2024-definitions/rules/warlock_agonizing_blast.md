---
splat_schema: 1
splat_id: 5ce9cc9b-4f04-5b0e-b199-bd01003a003d
splat_kind: rule
splat_key: dnd2024:rules/warlock_agonizing_blast
splat_name: Agonizing Blast
splat_origin:
  key: dnd2024:reference/pages/072
splat_sources:
- title: SRD 5.2.1
  locator: pp. 72
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_agonizing_blast
  parameters:
    prerequisites:
      warlock_level_minimum: 2
      known_warlock_damaging_cantrip: true
    choice:
      count: 1
      from: known_warlock_damaging_cantrips
    damage_roll_bonus: charisma_modifier
    repeatable: true
    repeat_choice_must_differ: true
  implementation: dnd2024:rules_warlock_agonizing_blast
splat_contracts:
- id: dnd2024:rules_warlock_agonizing_blast
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
  - 72
  engine_gaps: &id001
  - dnd2024:rules_warlock_agonizing_blast
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 258469dd449be9319828e61585e430c673c0cf7f110322d1007b9e22d89bf26c
  parent_name: Warlock
  line_span:
    first: 6144
    count: 9
splat_clauses:
- key: source_rules
  text: '# Agonizing Blast Prerequisite: Level 2+ Warlock, a Warlock Cantrip That Deals Damage Choose
    one of your known Warlock cantrips that deals damage. You can add your Charisma modifier to that spell’s
    damage rolls. Repeatable. You can gain this invocation more than once. Each time you do so, choose
    a different eligible cantrip.'
  sha256: a32857e6f15b628b432024fa12da9926d062c38bfd3d884094a674474bc9b01c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Agonizing Blast

<!-- SRD page 72 -->

Prerequisite: Level 2+ Warlock, a Warlock Cantrip\
That Deals Damage\
Choose one of your known Warlock cantrips that\
deals damage. You can add your Charisma modifier\
to that spell’s damage rolls.\
Repeatable. You can gain this invocation more\
than once. Each time you do so, choose a different\
eligible cantrip.\
