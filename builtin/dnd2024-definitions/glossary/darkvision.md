---
splat_schema: 1
splat_id: f6a01864-3c13-5932-9b36-8e7eb12a3afd
splat_kind: rule
splat_key: dnd2024:glossary/darkvision
splat_name: Darkvision
splat_origin:
  key: dnd2024:reference/glossary/darkvision
splat_sources:
- title: SRD 5.2.1
  locator: pp. 180
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/darkvision
  parameters:
    range: feature_specified
    within_range_light_mapping:
      dim: bright
      darkness: dim
    darkness_colors: grayscale
  implementation: dnd2024:glossary_darkvision
splat_contracts:
- id: dnd2024:glossary_darkvision
  inputs:
    subject:
      type: participant
      required: true
    range_feet:
      type: number
      required: true
    target_location:
      type: location
      required: true
  outputs:
  - perceived_illumination
  - color_vision
  events: []
  capabilities:
  - senses
  - visibility
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 517d6dcf03dc4e9e6ece1374dfe2ef962ae8ac4c0f29fd2967e312a6e5f2cb33
  source_path: builtin/srd-5.2.1/glossary/darkvision.md
  pages:
  - 180
  engine_gaps: &id001
  - dnd2024:glossary_darkvision
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: d301cde5a479c54a78ab67462841991d39868522e38fa40421ee0de5415babc1
splat_clauses:
- key: source_rules
  text: '# Darkvision If you have Darkvision, you can see in Dim Light within a specified range as if
    it were Bright Light and in Darkness within that range as if it were Dim Light. You discern colors
    in that Darkness only as shades of gray. See also “Playing the Game” (“Exploration”).'
  sha256: 4dacbf697fff030b2e401c4a2316bde68947b195d48272bc4939833a1b758b14
  status: contract_specified
splat_requires_mechanics: *id001
---

# Darkvision

<!-- SRD page 180 -->

If you have Darkvision, you can see in Dim Light\
within a specified range as if it were Bright Light\
and in Darkness within that range as if it were\
Dim Light. You discern colors in that Darkness\
only as shades of gray. See also “Playing the Game”\
(“Exploration”).\

Original layout: [[../../srd-5.2.1/pages/180|page 180]].
