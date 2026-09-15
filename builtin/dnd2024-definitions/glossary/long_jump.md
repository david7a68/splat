---
splat_schema: 1
splat_id: 7137802e-4cb8-559f-a7fa-3bf5429c7007
splat_kind: rule
splat_key: dnd2024:glossary/long_jump
splat_name: Long Jump
splat_origin:
  key: dnd2024:reference/glossary/long_jump
splat_sources:
- title: SRD 5.2.1
  locator: pp. 184, 185
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/long_jump
  parameters:
    direction: horizontal
    running_start:
      minimum_feet: 10
      immediately_before: true
    running_maximum_feet:
      var: strength_score
    standing_multiplier: 0.5
    movement_cost_per_foot: 1
    difficult_terrain_landing:
      ability: dexterity
      skill: acrobatics
      dc: 10
      failure_condition: prone
    low_obstacle:
      height_maximum_jump_distance_fraction: 0.25
      check_required:
        chooser: gm
        options:
        - true
        - false
      ability: strength
      skill: athletics
      dc: 10
      failure: hit_obstacle
    default_geometry_assumption: jump_height_irrelevant
  implementation: dnd2024:glossary_long_jump
splat_contracts:
- id: dnd2024:glossary_long_jump
  inputs:
    subject:
      type: participant
      required: true
    run_up_feet:
      type: number
      required: true
    jump_distance_feet:
      type: number
      required: true
    difficult_landing:
      type: boolean
      required: true
    obstacle:
      required: false
      type: object
    gm_requires_obstacle_check:
      required: false
      type: boolean
  outputs:
  - jump_limit
  - movement_cost
  - checks
  - condition_changes
  events:
  - movement.pending
  - jump.landed
  capabilities:
  - movement
  - checks
  - geometry
  - gm_choices
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 310b2651d30d3fad4939ff67747e7403400f759575d7bde9b97fb88b4c2e7ddc
  source_path: builtin/srd-5.2.1/glossary/long_jump.md
  pages:
  - 184
  - 185
  engine_gaps: &id001
  - dnd2024:glossary_long_jump
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 436bc6eaa878d6f797245cd588ccfb0bb48db4b9b5782bdd4f5ca774c60ca4a6
splat_clauses:
- key: source_rules
  text: '# Long Jump When you make a Long Jump, you leap horizontally a number of feet up to your Strength
    score if you move at least 10 feet immediately before the jump. When you make a standing Long Jump,
    you can leap only half that distance. Either way, each foot you jump costs a foot of movement. If
    you land in Difficult Terrain, you must succeed on a DC 10 Dexterity (Acrobatics) check or have the
    Prone condition. This Long Jump rule assumes that the height of the jump doesn’t matter, such as a
    jump across a stream or chasm. At your GM’s option, you must succeed on a DC 10 Strength (Athletics)
    check to clear a low obstacle (no taller than a quarter of the jump’s distance), such as a hedge or
    low wall. Otherwise, you hit the obstacle.'
  sha256: bb738fac6ba6299f9b1175ff2194120aea623a87ef29cfa148dff45b90238fd8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Long Jump

<!-- SRD page 184 -->

When you make a Long Jump, you leap horizontally\
a number of feet up to your Strength score if you\

<!-- SRD page 185 -->

move at least 10 feet immediately before the jump.\
When you make a standing Long Jump, you can leap\
only half that distance. Either way, each foot you\
jump costs a foot of movement.\
If you land in Difficult Terrain, you must succeed\
on a DC 10 Dexterity (Acrobatics) check or have the\
Prone condition.\
This Long Jump rule assumes that the height of\
the jump doesn’t matter, such as a jump across a\
stream or chasm. At your GM’s option, you must suc-\
ceed on a DC 10 Strength (Athletics) check to clear a\
low obstacle (no taller than a quarter of the jump’s\
distance), such as a hedge or low wall. Otherwise,\
you hit the obstacle.\

Original layout: [[../../srd-5.2.1/pages/184|page 184]], [[../../srd-5.2.1/pages/185|page 185]].
