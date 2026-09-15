---
splat_schema: 1
splat_id: 06a93d1d-494d-59a7-9d12-c3156f10b120
splat_kind: rule
splat_key: dnd2024:glossary/fly_speed
splat_name: Fly Speed
splat_origin:
  key: dnd2024:reference/glossary/fly_speed
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/fly_speed
  parameters:
    movement_mode: fly
    medium: air
    stay_aloft_until_any:
    - land
    - fall
    - die
  implementation: dnd2024:glossary_fly_speed
splat_contracts:
- id: dnd2024:glossary_fly_speed
  inputs:
    subject:
      type: participant
      required: true
    distance_feet:
      type: number
      required: true
  outputs:
  - flight_budget
  - aloft_state
  events: []
  capabilities:
  - movement
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 83481e78ed41e60319b2471582cab0ae85e77775acc8e20abe5c8754ae8fcc9b
  source_path: builtin/srd-5.2.1/glossary/fly_speed.md
  pages:
  - 182
  engine_gaps: &id001
  - dnd2024:glossary_fly_speed
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: dfa4b6d3b46a69f9d9a65b91d9b39dd6167370cf38724d662b323d88031b069e
splat_clauses:
- key: source_rules
  text: '# Fly Speed A Fly Speed can be used to travel through the air. While you have a Fly Speed, you
    can stay aloft until you land, fall, or die. See also “Flying” and “Speed.”'
  sha256: 99bc876821ad757ca5e36ceda6bf43b1c2d615eb116c375323678097388035dd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Fly Speed

<!-- SRD page 182 -->

A Fly Speed can be used to travel through the air.\
While you have a Fly Speed, you can stay aloft until\
you land, fall, or die. See also “Flying” and “Speed.”\

Original layout: [[../../srd-5.2.1/pages/182|page 182]].
