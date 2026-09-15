---
splat_schema: 1
splat_id: b6abf51d-2706-5b2d-a211-7ce1d3b86d16
splat_kind: rule
splat_key: dnd2024:rules/environmental_effects_extreme_heat
splat_name: Extreme Heat
splat_origin:
  key: dnd2024:reference/pages/195
splat_sources:
- title: SRD 5.2.1
  locator: pp. 195
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/environmental_effects_extreme_heat
  parameters:
    temperature_f_minimum: 100
    requires_no_drinkable_water: true
    save:
      ability: constitution
      event: exposure_hour.ended
      dc:
        initial: 5
        increment_per_additional_hour: 1
      disadvantage_if_armor:
      - medium
      - heavy
      automatic_success_if_any:
      - fire_resistance
      - fire_immunity
    failure:
      exhaustion_levels: 1
  implementation: dnd2024:rules_environmental_effects_extreme_heat
splat_contracts:
- id: dnd2024:rules_environmental_effects_extreme_heat
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
  - time.hour_elapsed
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
  - dnd2024:rules_environmental_effects_extreme_heat
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: bcbf6cf2dc37a9af1987469524a9bc535770b01a6ceff82dd66c204154e97be6
  parent_name: Environmental Effects
  line_span:
    first: 9625
    count: 11
splat_clauses:
- key: source_rules
  text: '# Extreme Heat When the temperature is 100 degrees Fahrenheit or higher, a creature exposed to
    the extreme heat and without access to drinkable water must succeed on a Constitution saving throw
    at the end of each hour or gain 1 Exhaustion level. The DC is 5 for the first hour and increases by
    1 for each additional hour. Creatures wearing Medium or Heavy armor have Disadvantage on the save.
    Creatures that have Resistance or Immunity to Fire damage automatically succeed on the save.'
  sha256: 21e4adeead0aa2656dc04a31829498a41eadb36cd8d543edc21e3022b502f496
  status: contract_specified
splat_requires_mechanics: *id001
---

# Extreme Heat

<!-- SRD page 195 -->

When the temperature is 100 degrees Fahrenheit or\
higher, a creature exposed to the extreme heat and\
without access to drinkable water must succeed on\
a Constitution saving throw at the end of each hour\
or gain 1 Exhaustion level. The DC is 5 for the first\
hour and increases by 1 for each additional hour.\
Creatures wearing Medium or Heavy armor have\
Disadvantage on the save. Creatures that have Re-\
sistance or Immunity to Fire damage automatically\
succeed on the save.\
