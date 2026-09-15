---
splat_schema: 1
splat_id: 3d15486b-5227-541e-9b4f-8ccd165ba650
splat_kind: rule
splat_key: dnd2024:rules/sentient_magic_items_senses
splat_name: Senses
splat_origin:
  key: dnd2024:reference/pages/207
splat_sources:
- title: SRD 5.2.1
  locator: pp. 207
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sentient_magic_items_senses
  parameters:
    generation_choice:
    - gm_select
    - random
    roll:
      dice:
        count: 1
        sides: 4
    results:
    - roll: 1
      hearing_feet: 30
      standard_vision_feet: 30
    - roll: 2
      hearing_feet: 60
      standard_vision_feet: 60
    - roll: 3
      hearing_feet: 120
      standard_vision_feet: 120
    - roll: 4
      hearing_feet: 120
      darkvision_feet: 120
  implementation: dnd2024:rule_sentient_magic_items_senses
splat_contracts:
- id: dnd2024:rule_sentient_magic_items_senses
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - item_changes
  events: []
  capabilities:
  - magic_items
  - senses
  - random_tables
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 207
  engine_gaps: &id001
  - dnd2024:rule_sentient_magic_items_senses
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 26b0714cc453e77d6f804eb0edf2bb55ba13ad64197198ebd97d5320d3faa785
  parent_name: Sentient Magic Items
  line_span:
    first: 10768
    count: 4
splat_clauses:
- key: source_rules
  text: '# Senses A sentient item can perceive its surroundings out to a limited range. You can choose
    its senses or roll on the Sentient Item’s Senses table.'
  sha256: 5b5b6ca443d9712c5a7f9e618275954a4ab155bb1dc872c69de84641ecbacfba
  status: contract_specified
splat_requires_mechanics: *id001
---

# Senses

<!-- SRD page 207 -->

A sentient item can perceive its surroundings out to\
a limited range. You can choose its senses or roll on\
the Sentient Item’s Senses table.\
