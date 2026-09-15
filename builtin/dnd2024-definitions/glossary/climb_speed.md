---
splat_schema: 1
splat_id: b26bfd17-bd31-53ee-8535-096cf29fcca2
splat_kind: rule
splat_key: dnd2024:glossary/climb_speed
splat_name: Climb Speed
splat_origin:
  key: dnd2024:reference/glossary/climb_speed
splat_sources:
- title: SRD 5.2.1
  locator: pp. 179
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/climb_speed
  parameters:
    movement_mode: climb
    replaces_speed: true
    ignores_mode_extra_movement_cost: true
  implementation: dnd2024:glossary_climb_speed
splat_contracts:
- id: dnd2024:glossary_climb_speed
  inputs:
    subject:
      type: participant
      required: true
    distance_feet:
      type: number
      required: true
  outputs:
  - movement_cost
  events: []
  capabilities:
  - movement
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 7b33dae5c697faa9472e04f6baa1ad9b68641ccd3a6a4bd5c36c35cb977f6ded
  source_path: builtin/srd-5.2.1/glossary/climb_speed.md
  pages:
  - 179
  engine_gaps: &id001
  - dnd2024:glossary_climb_speed
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 90441b99c423400aa0416fd95bc2034886e174171ddc2285e9847f1889ca0c9e
splat_clauses:
- key: source_rules
  text: '# Climb Speed A Climb Speed can be used in place of Speed to traverse a vertical surface without
    expending the extra movement normally associated with climbing. See also “Climbing” and “Speed.”'
  sha256: 03bba0c0a5a4bff776d200fc681e95a46ac6f70ca8f1a46c5813f6ba9a451a99
  status: contract_specified
splat_requires_mechanics: *id001
---

# Climb Speed

<!-- SRD page 179 -->

A Climb Speed can be used in place of Speed to tra-\
verse a vertical surface without expending the extra\
movement normally associated with climbing. See\
also “Climbing” and “Speed.”\

Original layout: [[../../srd-5.2.1/pages/179|page 179]].
