---
splat_schema: 1
splat_id: ac881dc0-135f-5b7f-8b1f-5bb10eee10f0
splat_kind: rule
splat_key: dnd2024:rules/sentient_magic_items_communication
splat_name: Communication
splat_origin:
  key: dnd2024:reference/pages/207
splat_sources:
- title: SRD 5.2.1
  locator: pp. 207
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sentient_magic_items_communication
  parameters:
    selection:
    - gm_choice
    - random_roll
    die:
      count: 1
      sides: 10
    table:
    - min: 1
      max: 6
      modes:
      - transmit_emotions
      recipient: carrying_or_wielding_creature
    - min: 7
      max: 9
      modes:
      - speech
      language_count_min: 1
      languages: gm_choice
    - min: 10
      max: 10
      modes:
      - speech
      - telepathy
      language_count_min: 1
      languages: gm_choice
      telepathy_recipient: carrying_or_wielding_creature
  implementation: dnd2024:rules_sentient_magic_items_communication
splat_contracts:
- id: dnd2024:rules_sentient_magic_items_communication
  inputs:
    item:
      type: item_instance
      required: true
    communication_choice:
      type: choice
      required: true
    languages:
      required: false
      type: choice
  outputs:
  - item_communication
  events: []
  capabilities:
  - sentient_items
  - random_tables
  - languages
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 207
  engine_gaps: &id001
  - dnd2024:rules_sentient_magic_items_communication
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 3c57ecf730689a34a7aa8663f460bbad9befaca9964426fe2e05a7f8e7397123
  parent_name: Sentient Magic Items
  line_span:
    first: 10762
    count: 6
splat_clauses:
- key: source_rules
  text: '# Communication A sentient item communicates by sharing its emotions, broadcasting its thoughts
    telepathically, or speaking aloud. You can choose how it communicates or roll on the Sentient Item’s
    Communication table.'
  sha256: bac81006976c2446ff499ffae7c66e01ac4e30a778ffe80ccf13d5b9e896145a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Communication

<!-- SRD page 207 -->

A sentient item communicates by sharing its emo-\
tions, broadcasting its thoughts telepathically, or\
speaking aloud. You can choose how it communi-\
cates or roll on the Sentient Item’s Communication\
table.\
