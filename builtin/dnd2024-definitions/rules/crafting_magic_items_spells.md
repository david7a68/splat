---
splat_schema: 1
splat_id: ebfb06ec-86bc-586d-83e5-15aa8d7d5e49
splat_kind: rule
splat_key: dnd2024:rules/crafting_magic_items_spells
splat_name: Spells
splat_origin:
  key: dnd2024:reference/pages/207
splat_sources:
- title: SRD 5.2.1
  locator: pp. 207
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/crafting_magic_items_spells
  parameters:
    if_item_grants_spell_casting:
      required_prepared_spells: all_spells_castable_from_item
      timing: each_crafting_day
  implementation: dnd2024:rules_crafting_magic_items_spells
splat_contracts:
- id: dnd2024:rules_crafting_magic_items_spells
  inputs:
    crafter:
      type: participant
      required: true
    item:
      type: definition
      required: true
  outputs:
  - crafting_eligibility
  events:
  - crafting.day_started
  capabilities:
  - crafting
  - spell_preparation
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 207
  engine_gaps: &id001
  - dnd2024:rules_crafting_magic_items_spells
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 2b4d9e3f1f55aa7350c0994b8f08534e0067b2635508247fd4b44fc4a03bcc12
  parent_name: Crafting Magic Items
  line_span:
    first: 10694
    count: 4
splat_clauses:
- key: source_rules
  text: '# Spells If a magic item allows its user to cast any spells from it, you must have all those
    spells prepared every day you spend crafting the item.'
  sha256: 994d0d113a00338cd59ddb40946d7e5c081da813af72272bc615c6589accef61
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spells

<!-- SRD page 207 -->

If a magic item allows its user to cast any spells\
from it, you must have all those spells prepared ev-\
ery day you spend crafting the item.\
