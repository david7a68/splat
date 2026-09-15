---
splat_schema: 1
splat_id: 36f8521e-68ca-57d2-900a-89da5c6a91b4
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_2_monk_s_focus
splat_name: 'Level 2: Monk’s Focus'
splat_origin:
  key: dnd2024:reference/pages/050
splat_sources:
- title: SRD 5.2.1
  locator: pp. 50, 51
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/monk_level_2_monk_s_focus
  parameters:
    resource:
      id: focus_points
      maximum:
        stat: classes.monk.level
      recover:
        events:
        - rest.short_rest_completed
        - rest.long_rest_completed
        amount: all_expended
    feature_save_dc:
      op: add
      args:
      - 8
      - stat: abilities.wisdom.modifier
      - stat: proficiency_bonus
    options:
    - id: flurry_of_blows
      action: bonus_action
      focus_cost: 1
      unarmed_strikes: 2
    - id: patient_defense
      action: bonus_action
      focus_cost: 0
      actions:
      - disengage
      paid_alternative:
        focus_cost: 1
        actions:
        - disengage
        - dodge
    - id: step_of_the_wind
      action: bonus_action
      focus_cost: 0
      actions:
      - dash
      paid_alternative:
        focus_cost: 1
        actions:
        - disengage
        - dash
        jump_distance_multiplier: 2
        duration: current_turn
  implementation: dnd2024:class_features_monk_level_2_monk_s_focus
splat_contracts:
- id: dnd2024:class_features_monk_level_2_monk_s_focus
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
  - resource_changes
  - movement
  events:
  - action.bonus
  - rest.short_rest_completed
  - rest.long_rest_completed
  capabilities:
  - resources
  - attacks
  - action_economy
  - movement
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 50
  - 51
  engine_gaps: &id001
  - dnd2024:class_features_monk_level_2_monk_s_focus
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 08a920e2a9cad519e1f227527982cfb050ec60208d5764b961f1df2b12d0046e
  parent_name: Monk
  line_span:
    first: 4198
    count: 28
splat_clauses:
- key: source_rules
  text: '# Level 2: Monk’s Focus Your focus and martial training allow you to harness a well of extraordinary
    energy within yourself. This energy is represented by Focus Points. Your Monk level determines the
    number of points you have, as shown in the Focus Points column of the Monk Features table. You can
    expend these points to enhance or fuel certain Monk features. You start knowing three such features:
    Flurry of Blows, Patient Defense, and Step of the Wind, each of which is detailed below. When you
    expend a Focus Point, it is unavailable until you finish a Short or Long Rest, at the end of which
    you regain all your expended points. Some features that use Focus Points require your target to make
    a saving throw. The save DC equals 8 plus your Wisdom modifier and Proficiency Bonus. Flurry of Blows.
    You can expend 1 Focus Point to make two Unarmed Strikes as a Bonus Action. Patient Defense. You can
    take the Disengage action as a Bonus Action. Alternatively, you can expend 1 Focus Point to take both
    the Disengage and the Dodge actions as a Bonus Action. Step of the Wind. You can take the Dash action
    as a Bonus Action. Alternatively, you can expend 1 Focus Point to take both the Disengage and Dash
    actions as a Bonus Action, and your jump distance is doubled for the turn.'
  sha256: 8e800d56eb26509014a0d36195d2da2a6396938b210336b387abb814654e5661
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Monk’s Focus

<!-- SRD page 50 -->

Your focus and martial training allow you to har-\
ness a well of extraordinary energy within yourself.\
This energy is represented by Focus Points. Your\
Monk level determines the number of points you\
have, as shown in the Focus Points column of the\
Monk Features table.\
You can expend these points to enhance or fuel\
certain Monk features. You start knowing three\
such features: Flurry of Blows, Patient Defense, and\
Step of the Wind, each of which is detailed below.\
When you expend a Focus Point, it is unavailable\
until you finish a Short or Long Rest, at the end of\
which you regain all your expended points.\
Some features that use Focus Points require your\
target to make a saving throw. The save DC equals 8\
plus your Wisdom modifier and Proficiency Bonus.\
Flurry of Blows. You can expend 1 Focus Point to\
make two Unarmed Strikes as a Bonus Action.\
Patient Defense. You can take the Disengage\
action as a Bonus Action. Alternatively, you can\

<!-- SRD page 51 -->

expend 1 Focus Point to take both the Disengage\
and the Dodge actions as a Bonus Action.\
Step of the Wind. You can take the Dash action\
as a Bonus Action. Alternatively, you can expend 1\
Focus Point to take both the Disengage and Dash\
actions as a Bonus Action, and your jump distance is\
doubled for the turn.\
