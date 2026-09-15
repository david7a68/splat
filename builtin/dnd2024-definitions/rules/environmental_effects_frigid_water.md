---
splat_schema: 1
splat_id: 382c25fc-03ac-52bc-98dd-9248b89393ed
splat_kind: rule
splat_key: dnd2024:rules/environmental_effects_frigid_water
splat_name: Frigid Water
splat_origin:
  key: dnd2024:reference/pages/195
splat_sources:
- title: SRD 5.2.1
  locator: pp. 195
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/environmental_effects_frigid_water
  parameters:
    environment: frigid_water
    safe_immersion_minutes:
      var: subject.constitution_score
    trigger: each_additional_immersion_minute
    save:
      ability: constitution
      dc: 10
    automatic_success_any:
    - cold_resistance
    - cold_immunity
    - naturally_adapted_to_ice_cold_water
    failure:
      exhaustion_gain: 1
  implementation: dnd2024:rules_environmental_effects_frigid_water
splat_contracts:
- id: dnd2024:rules_environmental_effects_frigid_water
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - immersion.minute_finished
  capabilities:
  - environment
  - saving_throws
  - exhaustion
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 195
  engine_gaps: &id001
  - dnd2024:rules_environmental_effects_frigid_water
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 8eb78468b4198c06066567f9c0e7cc760f75b08144ddba620dd3d406f4a7de56
  parent_name: Environmental Effects
  line_span:
    first: 9636
    count: 10
splat_clauses:
- key: source_rules
  text: '# Frigid Water A creature can be immersed in frigid water for a number of minutes equal to its
    Constitution score before suffering any ill effects. Each additional minute spent in frigid water
    requires the creature to succeed on a DC 10 Constitution saving throw or gain 1 Exhaustion level.
    Creatures with Resistance or Immunity to Cold damage automatically succeed on the save, as do creatures
    that are naturally adapted to living in ice-cold water.'
  sha256: e819364120ca54d1c21f22da22bddc21cc518560641b54819f70f633919d8851
  status: contract_specified
splat_requires_mechanics: *id001
---

# Frigid Water

<!-- SRD page 195 -->

A creature can be immersed in frigid water for a\
number of minutes equal to its Constitution score\
before suffering any ill effects. Each additional\
minute spent in frigid water requires the creature\
to succeed on a DC 10 Constitution saving throw or\
gain 1 Exhaustion level. Creatures with Resistance\
or Immunity to Cold damage automatically suc-\
ceed on the save, as do creatures that are naturally\
adapted to living in ice-cold water.\
