---
splat_schema: 1
splat_id: 3bb900e6-1df4-5633-b918-8ccf28dfdbea
splat_kind: rule
splat_key: dnd2024:rules/magic_item_categories_staffs
splat_name: Staffs
splat_origin:
  key: dnd2024:reference/pages/205
splat_sources:
- title: SRD 5.2.1
  locator: pp. 205
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magic_item_categories_staffs
  parameters:
    category: staff
    weight_pounds_range:
    - 2
    - 7
    default_uses:
    - weapon:
        key: dnd2024:equipment/quarterstaff
      magical_weapon: false
    - focus: arcane
    description_exception_overrides: true
  implementation: dnd2024:rule_magic_item_categories_staffs
splat_contracts:
- id: dnd2024:rule_magic_item_categories_staffs
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
  - 205
  engine_gaps: &id001
  - dnd2024:rule_magic_item_categories_staffs
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 211ba2f579712e072d1c61fbb0b9a01e2aab3be56a4662112eec2476e69aa575
  parent_name: Magic Item Categories
  line_span:
    first: 10497
    count: 10
splat_clauses:
- key: source_rules
  text: '# Staffs Items in the Staff category vary widely in appearance: some are of nearly equal diameter
    throughout and smooth, others are gnarled and twisted, some are made of wood, and others are composed
    of polished metal or crystal. A staff weighs between 2 and 7 pounds and serves well as a walking stick
    or cane. Unless its description notes otherwise, a staff can be used as a nonmagical Quarterstaff
    and an Arcane Focus.'
  sha256: 81df51ea5e05976a0485c1b40eb97f70ede34d7f9ca9bf92edc9e8e76fc72d73
  status: contract_specified
splat_requires_mechanics: *id001
---

# Staffs

<!-- SRD page 205 -->

Items in the Staff category vary widely in appear-\
ance: some are of nearly equal diameter throughout\
and smooth, others are gnarled and twisted, some\
are made of wood, and others are composed of pol-\
ished metal or crystal. A staff weighs between 2 and\
7 pounds and serves well as a walking stick or cane.\
Unless its description notes otherwise, a staff can\
be used as a nonmagical Quarterstaff and an Arcane\
Focus.\
