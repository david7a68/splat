---
splat_schema: 1
splat_id: 4b4f5afd-f734-5282-98ce-11980ff04483
splat_kind: rule
splat_key: dnd2024:rules/crafting_magic_items_arcana_proficiency
splat_name: Arcana Proficiency
splat_origin:
  key: dnd2024:reference/pages/206
splat_sources:
- title: SRD 5.2.1
  locator: pp. 206
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/crafting_magic_items_arcana_proficiency
  parameters:
    arcana_proficiency_required_for:
    - crafter
    - all_assistants
  implementation: dnd2024:rule_crafting_magic_items_arcana_proficiency
splat_contracts:
- id: dnd2024:rule_crafting_magic_items_arcana_proficiency
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - crafted_items
  events: []
  capabilities:
  - crafting
  - proficiency
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 206
  engine_gaps: &id001
  - dnd2024:rule_crafting_magic_items_arcana_proficiency
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 51008a639b8d73161117717a43b92a08d0c2eac622b327a1109c9b090a3a3d23
  parent_name: Crafting Magic Items
  line_span:
    first: 10664
    count: 3
splat_clauses:
- key: source_rules
  text: '# Arcana Proficiency To craft a magic item, you and any assistants must have proficiency in the
    Arcana skill.'
  sha256: fad5286188af80b5200828064386bfb67e73b1f211cedf5c11887afdfdc6322d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Arcana Proficiency

<!-- SRD page 206 -->

To craft a magic item, you and any assistants must\
have proficiency in the Arcana skill.\
