---
splat_schema: 1
splat_id: 9e1ed204-f430-5265-a98d-f8558da4656d
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_somatic_s
splat_name: Somatic (S)
splat_origin:
  key: dnd2024:reference/pages/105
splat_sources:
- title: SRD 5.2.1
  locator: pp. 105
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/casting_spells_somatic_s
  parameters:
    component: somatic
    hands_required_min: 1
    requires_perform_gestures: true
  implementation: dnd2024:rules_casting_spells_somatic_s
splat_contracts:
- id: dnd2024:rules_casting_spells_somatic_s
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - spell.cast_pending
  capabilities:
  - spell_components
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 105
  engine_gaps: &id001
  - dnd2024:rules_casting_spells_somatic_s
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 08141adde6e8629edc9d82359324cb8c5eda98820d218ad626364eb37d97b3eb
  parent_name: Casting Spells
  line_span:
    first: 9207
    count: 5
splat_clauses:
- key: source_rules
  text: '# Somatic (S) A Somatic component is a forceful gesticulation or an intricate set of gestures.
    A spellcaster must use at least one of their hands to perform these movements.'
  sha256: bd2caac6c03f676b8728bc748db3ad4c199c7ee9684977a3ce97315ee9eadfc3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Somatic (S)

<!-- SRD page 105 -->

A Somatic component is a forceful gesticulation\
or an intricate set of gestures. A spellcaster must\
use at least one of their hands to perform these\
movements.\
