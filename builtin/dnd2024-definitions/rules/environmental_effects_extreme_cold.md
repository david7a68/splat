---
splat_schema: 1
splat_id: 19858d37-f205-5771-83ad-26c554f3dd29
splat_kind: rule
splat_key: dnd2024:rules/environmental_effects_extreme_cold
splat_name: Extreme Cold
splat_origin:
  key: dnd2024:reference/pages/195
splat_sources:
- title: SRD 5.2.1
  locator: pp. 195
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/environmental_effects_extreme_cold
  parameters:
    temperature_fahrenheit_maximum: 0
    exposure_save:
      timing: end_of_each_hour
      ability: constitution
      dc: 10
      failure:
        exhaustion_levels_add: 1
      automatic_success_if_any:
      - cold_resistance
      - cold_immunity
  implementation: dnd2024:rule_environmental_effects_extreme_cold
splat_contracts:
- id: dnd2024:rule_environmental_effects_extreme_cold
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - effects
  events:
  - hour.ended
  capabilities:
  - environment
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 195
  engine_gaps: &id001
  - dnd2024:rule_environmental_effects_extreme_cold
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 38e5e8636509c0f68699033a85508e7c306dd89c5832dca4388933c206097b5e
  parent_name: Environmental Effects
  line_span:
    first: 9618
    count: 7
splat_clauses:
- key: source_rules
  text: '# Extreme Cold When the temperature is 0 degrees Fahrenheit or lower, a creature exposed to the
    extreme cold must succeed on a DC 10 Constitution saving throw at the end of each hour or gain 1 Exhaustion
    level. Creatures that have Resistance or Immunity to Cold damage automatically succeed on the save.'
  sha256: 63ca095fe819118d897288d56b0c4bc39ebcce6ec0ddb78e9c46b08e2c5bc35d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Extreme Cold

<!-- SRD page 195 -->

When the temperature is 0 degrees Fahrenheit or\
lower, a creature exposed to the extreme cold must\
succeed on a DC 10 Constitution saving throw at\
the end of each hour or gain 1 Exhaustion level.\
Creatures that have Resistance or Immunity to Cold\
damage automatically succeed on the save.\
