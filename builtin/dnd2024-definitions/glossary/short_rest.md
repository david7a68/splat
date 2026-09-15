---
splat_schema: 1
splat_id: 53fba91c-b91a-55db-a4d9-951e4bab37bd
splat_kind: rule
splat_key: dnd2024:glossary/short_rest
splat_name: Short Rest
splat_origin:
  key: dnd2024:reference/glossary/short_rest
splat_sources:
- title: SRD 5.2.1
  locator: pp. 187
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/short_rest
  parameters:
    duration_hours: 1
    maximum_exertion: light_activity
    allowed_examples:
    - read
    - talk
    - eat
    - stand_watch
    start_minimum_hit_points: 1
    completion:
      hit_dice:
        optional: true
        spend_available_only: true
        one_at_a_time: true
        healing_per_die:
          op: max
          args:
          - 1
          - op: add
            args:
            - var: hit_die_roll
            - var: constitution_modifier
        decide_next_after_each_roll: true
      feature_recharge: feature_description_defined
    interruptions:
    - event: initiative.rolled
    - event: spell.cast
      spell_level_min: 1
    - event: damage.taken
    interruption_result:
      rest_stops: true
      benefits: false
  implementation: dnd2024:glossary_short_rest
splat_contracts:
- id: dnd2024:glossary_short_rest
  inputs:
    subject:
      type: participant
      required: true
    rest_log:
      type: resolution
      required: true
    hit_dice_spending:
      type: choice
      required: true
  outputs:
  - rest_progress
  - healing
  - hit_dice_spent
  - feature_recharge
  events:
  - short_rest.started
  - short_rest.completed
  - initiative.rolled
  - spell.cast
  - damage.taken
  capabilities:
  - rests
  - hit_points
  - resources
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 3eaf71378392ea6b6d2684594060e350d89034b136227af8b5dfc39257332727
  source_path: builtin/srd-5.2.1/glossary/short_rest.md
  pages:
  - 187
  engine_gaps: &id001
  - dnd2024:glossary_short_rest
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 3a9373c185ff8a7105324844facdfd444daeb0fe6a5f58bad6ce1646a2226320
splat_clauses:
- key: source_rules
  text: '# Short Rest A Short Rest is a 1-hour period of downtime, during which a creature does nothing
    more strenuous than reading, talking, eating, or standing watch. To start a Short Rest, you must have
    at least 1 Hit Point. Benefits of the Rest. When you finish the rest, you gain the following benefits:
    Spend Hit Point Dice. You can spend one or more of your Hit Point Dice to regain Hit Points. For each
    Hit Point Die you spend in this way, roll the die and add your Constitution modifier to it. You regain
    Hit Points equal to the total (minimum of 1 Hit Point). You can decide to spend an additional Hit
    Point Die after each roll. Special Feature. Some features are recharged by a Short Rest. If you have
    such a feature, it recharges in the way specified in its description. Interrupting the Rest. A Short
    Rest is stopped by the following interruptions: • Rolling Initiative • Casting a spell other than
    a cantrip • Taking any damage An interrupted Short Rest confers no benefits.'
  sha256: 3b83fd1e326ec0b9382e81a531854799e008782b7824c88026aee20d2ed9b8a7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Short Rest

<!-- SRD page 187 -->

A Short Rest is a 1-hour period of downtime, during\
which a creature does nothing more strenuous than\
reading, talking, eating, or standing watch. To start\
a Short Rest, you must have at least 1 Hit Point.\
Benefits of the Rest. When you finish the rest, you\
gain the following benefits:\
Spend Hit Point Dice. You can spend one or more\
of your Hit Point Dice to regain Hit Points. For\
each Hit Point Die you spend in this way, roll the\
die and add your Constitution modifier to it. You\
regain Hit Points equal to the total (minimum of 1\
Hit Point). You can decide to spend an additional\
Hit Point Die after each roll.\
Special Feature. Some features are recharged by a\
Short Rest. If you have such a feature, it recharges\
in the way specified in its description.\
Interrupting the Rest. A Short Rest is stopped by\
the following interruptions:\
•  Rolling Initiative\
•  Casting a spell other than a cantrip\
•  Taking any damage\
An interrupted Short Rest confers no benefits.\

Original layout: [[../../srd-5.2.1/pages/187|page 187]].
