---
splat_schema: 1
splat_id: 817b77a6-8def-514e-b530-416e9ede4a56
splat_kind: rule
splat_key: dnd2024:rules/magic_items_multiple_items_of_the_same_kind
splat_name: Multiple Items of the Same Kind
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magic_items_multiple_items_of_the_same_kind
  parameters:
    worn_category_limits:
      footwear_pair: 1
      gloves_or_gauntlets_pair: 1
      bracers_pair: 1
      armor_suit: 1
      headwear: 1
      cloak: 1
    exceptions:
      actor: gm
      scope: wearing_multiple_items
  implementation: dnd2024:rule_magic_items_multiple_items_of_the_same_kind
splat_contracts:
- id: dnd2024:rule_magic_items_multiple_items_of_the_same_kind
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
  - equipment
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 103
  engine_gaps: &id001
  - dnd2024:rule_magic_items_multiple_items_of_the_same_kind
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 09f0baaa6f338a690d177459e0b1f25e9482af21777dc3a67ae90694bb65f298
  parent_name: Magic Items
  line_span:
    first: 8947
    count: 6
splat_clauses:
- key: source_rules
  text: '# Multiple Items of the Same Kind You can’t wear more than one of certain magic items. You can’t
    normally wear more than one pair of footwear, one pair of gloves or gauntlets, one pair of bracers,
    one suit of armor, one item of headwear, or one cloak. The GM might make exceptions.'
  sha256: bf4a6aaf98c22f19bea3e93aad2b216e7d42c2cdf17220391dd535414a1accfc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Multiple Items of the Same Kind

<!-- SRD page 103 -->

You can’t wear more than one of certain magic\
items. You can’t normally wear more than one pair\
of footwear, one pair of gloves or gauntlets, one pair\
of bracers, one suit of armor, one item of headwear,\
or one cloak. The GM might make exceptions.\
