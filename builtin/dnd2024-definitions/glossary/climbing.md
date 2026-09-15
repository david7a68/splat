---
splat_schema: 1
splat_id: 7556bf31-64d6-5732-89a5-292ea63b46a7
splat_kind: rule
splat_key: dnd2024:glossary/climbing
splat_name: Climbing
splat_origin:
  key: dnd2024:reference/glossary/climbing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 178, 179
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/climbing
  parameters:
    movement_mode: climb
    per_foot_cost:
      normal: 2
      difficult_terrain: 3
    mode_extra_cost_ignored_if:
      has_special_speed: climb
      uses_special_speed: true
    gm_optional_check:
      terrain_any:
      - slippery_surface
      - few_handholds
      required:
        chooser: gm
        options:
        - true
        - false
      ability: strength
      skill: athletics
      dc: 15
      success_required_to_move: true
  implementation: dnd2024:glossary_climbing
splat_contracts:
- id: dnd2024:glossary_climbing
  inputs:
    subject:
      type: participant
      required: true
    distance_feet:
      type: number
      required: true
    terrain:
      type: choice
      required: true
    use_special_speed:
      type: boolean
      required: true
    gm_requires_check:
      type: boolean
      required: true
  outputs:
  - movement_cost
  - check_requirement
  events:
  - movement.pending
  capabilities:
  - movement
  - checks
  - gm_choices
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 0f28c47f314127aaca84e07947d13158c4ddc45e1e29f9f747d6db4003f2229d
  source_path: builtin/srd-5.2.1/glossary/climbing.md
  pages:
  - 178
  - 179
  engine_gaps: &id001
  - dnd2024:glossary_climbing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 5b581f8c0b07e4e2018ff3f333c3fad7072a8b8311157850a4c41f118fdfa17a
splat_clauses:
- key: source_rules
  text: '# Climbing While you’re climbing, each foot of movement costs 1 extra foot (2 extra feet in Difficult
    Terrain). You ignore this extra cost if you have a Climb Speed and use it to climb. At the GM’s option,
    climbing a slippery surface or one with few handholds might require a successful DC 15 Strength (Athletics)
    check.'
  sha256: 4608b2af2e14fdbc57a382f6708a168d59c54f6a31021ee2d7971f962d059cff
  status: contract_specified
splat_requires_mechanics: *id001
---

# Climbing

<!-- SRD page 178 -->

While you’re climbing, each foot of movement costs\
1 extra foot (2 extra feet in Difficult Terrain). You\
ignore this extra cost if you have a Climb Speed and\
use it to climb.\

<!-- SRD page 179 -->

At the GM’s option, climbing a slippery surface or\
one with few handholds might require a successful\
DC 15 Strength (Athletics) check.\

Original layout: [[../../srd-5.2.1/pages/178|page 178]], [[../../srd-5.2.1/pages/179|page 179]].
