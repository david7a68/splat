---
splat_schema: 1
splat_id: 07122e38-0ba9-58cb-b12f-52525bac2cf0
splat_kind: rule
splat_key: dnd2024:rules/magic_items_attunement
splat_name: Attunement
splat_origin:
  key: dnd2024:reference/pages/102
splat_sources:
- title: SRD 5.2.1
  locator: pp. 102
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magic_items_attunement
  parameters:
    requires_attunement_item:
      magical_benefits_require_attunement: true
      nonmagical_benefits_require_attunement: false
      item_specific_exception_allowed: true
  implementation: dnd2024:rules_magic_items_attunement
splat_contracts:
- id: dnd2024:rules_magic_items_attunement
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
  - 102
  engine_gaps: &id001
  - dnd2024:rules_magic_items_attunement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 35c60a195b8220e33cbbe1a742ea53bb179b8c16adf3df84668ce1d709402089
  parent_name: Magic Items
  line_span:
    first: 8899
    count: 10
splat_clauses:
- key: source_rules
  text: '# Attunement Some magic items require a creature to form a bond—called Attunement—with them before
    the creature can use an item’s magical properties. Without becoming attuned to an item that requires
    Attunement, you gain only its nonmagical benefits unless its description states otherwise. For example,
    a magic Shield that requires Attunement provides the benefits of a normal Shield if you aren’t attuned
    to it, but none of its magical properties.'
  sha256: 2e53cd73d5e92a10dbc26dc4b1fe3abce1bb55e92936293538d4ca32a92e8288
  status: contract_specified
splat_requires_mechanics: *id001
---

# Attunement

<!-- SRD page 102 -->

Some magic items require a creature to form a\
bond—called Attunement—with them before the\
creature can use an item’s magical properties. With-\
out becoming attuned to an item that requires At-\
tunement, you gain only its nonmagical benefits un-\
less its description states otherwise. For example, a\
magic Shield that requires Attunement provides the\
benefits of a normal Shield if you aren’t attuned to\
it, but none of its magical properties.\
