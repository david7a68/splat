---
splat_schema: 1
splat_id: 7da65499-488c-5f4f-942a-103c2a1e1dcf
splat_kind: rule
splat_key: dnd2024:rules/environmental_effects_deep_water
splat_name: Deep Water
splat_origin:
  key: dnd2024:reference/pages/195
splat_sources:
- title: SRD 5.2.1
  locator: pp. 195
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/environmental_effects_deep_water
  parameters:
    water_depth_ft_greater_than: 100
    subject_lacks_swim_speed: true
    trigger: each_swimming_hour_end
    save:
      ability: constitution
      dc: 10
    failure:
      exhaustion_gain: 1
  implementation: dnd2024:rules_environmental_effects_deep_water
splat_contracts:
- id: dnd2024:rules_environmental_effects_deep_water
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - swimming.hour_finished
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
  - dnd2024:rules_environmental_effects_deep_water
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 31826f00ae8cbaab4e41128ad0c05b7564677a0d769b6983a665ef0faa8555c0
  parent_name: Environmental Effects
  line_span:
    first: 9611
    count: 7
splat_clauses:
- key: source_rules
  text: '# Deep Water Swimming through deep water (more than 100 feet deep) presents additional challenges
    because of the water’s pressure and cold temperature. After each hour of swimming in deep water, a
    creature that lacks a Swim Speed must succeed on a DC 10 Constitution saving throw or gain 1 Exhaustion
    level.'
  sha256: 183f121b5ffdcc6b7cf654cd5d4bb8076d03e418d6f8df0a216c38d3248241eb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Deep Water

<!-- SRD page 195 -->

Swimming through deep water (more than 100 feet\
deep) presents additional challenges because of the\
water’s pressure and cold temperature. After each\
hour of swimming in deep water, a creature that\
lacks a Swim Speed must succeed on a DC 10 Consti-\
tution saving throw or gain 1 Exhaustion level.\
