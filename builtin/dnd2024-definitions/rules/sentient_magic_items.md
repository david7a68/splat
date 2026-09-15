---
splat_schema: 1
splat_id: bf199fa5-ef53-5f73-83c3-ad4ff5d66cd7
splat_kind: rule
splat_key: dnd2024:rules/sentient_magic_items
splat_name: Sentient Magic Items
splat_origin:
  key: dnd2024:reference/pages/207
splat_sources:
- title: SRD 5.2.1
  locator: pp. 207
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sentient_magic_items
  parameters:
    sentience:
      single_use_item_allowed: false
      controller: gm
      activated_properties_controller: gm
      good_relationship: bearer_access
      strained_relationship: possible_conflict
    conflict_rules:
      key: dnd2024:rules/sentient_magic_items_conflict
  implementation: dnd2024:rules_sentient_magic_items
splat_contracts:
- id: dnd2024:rules_sentient_magic_items
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
  - 207
  engine_gaps: &id001
  - dnd2024:rules_sentient_magic_items
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: f28b42d71a78111f78f2386f1f7a8507f8cb8dedd8b6faaf8e9e18d9712e4c84
  parent_name: Sentient Magic Items
  line_span:
    first: 10733
    count: 15
splat_clauses:
- key: source_rules
  text: '# Sentient Magic Items Some magic items have sentience and personality. Such an item might be
    possessed, haunted by the spirit of a previous owner, or self-aware thanks to the magic used to create
    it. A sentient item might be a cherished ally to its wielder or a continual thorn in the side. Most
    sentient items are weapons, but other kinds of items can manifest sentience. Single-use items such
    as potions and scrolls are never sentient. The GM controls sentient magic items and their activated
    properties. A bearer who maintains a good relationship with the item can access those properties.
    If the relationship is strained, a conflict can ensue (see “Conflict” below).'
  sha256: 45dfb1dea0b2c555a6a51af393f22ef994350594180d925f7e94b5c1a515895b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sentient Magic Items

<!-- SRD page 207 -->

Some magic items have sentience and personality.\
Such an item might be possessed, haunted by the\
spirit of a previous owner, or self-aware thanks to\
the magic used to create it. A sentient item might be\
a cherished ally to its wielder or a continual thorn in\
the side.\
Most sentient items are weapons, but other kinds\
of items can manifest sentience. Single-use items\
such as potions and scrolls are never sentient.\
The GM controls sentient magic items and their\
activated properties. A bearer who maintains a\
good relationship with the item can access those\
properties. If the relationship is strained, a conflict\
can ensue (see “Conflict” below).\
