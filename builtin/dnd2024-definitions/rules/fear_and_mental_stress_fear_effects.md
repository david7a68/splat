---
splat_schema: 1
splat_id: 3f86bacd-c824-5fbd-8b2e-da184b352930
splat_kind: rule
splat_key: dnd2024:rules/fear_and_mental_stress_fear_effects
splat_name: Fear Effects
splat_origin:
  key: dnd2024:reference/pages/196
splat_sources:
- title: SRD 5.2.1
  locator: pp. 196
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/fear_and_mental_stress_fear_effects
  parameters:
    trigger: supernaturally_frightful_encounter
    default_save_ability: wisdom
    failed_save_condition: frightened
    dc_choice:
      actor: gm
      criterion: situation_terror
      examples:
      - situation: harmless_sarcophagus_apparition
        dc: 10
      - situation: magical_trap_private_worst_fear_illusion
        dc: 15
      - situation: abyss_portal_reveals_torment
        dc: 20
    default_repeat_save:
      timing: end_of_each_target_turn
      success: effect_ends
    optional_riders:
      actor: gm
      duration: while_frightened
      choices:
      - must_take_action: dash
        movement_direction: away_from_fear_source
      - attacks_against_target: advantage
      - per_turn_choose_exactly_one:
        - movement
        - action
        - bonus_action
  implementation: dnd2024:rule_fear_and_mental_stress_fear_effects
splat_contracts:
- id: dnd2024:rule_fear_and_mental_stress_fear_effects
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - turn.ended
  capabilities:
  - fear
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 196
  engine_gaps: &id001
  - dnd2024:rule_fear_and_mental_stress_fear_effects
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: afabb17b8f2fddbe780e96bb2ec46600f7baf42f7bbfabdc821cafaf694ed9f7
  parent_name: Fear and Mental Stress
  line_span:
    first: 9693
    count: 31
splat_clauses:
- key: source_rules
  text: '# Fear Effects Whenever the characters encounter something that is supernaturally frightful,
    use the Frightened condition as the baseline effect. Fear effects typically require a Wisdom saving
    throw, with a save DC based on how terrifying the situation is. The Sample Fear DCs table provides
    some examples. Sample Fear DCs Example Save DC When the characters open a sarcophagus, a 10 harmless
    yet terrifying apparition appears. A character triggers a magical trap that 15 creates an illusory
    manifestation of that character’s worst fears, visible only to that character. A portal to the Abyss
    opens, revealing a 20 nightmarish realm of torment and slaughter. Typically, a Frightened creature
    repeats the saving throw at the end of each of its turns, ending the effect on itself on a success.
    At your discretion, a Frightened creature might be subject to other effects as long as the Frightened
    condition lasts. Consider these examples: • The Frightened creature must take the Dash action on each
    of its turns and uses its movement to get farther away from the source of its fear. • Attack rolls
    against the Frightened creature have Advantage. • The Frightened creature can do only one of the following
    on each of its turns: move, take an action, or take a Bonus Action.'
  sha256: 30e05be5c8cb514ac31ba7d88b3aea1be37794202942df3417a2f0bfe939c929
  status: contract_specified
splat_requires_mechanics: *id001
---

# Fear Effects

<!-- SRD page 196 -->

Whenever the characters encounter something that\
is supernaturally frightful, use the Frightened con-\
dition as the baseline effect. Fear effects typically\
require a Wisdom saving throw, with a save DC\
based on how terrifying the situation is. The Sample\
Fear DCs table provides some examples.\
Sample Fear DCs\
Example Save DC\
When the characters open a sarcophagus, a 10\
harmless yet terrifying apparition appears.\
A character triggers a magical trap that 15\
creates an illusory manifestation of that\
character’s worst fears, visible only to that\
character.\
A portal to the Abyss opens, revealing a 20\
nightmarish realm of torment and slaughter.\
Typically, a Frightened creature repeats the saving\
throw at the end of each of its turns, ending the ef-\
fect on itself on a success.\
At your discretion, a Frightened creature might\
be subject to other effects as long as the Frightened\
condition lasts. Consider these examples:\
•  The Frightened creature must take the Dash ac-\
tion on each of its turns and uses its movement to\
get farther away from the source of its fear.\
•  Attack rolls against the Frightened creature have\
Advantage.\
•    The Frightened creature can do only one of the\
following on each of its turns: move, take an ac-\
tion, or take a Bonus Action.\
