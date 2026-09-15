---
splat_schema: 1
splat_id: d7e7b44b-a196-53c8-97e2-1a0c79c26c4e
splat_kind: rule
splat_key: dnd2024:rules/casting_spells
splat_name: Casting Spells
splat_origin:
  key: dnd2024:reference/pages/104
splat_sources:
- title: SRD 5.2.1
  locator: pp. 104
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/casting_spells
  parameters:
    armor_worn_requires_training_to_cast: true
  implementation: dnd2024:rule_casting_spells
splat_contracts:
- id: dnd2024:rule_casting_spells
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - spell_resolution
  events: []
  capabilities:
  - spellcasting
  - proficiency
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 104
  engine_gaps: &id001
  - dnd2024:rule_casting_spells
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 962753713de9af37bb5614944e914c56dc9b188a748398621f782848168302b7
  parent_name: Casting Spells
  line_span:
    first: 9062
    count: 9
splat_clauses:
- key: source_rules
  text: '# Casting Spells Each spell description has a series of entries that provide the details needed
    to cast the spell. The following sections explain each of those entries, which follow a spell’s name.
    castinG in arMor You must have training with any armor you are wearing to cast spells while wearing
    it. You are otherwise too hampered by the armor for spellcasting.'
  sha256: af98a38a2fbb7feb39108017fa806edb0a3054d248b5222614bc8b848570d1de
  status: contract_specified
splat_requires_mechanics: *id001
---

# Casting Spells

<!-- SRD page 104 -->

Each spell description has a series of entries that\
provide the details needed to cast the spell. The fol-\
lowing sections explain each of those entries, which\
follow a spell’s name.\
castinG in arMor\
You must have training with any armor you are wear-\
ing to cast spells while wearing it. You are otherwise\
too hampered by the armor for spellcasting.\
