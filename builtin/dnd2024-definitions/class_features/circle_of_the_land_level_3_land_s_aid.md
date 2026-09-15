---
splat_schema: 1
splat_id: 4ea24045-6ee6-51ce-9c40-3968055ba7fc
splat_kind: rule
splat_key: dnd2024:class_features/circle_of_the_land_level_3_land_s_aid
splat_name: 'Level 3: Land’s Aid'
splat_origin:
  key: dnd2024:reference/pages/046
splat_sources:
- title: SRD 5.2.1
  locator: pp. 46
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/circle_of_the_land_level_3_land_s_aid
  parameters:
    action: magic
    cost:
      resource: wild_shape
      amount: 1
    origin:
      range_feet: 60
      type: point
    area:
      shape: sphere
      radius_feet: 10
    damage_targets:
      creatures: chosen_in_area
    save:
      ability: constitution
      dc:
        stat: spell_save_dc
      success_damage_multiplier: 0.5
    damage:
      type: necrotic
      dice_by_druid_level:
        '3':
          dice:
            count: 2
            sides: 6
        '10':
          dice:
            count: 3
            sides: 6
        '14':
          dice:
            count: 4
            sides: 6
    healing:
      target_count: 1
      target_choice: creature_in_area
      dice_by_druid_level:
        '3':
          dice:
            count: 2
            sides: 6
        '10':
          dice:
            count: 3
            sides: 6
        '14':
          dice:
            count: 4
            sides: 6
  implementation: dnd2024:class_features_circle_of_the_land_level_3_land_s_aid
splat_contracts:
- id: dnd2024:class_features_circle_of_the_land_level_3_land_s_aid
  inputs:
    actor:
      type: participant
      required: true
    trigger:
      required: false
      type: event
    targets:
      required: false
      type: participant_list
  outputs:
  - feature_state
  - resource_changes
  - damage
  - healing
  events:
  - action.magic
  capabilities:
  - areas
  - saving_throws
  - damage
  - healing
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 46
  engine_gaps: &id001
  - dnd2024:class_features_circle_of_the_land_level_3_land_s_aid
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: f0634313544d5289289c92e06670f8f9a8279a440f7544221c3e7a4b72ccce2e
  parent_name: Circle of the Land
  line_span:
    first: 3826
    count: 13
splat_clauses:
- key: source_rules
  text: '# Level 3: Land’s Aid As a Magic action, you can expend a use of your Wild Shape and choose a
    point within 60 feet of yourself. Vitality-giving flowers and life-draining thorns appear for a moment
    in a 10-foot-radius Sphere centered on that point. Each creature of your choice in the Sphere must
    make a Constitution saving throw against your spell save DC, taking 2d6 Necrotic damage on a failed
    save or half as much damage on a successful one. One creature of your choice in that area regains
    2d6 Hit Points. The damage and healing increase by 1d6 when you reach Druid levels 10 (3d6) and 14
    (4d6).'
  sha256: edf63b3e3abaf2c69c386d85a39efa7a59657b8fb7e2429d150f4af63747a272
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Land’s Aid

<!-- SRD page 46 -->

As a Magic action, you can expend a use of your Wild\
Shape and choose a point within 60 feet of yourself.\
Vitality-giving flowers and life-draining thorns\
appear for a moment in a 10-foot-radius Sphere cen-\
tered on that point. Each creature of your choice in\
the Sphere must make a Constitution saving throw\
against your spell save DC, taking 2d6 Necrotic\
damage on a failed save or half as much damage on\
a successful one. One creature of your choice in that\
area regains 2d6 Hit Points.\
The damage and healing increase by 1d6 when\
you reach Druid levels 10 (3d6) and 14 (4d6).\
