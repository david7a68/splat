---
splat_schema: 1
splat_id: 47868ad9-a302-5fb2-aacc-8d29832f211d
splat_kind: rule
splat_key: dnd2024:rules/environmental_effects_thin_ice
splat_name: Thin Ice
splat_origin:
  key: dnd2024:reference/pages/195
splat_sources:
- title: SRD 5.2.1
  locator: pp. 195
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/environmental_effects_thin_ice
  parameters:
    area:
      shape: square
      side_feet: 10
    weight_tolerance_lb:
      roll:
        count: 3
        sides: 10
        add: 0
      multiplier: 10
    break_when_weight: greater_than_tolerance
    break_effect:
      creatures: all_on_broken_ice
      fall_into: frigid_water
      water_rules:
        key: dnd2024:rules/environmental_effects_frigid_water
  implementation: dnd2024:rules_environmental_effects_thin_ice
splat_contracts:
- id: dnd2024:rules_environmental_effects_thin_ice
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    hazard:
      type: object
      required: true
    event:
      type: event
      required: true
    resolution:
      type: resolution
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - effects
  - hazard_updates
  events:
  - object.load_changed
  capabilities:
  - hazards
  - effects
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 195
  engine_gaps: &id001
  - dnd2024:rules_environmental_effects_thin_ice
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 98426429bf25c22b5caebc766429607db176639b894f678110d0bd542277efe4
  parent_name: Environmental Effects
  line_span:
    first: 9676
    count: 7
splat_clauses:
- key: source_rules
  text: '# Thin Ice Thin ice has a weight tolerance of 3d10 × 10 pounds per 10-foot-square area. Whenever
    the total weight on an area of thin ice exceeds its tolerance, the ice in that area breaks. All creatures
    on broken ice fall through. Below the ice is frigid water (see “Frigid Water” above).'
  sha256: 3564920bf7331e66ef89534cf3a1589bb06a1c24f5cf671e6509fef28bbb909c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Thin Ice

<!-- SRD page 195 -->

Thin ice has a weight tolerance of 3d10 × 10 pounds\
per 10-foot-square area. Whenever the total weight\
on an area of thin ice exceeds its tolerance, the ice\
in that area breaks. All creatures on broken ice fall\
through. Below the ice is frigid water (see “Frigid\
Water” above).\
