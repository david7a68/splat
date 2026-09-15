---
splat_schema: 1
splat_id: 4aa5c603-b801-5b3f-af76-41fed93d29d7
splat_kind: rule
splat_key: dnd2024:class_features/circle_of_the_land_level_14_nature_s_sanctuary
splat_name: 'Level 14: Nature’s Sanctuary'
splat_origin:
  key: dnd2024:reference/pages/046
splat_sources:
- title: SRD 5.2.1
  locator: pp. 46
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/circle_of_the_land_level_14_nature_s_sanctuary
  parameters:
    action: magic
    resource_cost:
      id: druid.wild_shape
      amount: 1
    area:
      shape: cube
      side_feet: 15
      placement: ground
      range_feet: 120
      geometry_validation: explicit_location_choice
    duration:
      minutes: 1
    ends_on:
    - actor.incapacitated
    - actor.died
    within_area:
      actor_and_allies:
        cover: half
      allies:
        resistance:
          var: actor.natures_ward.current_resistance
    move_area:
      action: bonus_action
      maximum_feet: 60
      destination:
        ground: true
        within_actor_feet: 120
    class_scope:
      class: druid
      minimum_level: 14
  implementation: dnd2024:class_features_circle_of_the_land_level_14_nature_s_sanctuary
splat_contracts:
- id: dnd2024:class_features_circle_of_the_land_level_14_nature_s_sanctuary
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - action.magic
  - action.bonus
  - participant.moved
  - condition.applied
  - participant.died
  capabilities:
  - character_features
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 46
  engine_gaps: &id001
  - dnd2024:class_features_circle_of_the_land_level_14_nature_s_sanctuary
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: d1712e35781a1da8be443ba18810769d67f0de5c1b58d8fffe720e68a05cf391
  parent_name: Circle of the Land
  line_span:
    first: 3863
    count: 11
splat_clauses:
- key: source_rules
  text: '# Level 14: Nature’s Sanctuary As a Magic action, you can expend a use of your Wild Shape and
    cause spectral trees and vines to appear in a 15-foot Cube on the ground within 120 feet of yourself.
    They last there for 1 minute or until you have the Incapacitated condition or die. You and your allies
    have Half Cover while in that area, and your allies gain the current Resistance of your Nature’s Ward
    while there. As a Bonus Action, you can move the Cube up to 60 feet to ground within 120 feet of yourself.'
  sha256: a6ddce96f2d9970c4a9bcbf605b8822e9060bb075603e071e52d8f837e82dee2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 14: Nature’s Sanctuary

<!-- SRD page 46 -->

As a Magic action, you can expend a use of your Wild\
Shape and cause spectral trees and vines to appear\
in a 15-foot Cube on the ground within 120 feet of\
yourself. They last there for 1 minute or until you\
have the Incapacitated condition or die. You and\
your allies have Half Cover while in that area, and\
your allies gain the current Resistance of your Na-\
ture’s Ward while there.\
As a Bonus Action, you can move the Cube up to\
60 feet to ground within 120 feet of yourself.\
