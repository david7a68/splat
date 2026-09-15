---
splat_schema: 1
splat_id: c64adbe1-134f-520f-8dbb-a1702feee4d9
splat_kind: rule
splat_key: dnd2024:rules/scribing_spell_scrolls_time_and_cost
splat_name: Time and Cost
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/scribing_spell_scrolls_time_and_cost
  parameters:
    work_hours_per_day: 8
    days_must_be_consecutive: false
    table:
    - spell_level: 0
      days: 1
      cost_gp: 15
    - spell_level: 1
      days: 1
      cost_gp: 25
    - spell_level: 2
      days: 3
      cost_gp: 100
    - spell_level: 3
      days: 5
      cost_gp: 150
    - spell_level: 4
      days: 10
      cost_gp: 1000
    - spell_level: 5
      days: 25
      cost_gp: 1500
    - spell_level: 6
      days: 40
      cost_gp: 10000
    - spell_level: 7
      days: 50
      cost_gp: 12500
    - spell_level: 8
      days: 60
      cost_gp: 15000
    - spell_level: 9
      days: 120
      cost_gp: 50000
  implementation: dnd2024:rules_scribing_spell_scrolls_time_and_cost
splat_contracts:
- id: dnd2024:rules_scribing_spell_scrolls_time_and_cost
  inputs:
    spell:
      type: definition
      required: true
  outputs:
  - scribe_time
  - scribe_cost
  events: []
  capabilities:
  - crafting
  - spell_scrolls
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 103
  engine_gaps: &id001
  - dnd2024:rules_scribing_spell_scrolls_time_and_cost
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: e4b916232eabb0c8a1d334a98f87e192995a004a7c640a266dc14188d2a9de90
  parent_name: Scribing Spell Scrolls
  line_span:
    first: 8999
    count: 6
splat_clauses:
- key: source_rules
  text: '# Time and Cost Scribing a scroll takes an amount of time and money based on the level of the
    spell, as shown in the Spell Scroll Costs table. For each day of inscription, you must work for 8
    hours. If a scroll requires multiple days, those days needn’t be consecutive.'
  sha256: 62111812a3459e77b3c61c3291b62d301aea9b2bd876ba75707abffb8132a26f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Time and Cost

<!-- SRD page 103 -->

Scribing a scroll takes an amount of time and money\
based on the level of the spell, as shown in the Spell\
Scroll Costs table. For each day of inscription, you\
must work for 8 hours. If a scroll requires multiple\
days, those days needn’t be consecutive.\
