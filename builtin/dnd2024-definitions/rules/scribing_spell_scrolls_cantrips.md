---
splat_schema: 1
splat_id: 1c510fca-c984-5d0a-80e7-6a53fa65f24e
splat_kind: rule
splat_key: dnd2024:rules/scribing_spell_scrolls_cantrips
splat_name: Cantrips
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/scribing_spell_scrolls_cantrips
  parameters:
    cantrip_effective_caster_level: scribe_level
    scroll_costs:
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
  implementation: dnd2024:rules_scribing_spell_scrolls_cantrips
splat_contracts:
- id: dnd2024:rules_scribing_spell_scrolls_cantrips
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      type: choice
      required: true
    event:
      required: false
      type: event
  outputs:
  - item_updates
  - cost
  - effects
  events: []
  capabilities:
  - items
  - attunement
  - crafting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 103
  engine_gaps: &id001
  - dnd2024:rules_scribing_spell_scrolls_cantrips
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: e3b2609ce96934346ecb1515762523baa1a77f27ac2755542ad1c5840bcf6c6d
  parent_name: Scribing Spell Scrolls
  line_span:
    first: 9014
    count: 15
splat_clauses:
- key: source_rules
  text: '# Cantrips If the scribed spell is a cantrip, the version on the scroll works as if the caster
    were your level. Spell Scroll Costs Spell Level Time Cost Cantrip 1 day 15 GP 1 1 day 25 GP 2 3 days
    100 GP 3 5 days 150 GP 4 10 days 1,000 GP 5 25 days 1,500 GP 6 40 days 10,000 GP 7 50 days 12,500
    GP 8 60 days 15,000 GP 9 120 days 50,000 GP'
  sha256: 4851db75edfb56c1376543731dec85664baf2996ee39c5a93411b7a9f1bb7d5b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cantrips

<!-- SRD page 103 -->

If the scribed spell is a cantrip, the version on the\
scroll works as if the caster were your level.\
Spell Scroll Costs\
Spell Level Time Cost\
Cantrip 1 day 15 GP\
1 1 day 25 GP\
2 3 days 100 GP\
3 5 days 150 GP\
4 10 days 1,000 GP\
5 25 days 1,500 GP\
6 40 days 10,000 GP\
7 50 days 12,500 GP\
8 60 days 15,000 GP\
9 120 days 50,000 GP\
