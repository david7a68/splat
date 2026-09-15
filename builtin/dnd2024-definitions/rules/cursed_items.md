---
splat_schema: 1
splat_id: becbde68-276f-5795-8d7d-f9bdba546ba7
splat_kind: rule
splat_key: dnd2024:rules/cursed_items
splat_name: Cursed Items
splat_origin:
  key: dnd2024:reference/pages/206
splat_sources:
- title: SRD 5.2.1
  locator: pp. 206
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/cursed_items
  parameters:
    curse_presence_source: item_description
    identification_reveals_curse_default: false
    identify_spell_reveals_curse: false
    voluntary_attunement_end_requires: curse_broken
    curse_removal_example:
      key: dnd2024:spells/remove_curse
  implementation: dnd2024:rule_cursed_items
splat_contracts:
- id: dnd2024:rule_cursed_items
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
  - curses
  - attunement
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 206
  engine_gaps: &id001
  - dnd2024:rule_cursed_items
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 2adf92b6294b03d2f6804b53d4a3ea8900c4d5b26b016147fe31972223782d21
  parent_name: Cursed Items
  line_span:
    first: 10643
    count: 7
splat_clauses:
- key: source_rules
  text: '# Cursed Items A magic item’s description specifies whether it bears a curse. Most methods of
    identifying items, including the Identify spell, fail to reveal such a curse. Attunement to a cursed
    item can’t be ended voluntarily unless the curse is broken first, such as with a Remove Curse spell.'
  sha256: e9689f616d03b0d0be84145bfe5abbfd4859b621301d13f70811e19a3f5acbd3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cursed Items

<!-- SRD page 206 -->

A magic item’s description specifies whether it\
bears a curse. Most methods of identifying items, in-\
cluding the Identify spell, fail to reveal such a curse.\
Attunement to a cursed item can’t be ended volun-\
tarily unless the curse is broken first, such as with a\
Remove Curse spell.\
