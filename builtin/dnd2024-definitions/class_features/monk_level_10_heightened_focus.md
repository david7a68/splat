---
splat_schema: 1
splat_id: 05de10ba-5e62-5333-b1d2-a121799a62b2
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_10_heightened_focus
splat_name: 'Level 10: Heightened Focus'
splat_origin:
  key: dnd2024:reference/pages/051
splat_sources:
- title: SRD 5.2.1
  locator: pp. 51, 52
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/monk_level_10_heightened_focus
  parameters:
    flurry_of_blows:
      focus_cost: 1
      replace_unarmed_strike_count: 3
    patient_defense:
      when_focus_spent: true
      temporary_hit_points:
        dice:
          count: 2
          sides:
            stat: martial_arts_die.sides
    step_of_the_wind:
      when_focus_spent: true
      companion_choice:
        count: 1
        willing: true
        maximum_distance_feet: 5
        maximum_size: large
      moves_with_source_until: current_turn_end
      companion_movement_provokes_opportunity_attacks: false
  implementation: dnd2024:class_features_monk_level_10_heightened_focus
splat_contracts:
- id: dnd2024:class_features_monk_level_10_heightened_focus
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
    destination:
      required: false
      type: location
  outputs:
  - feature_state
  - movement
  events:
  - focus_feature.used
  capabilities:
  - attacks
  - temporary_hit_points
  - movement
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 51
  - 52
  engine_gaps: &id001
  - dnd2024:class_features_monk_level_10_heightened_focus
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 7ca9ce207b09f5332294138dbb0a01e90223eb542da51ec626bedfd599ba53c6
  parent_name: Monk
  line_span:
    first: 4299
    count: 16
splat_clauses:
- key: source_rules
  text: '# Level 10: Heightened Focus Your Flurry of Blows, Patient Defense, and Step of the Wind gain
    the following benefits. Flurry of Blows. You can expend 1 Focus Point to use Flurry of Blows and make
    three Unarmed Strikes with it instead of two. Patient Defense. When you expend a Focus Point to use
    Patient Defense, you gain a number of Temporary Hit Points equal to two rolls of your Martial Arts
    die. Step of the Wind. When you expend a Focus Point to use Step of the Wind, you can choose a willing
    creature within 5 feet of yourself that is Large or smaller. You move the creature with you until
    the end of your turn. The creature’s movement doesn’t provoke Opportunity Attacks.'
  sha256: 3dac125af6b8d80623e9ac777aaff893ffd7caae5971eb6cf6fc9e304d0227f3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 10: Heightened Focus

<!-- SRD page 51 -->

Your Flurry of Blows, Patient Defense, and Step of\
the Wind gain the following benefits.\
Flurry of Blows. You can expend 1 Focus Point\
to use Flurry of Blows and make three Unarmed\
Strikes with it instead of two.\

<!-- SRD page 52 -->

Patient Defense. When you expend a Focus Point\
to use Patient Defense, you gain a number of Tem-\
porary Hit Points equal to two rolls of your Martial\
Arts die.\
Step of the Wind. When you expend a Focus Point\
to use Step of the Wind, you can choose a willing\
creature within 5 feet of yourself that is Large or\
smaller. You move the creature with you until the\
end of your turn. The creature’s movement doesn’t\
provoke Opportunity Attacks.\
