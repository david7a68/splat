---
splat_schema: 1
splat_id: bca7b937-6363-525d-bcf1-41168424e5e2
splat_kind: rule
splat_key: dnd2024:rules/warlock_lifedrinker
splat_name: Lifedrinker
splat_origin:
  key: dnd2024:reference/pages/073
splat_sources:
- title: SRD 5.2.1
  locator: pp. 73
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_lifedrinker
  parameters:
    prerequisite:
      warlock_level_minimum: 9
      invocation: pact_of_the_blade
    trigger: hit_creature_with_pact_weapon
    uses_per_turn: 1
    extra_damage:
      amount:
        dice:
          count: 1
          sides: 6
      type_choice:
      - necrotic
      - psychic
      - radiant
    optional_heal:
      spend_hit_dice: 1
      amount:
        op: max
        args:
        - 1
        - op: add
          args:
          - variable: spent_hit_die_roll
          - variable: constitution_modifier
  implementation: dnd2024:rule_warlock_lifedrinker
splat_contracts:
- id: dnd2024:rule_warlock_lifedrinker
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    constitution_modifier:
      type: number
      required: true
    spent_hit_die_roll:
      type: number
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - feature_effects
  events:
  - attack.hit
  capabilities:
  - invocations
  - damage
  - healing
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 73
  engine_gaps: &id001
  - dnd2024:rule_warlock_lifedrinker
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 407d0606ef63023195bf2ccc6f68bc1f6740bc5a9800cf5f0c7fb9002719bf58
  parent_name: Warlock
  line_span:
    first: 6263
    count: 10
splat_clauses:
- key: source_rules
  text: '# Lifedrinker Prerequisite: Level 9+ Warlock, Pact of the Blade Invocation Once per turn when
    you hit a creature with your pact weapon, you can deal an extra 1d6 Necrotic, Psychic, or Radiant
    damage (your choice) to the creature, and you can expend one of your Hit Point Dice to roll it and
    regain a number of Hit Points equal to the roll plus your Constitution modifier (minimum of 1 Hit
    Point).'
  sha256: 4d3288d82cbe72285ac9100e5f86f6a2e15487afd5a444a93a7e4b2de77a0a45
  status: contract_specified
splat_requires_mechanics: *id001
---

# Lifedrinker

<!-- SRD page 73 -->

Prerequisite: Level 9+ Warlock, Pact of the Blade\
Invocation\
Once per turn when you hit a creature with your\
pact weapon, you can deal an extra 1d6 Necrotic,\
Psychic, or Radiant damage (your choice) to the\
creature, and you can expend one of your Hit Point\
Dice to roll it and regain a number of Hit Points\
equal to the roll plus your Constitution modifier\
(minimum of 1 Hit Point).\
