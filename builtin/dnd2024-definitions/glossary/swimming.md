---
splat_schema: 1
splat_id: 8910f23e-77e5-525c-a122-2ab5a264c305
splat_kind: rule
splat_key: dnd2024:glossary/swimming
splat_name: Swimming
splat_origin:
  key: dnd2024:reference/glossary/swimming
splat_sources:
- title: SRD 5.2.1
  locator: pp. 189
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/swimming
  parameters:
    movement_mode: swim
    per_foot_cost:
      normal: 2
      difficult_terrain: 3
    mode_extra_cost_ignored_if:
      has_special_speed: swim
      uses_special_speed: true
    gm_optional_check:
      terrain_any:
      - rough_water
      required:
        chooser: gm
        options:
        - true
        - false
      ability: strength
      skill: athletics
      dc: 15
      success_required_to_move: true
  implementation: dnd2024:glossary_swimming
splat_contracts:
- id: dnd2024:glossary_swimming
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
  source_sha256: 54a74dc746d258b6ce231deb935c5150f342c5cd8c0963018b9ada7d1d633ec5
  source_path: builtin/srd-5.2.1/glossary/swimming.md
  pages:
  - 189
  engine_gaps: &id001
  - dnd2024:glossary_swimming
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 3c37218b2d9ea701766c512a97e976ab48ddfa3a2e5f7e036698e91e6b1a8557
splat_clauses:
- key: source_rules
  text: '# Swimming While you’re swimming, each foot of movement costs 1 extra foot (2 extra feet in Difficult
    Terrain). You ignore this extra cost if you have a Swim Speed and use it to swim. At the GM’s option,
    moving any distance in rough water might require a successful DC 15 Strength (Athletics) check.'
  sha256: b832b0923f8248f0598431b6014053e0c5f8c48162a0fd13497ed3313e874dc3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Swimming

<!-- SRD page 189 -->

While you’re swimming, each foot of movement\
costs 1 extra foot (2 extra feet in Difficult Terrain).\
You ignore this extra cost if you have a Swim Speed\
and use it to swim. At the GM’s option, moving any\
distance in rough water might require a successful\
DC 15 Strength (Athletics) check.\

Original layout: [[../../srd-5.2.1/pages/189|page 189]].
