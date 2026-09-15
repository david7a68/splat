---
splat_schema: 1
splat_id: 5d2d566f-ec0b-5790-82d5-550176a81d5f
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_languages
splat_name: Languages
splat_origin:
  key: dnd2024:reference/pages/256
splat_sources:
- title: SRD 5.2.1
  locator: pp. 256
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_languages
  parameters:
    language_entry_modes:
    - communicate
    - understand_only
    - none
    none:
      understands_any_language: false
    understand_only:
      can_communicate_in_language: false
  implementation: dnd2024:rules_parts_of_a_stat_block_languages
splat_contracts:
- id: dnd2024:rules_parts_of_a_stat_block_languages
  inputs:
    definition:
      type: definition
      required: true
  outputs:
  - typed_attributes
  events: []
  capabilities:
  - definition_catalogs
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 256
  engine_gaps: &id001
  - dnd2024:rules_parts_of_a_stat_block_languages
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: b0393b28c86e97abf4b1e464a08a2885eadade777db0e5c76d7c86c19a2c4aa4
  parent_name: Parts of a Stat Block
  line_span:
    first: 11069
    count: 6
splat_clauses:
- key: source_rules
  text: '# Languages This entry lists languages that the monster can use to communicate. Sometimes a monster
    can understand a language but can’t communicate with it, which is noted in its entry. “None” indicates
    that a creature doesn’t comprehend any language.'
  sha256: 4097cb53a3776c5f60395aa4785343f14d231308bb160f4b0ba02b0e554973fa
  status: contract_specified
splat_requires_mechanics: *id001
---

# Languages

<!-- SRD page 256 -->

This entry lists languages that the monster can use\
to communicate. Sometimes a monster can under-\
stand a language but can’t communicate with it,\
which is noted in its entry. “None” indicates that a\
creature doesn’t comprehend any language.\
