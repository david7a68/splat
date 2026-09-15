---
splat_schema: 1
splat_id: 87ec9e0b-ffb6-579c-bf84-8139ff091e19
splat_kind: rule
splat_key: dnd2024:glossary/long_rest
splat_name: Long Rest
splat_origin:
  key: dnd2024:reference/glossary/long_rest
splat_sources:
- title: SRD 5.2.1
  locator: pp. 185
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/long_rest
  parameters:
    minimum_hours: 8
    minimum_sleep_hours: 6
    maximum_light_activity_hours: 2
    light_activities:
    - read
    - talk
    - eat
    - stand_watch
    sleep_condition: unconscious
    minimum_hours_after_previous_long_rest_finish_before_start: 16
    start_minimum_hit_points: 1
    completion:
      regain_lost_hit_points: all
      regain_spent_hit_dice: all
      restore_reduced_hit_point_maximum: true
      restore_reduced_ability_scores: true
      exhaustion_levels_removed: 1
      feature_recharge: each_feature_description
    interruptions:
    - event: initiative.rolled
    - event: spell.cast
      spell_level_min: 1
    - event: damage.taken
    - activity_any:
      - walking
      - physical_exertion
      hours: 1
    interruption_result:
      rest_stops: true
      short_rest_benefits_if_prior_rest_hours_at_least: 1
    resume:
      immediate_allowed: true
      additional_hours_per_interruption: 1
  implementation: dnd2024:glossary_long_rest
splat_contracts:
- id: dnd2024:glossary_long_rest
  inputs:
    subject:
      type: participant
      required: true
    rest_log:
      type: resolution
      required: true
    resume_after_interruption:
      required: false
      type: boolean
  outputs:
  - rest_progress
  - rest_benefits
  - interruption_result
  events:
  - long_rest.started
  - long_rest.completed
  - initiative.rolled
  - spell.cast
  - damage.taken
  - activity.completed
  capabilities:
  - rests
  - hit_points
  - statistics
  - conditions
  - resources
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 7942a8ded01f34b34e5d978fc4bcf58256918590b5f84f07251f79a35b67dd92
  source_path: builtin/srd-5.2.1/glossary/long_rest.md
  pages:
  - 185
  engine_gaps: &id001
  - dnd2024:glossary_long_rest
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 106a3ad9ef1287099a29dbbb4b78f3ed5956e9b5cd4633deb083bcee533b5d0a
splat_clauses:
- key: source_rules
  text: '# Long Rest A Long Rest is a period of extended downtime—at least 8 hours—available to any creature.
    During a Long Rest, you sleep for at least 6 hours and perform no more than 2 hours of light activity,
    such as reading, talking, eating, or standing watch. During sleep, you have the Unconscious condition.
    After you finish a Long Rest, you must wait at least 16 hours before starting another one. Benefits
    of the Rest. To start a Long Rest, you must have at least 1 Hit Point. When you finish the rest, you
    gain the following benefits: Regain All HP. You regain all lost Hit Points and all spent Hit Point
    Dice. If your Hit Point maximum was reduced, it returns to normal. Ability Scores Restored. If any
    of your ability scores were reduced, they return to normal. Exhaustion Reduced. If you have the Exhaustion
    condition, its level decreases by 1. Special Feature. Some features are recharged by a Long Rest.
    If you have such a feature, it recharges in the way specified in its description. Interrupting the
    Rest. A Long Rest is stopped by the following interruptions: • Rolling Initiative • Casting a spell
    other than a cantrip • Taking any damage • 1 hour of walking or other physical exertion If you rested
    at least 1 hour before the interruption, you gain the benefits of a Short Rest. See also “Short Rest.”
    You can resume a Long Rest immediately after an interruption. If you do so, the rest requires 1 additional
    hour per interruption to finish.'
  sha256: 4b65f9210f636dd926b8eff5094880a8974b3984b0ad86537ee99f4c5a9ce22e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Long Rest

<!-- SRD page 185 -->

A Long Rest is a period of extended downtime—at\
least 8 hours—available to any creature. During a\
Long Rest, you sleep for at least 6 hours and per-\
form no more than 2 hours of light activity, such as\
reading, talking, eating, or standing watch.\
During sleep, you have the Unconscious condition.\
After you finish a Long Rest, you must wait at least\
16 hours before starting another one.\
Benefits of the Rest. To start a Long Rest, you\
must have at least 1 Hit Point. When you finish the\
rest, you gain the following benefits:\
Regain All HP. You regain all lost Hit Points and all\
spent Hit Point Dice. If your Hit Point maximum\
was reduced, it returns to normal.\
Ability Scores Restored. If any of your ability\
scores were reduced, they return to normal.\
Exhaustion Reduced. If you have the Exhaustion\
condition, its level decreases by 1.\
Special Feature. Some features are recharged by a\
Long Rest. If you have such a feature, it recharges\
in the way specified in its description.\
Interrupting the Rest. A Long Rest is stopped by\
the following interruptions:\
•  Rolling Initiative\
•  Casting a spell other than a cantrip\
•  Taking any damage\
•  1 hour of walking or other physical exertion\
If you rested at least 1 hour before the interruption,\
you gain the benefits of a Short Rest. See also “Short\
Rest.”\
You can resume a Long Rest immediately after an\
interruption. If you do so, the rest requires 1 addi-\
tional hour per interruption to finish.\

Original layout: [[../../srd-5.2.1/pages/185|page 185]].
