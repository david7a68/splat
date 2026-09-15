---
splat_schema: 1
splat_id: a80aaf92-6a84-5d6f-9321-47b7aed615a0
splat_kind: rule
splat_key: dnd2024:glossary/exhaustion_condition
splat_name: Exhaustion [Condition]
splat_origin:
  key: dnd2024:reference/glossary/exhaustion_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 181
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/exhaustion_condition
  parameters:
    condition: exhaustion
    stacking: true
    levels_per_application: 1
    death_at_level: 6
    d20_modifier:
      op: multiply
      args:
      - -2
      - var: exhaustion_level
    speed_delta_feet:
      op: multiply
      args:
      - -5
      - var: exhaustion_level
    long_rest_levels_removed: 1
    ends_at_level: 0
  implementation: dnd2024:glossary_exhaustion_condition
splat_contracts:
- id: dnd2024:glossary_exhaustion_condition
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - exhaustion_level
  - modifiers
  - death
  events:
  - condition.applied
  - d20_test.pending
  - speed.calculated
  - long_rest.completed
  capabilities:
  - conditions
  - checks
  - movement
  - rests
  - death
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 9840387ee974bd036a215daca9b50aad644fcbe4460de5c213b2ff806385f4e1
  source_path: builtin/srd-5.2.1/glossary/exhaustion_condition.md
  pages:
  - 181
  engine_gaps: &id001
  - dnd2024:glossary_exhaustion_condition
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: d87780411dcfac950432fcdf4e7b3be60df369dc62f052f0997e68d7c677ab47
splat_clauses:
- key: source_rules
  text: '# Exhaustion [Condition] While you have the Exhaustion condition, you experience the following
    effects. Exhaustion Levels. This condition is cumulative. Each time you receive it, you gain 1 Exhaustion
    level. You die if your Exhaustion level is 6. D20 Tests Affected. When you make a D20 Test, the roll
    is reduced by 2 times your Exhaustion level. Speed Reduced. Your Speed is reduced by a number of feet
    equal to 5 times your Exhaustion level. Removing Exhaustion Levels. Finishing a Long Rest removes
    1 of your Exhaustion levels. When your Exhaustion level reaches 0, the condition ends.'
  sha256: 94c3409f9d3c620ea8a4f506aded8529af73751a94bcf7e7a4ba95bea5e7fd09
  status: contract_specified
splat_requires_mechanics: *id001
---

# Exhaustion [Condition]

<!-- SRD page 181 -->

While you have the Exhaustion condition, you\
experience the following effects.\
Exhaustion Levels. This condition is cumulative.\
Each time you receive it, you gain 1 Exhaustion\
level. You die if your Exhaustion level is 6.\
D20 Tests Affected. When you make a D20 Test,\
the roll is reduced by 2 times your Exhaustion level.\
Speed Reduced. Your Speed is reduced by a num-\
ber of feet equal to 5 times your Exhaustion level.\
Removing Exhaustion Levels. Finishing a Long\
Rest removes 1 of your Exhaustion levels. When\
your Exhaustion level reaches 0, the condition ends.\

Original layout: [[../../srd-5.2.1/pages/181|page 181]].
