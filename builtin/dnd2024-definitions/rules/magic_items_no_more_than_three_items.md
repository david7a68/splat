---
splat_schema: 1
splat_id: 4ff66ee4-42dc-5665-b6da-a29b38a97302
splat_kind: rule
splat_key: dnd2024:rules/magic_items_no_more_than_three_items
splat_name: No More Than Three Items
splat_origin:
  key: dnd2024:reference/pages/102
splat_sources:
- title: SRD 5.2.1
  locator: pp. 102
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magic_items_no_more_than_three_items
  parameters:
    attuned_items_maximum: 3
    fourth_attempt: fails
    same_definition_copies_attuned_maximum: 1
    must_end_existing_before_new_if_at_limit: true
  implementation: dnd2024:rule_magic_items_no_more_than_three_items
splat_contracts:
- id: dnd2024:rule_magic_items_no_more_than_three_items
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - item_changes
  events: []
  capabilities:
  - magic_items
  - attunement
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 102
  engine_gaps: &id001
  - dnd2024:rule_magic_items_no_more_than_three_items
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: c6e3f938ea9965dbbad34e20a5b6c5a6937e5902371750814ff63beb5557c3bf
  parent_name: Magic Items
  line_span:
    first: 8920
    count: 7
splat_clauses:
- key: source_rules
  text: '# No More Than Three Items You can be attuned to no more than three magic items at a time. Any
    attempt to attune to a fourth item fails; you must end your Attunement to an item first. Additionally,
    you can’t attune to more than one copy of an item. For example, you can’t attune to more than one
    Ring of Protection at a time.'
  sha256: 86137475c8ff0787f35330b1cce7d1d5ebe903219d7271dbc027abd6b8e5a48b
  status: contract_specified
splat_requires_mechanics: *id001
---

# No More Than Three Items

<!-- SRD page 102 -->

You can be attuned to no more than three magic\
items at a time. Any attempt to attune to a fourth\
item fails; you must end your Attunement to an item\
first. Additionally, you can’t attune to more than\
one copy of an item. For example, you can’t attune to\
more than one Ring of Protection at a time.\
