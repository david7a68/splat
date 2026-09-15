---
splat_schema: 1
splat_id: 4c950cfd-53fd-5274-8288-6b3c76a44b29
splat_kind: rule
splat_key: dnd2024:rules/warlock_eldritch_spear
splat_name: Eldritch Spear
splat_origin:
  key: dnd2024:reference/pages/073
splat_sources:
- title: SRD 5.2.1
  locator: pp. 73
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_eldritch_spear
  parameters:
    prerequisites:
      warlock_level_min: 2
    selection:
      count: 1
      known: true
      class: warlock
      level: 0
      deals_damage: true
      range_min_ft: 10
    range_increase_ft:
      op: multiply
      args:
      - 30
      - var: subject.warlock_level
    repeatable: true
    repeat_selection: different_eligible_cantrip
  implementation: dnd2024:rules_warlock_eldritch_spear
splat_contracts:
- id: dnd2024:rules_warlock_eldritch_spear
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - spell_range_modifier
  events: []
  capabilities:
  - character_options
  - spellcasting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 73
  engine_gaps: &id001
  - dnd2024:rules_warlock_eldritch_spear
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: de13321087268ab54c98dc5c97becdbed3d2909a788744886b8de02c2a02e722
  parent_name: Warlock
  line_span:
    first: 6182
    count: 10
splat_clauses:
- key: source_rules
  text: '# Eldritch Spear Prerequisite: Level 2+ Warlock, a Warlock Cantrip That Deals Damage Choose one
    of your known Warlock cantrips that deals damage and has a range of 10+ feet. When you cast that spell,
    its range increases by a number of feet equal to 30 times your Warlock level. Repeatable. You can
    gain this invocation more than once. Each time you do so, choose a different eligible cantrip.'
  sha256: fa9f612c12e79480577ca0b4ec2245e8eb2d5405c94a70254875c0dc1f323c17
  status: contract_specified
splat_requires_mechanics: *id001
---

# Eldritch Spear

<!-- SRD page 73 -->

Prerequisite: Level 2+ Warlock, a Warlock Cantrip\
That Deals Damage\
Choose one of your known Warlock cantrips that\
deals damage and has a range of 10+ feet. When you\
cast that spell, its range increases by a number of\
feet equal to 30 times your Warlock level.\
Repeatable. You can gain this invocation more\
than once. Each time you do so, choose a different\
eligible cantrip.\
