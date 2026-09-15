---
splat_schema: 1
splat_id: 7bf9199c-6917-5437-863d-b72a4d77bffe
splat_kind: rule
splat_key: dnd2024:rules/gaining_spells_always_prepared_spells
splat_name: Always-Prepared Spells
splat_origin:
  key: dnd2024:reference/pages/104
splat_sources:
- title: SRD 5.2.1
  locator: pp. 104
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/gaining_spells_always_prepared_spells
  parameters:
    always_prepared_spells_count_against_changeable_preparation_limit: false
  implementation: dnd2024:rules_gaining_spells_always_prepared_spells
splat_contracts:
- id: dnd2024:rules_gaining_spells_always_prepared_spells
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - prepared_spell_count
  events: []
  capabilities:
  - spell_preparation
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 104
  engine_gaps: &id001
  - dnd2024:rules_gaining_spells_always_prepared_spells
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 3ff443917839f9b060a9c77c75bd6c7a10fc88584284cc12d87385af74b8b2d1
  parent_name: Gaining Spells
  line_span:
    first: 9056
    count: 6
splat_clauses:
- key: source_rules
  text: '# Always-Prepared Spells Certain features might give you a spell that you always have prepared.
    If you also have a list of prepared spells that you can change, a spell that you always have prepared
    doesn’t count against the number of spells on that list.'
  sha256: c64bae4ff8ff5a69f4b23e8d88c8b89412eb9ebea60c90103f35adb057ed3412
  status: contract_specified
splat_requires_mechanics: *id001
---

# Always-Prepared Spells

<!-- SRD page 104 -->

Certain features might give you a spell that you\
always have prepared. If you also have a list of pre-\
pared spells that you can change, a spell that you\
always have prepared doesn’t count against the\
number of spells on that list.\
