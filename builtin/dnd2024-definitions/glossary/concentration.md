---
splat_schema: 1
splat_id: 82c65dea-f963-5882-8a3b-20ec4be53d74
splat_kind: rule
splat_key: dnd2024:glossary/concentration
splat_name: Concentration
splat_origin:
  key: dnd2024:reference/glossary/concentration
splat_sources:
- title: SRD 5.2.1
  locator: pp. 179
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/concentration
  parameters:
    maximum_simultaneous_effects: 1
    effect_ends_on_concentration_loss: true
    maximum_duration: effect_specified
    voluntary_end:
      timing: any_time
      cost: none
    breaks_when:
    - event: spell.cast_started
      requires_concentration: true
    - event: effect.activated
      requires_concentration: true
    - event: condition.applied
      condition: incapacitated
    - event: creature.died
    damage_save:
      ability: constitution
      dc:
        op: min
        args:
        - 30
        - op: max
          args:
          - 10
          - op: floor
            args:
            - op: divide
              args:
              - var: damage_taken
              - 2
      failure: end_concentration
  implementation: dnd2024:glossary_concentration
splat_contracts:
- id: dnd2024:glossary_concentration
  inputs:
    subject:
      type: participant
      required: true
    effect:
      type: effect
      required: true
    damage_taken:
      required: false
      type: integer
  outputs:
  - concentration_state
  - ended_effects
  - save
  events:
  - spell.cast_started
  - effect.activated
  - damage.taken
  - condition.applied
  - creature.died
  - concentration.ended
  capabilities:
  - concentration
  - effects
  - checks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: a3bf2accc9b33ea146504397911fc9c957c2009450f0907548c4a8bc0eab5eac
  source_path: builtin/srd-5.2.1/glossary/concentration.md
  pages:
  - 179
  engine_gaps: &id001
  - dnd2024:glossary_concentration
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 6d0ba28b8458211b17af89271387a519f105e7b8af6254eb3d2b8ddd6dca2b2c
splat_clauses:
- key: source_rules
  text: '# Concentration Some spells and other effects require Concentration to remain active, as specified
    in their descriptions. If the effect’s creator loses Concentration, the effect ends. If the effect
    has a maximum duration, the effect’s description specifies how long the creator can concentrate on
    it: up to 1 minute, 1 hour, or some other duration. The creator can end Concentration at any time
    (no action required). The following factors break Concentration. Another Concentration Effect. You
    lose Concentration on an effect the moment you start casting a spell that requires Concentration or
    activate another effect that requires Concentration. Damage. If you take damage, you must succeed
    on a Constitution saving throw to maintain Concentration. The DC equals 10 or half the damage taken
    (round down), whichever number is higher, up to a maximum DC of 30. Incapacitated or Dead. Your Concentration
    ends if you have the Incapacitated condition or you die.'
  sha256: 3cce43ea73d216d3f34fbff58985304423b9856827cc4524b34b9a738f5c5d04
  status: contract_specified
splat_requires_mechanics: *id001
---

# Concentration

<!-- SRD page 179 -->

Some spells and other effects require Concentration\
to remain active, as specified in their descriptions.\
If the effect’s creator loses Concentration, the effect\
ends. If the effect has a maximum duration, the ef-\
fect’s description specifies how long the creator can\
concentrate on it: up to 1 minute, 1 hour, or some\
other duration. The creator can end Concentration\
at any time (no action required). The following fac-\
tors break Concentration.\
Another Concentration Effect. You lose Concen-\
tration on an effect the moment you start casting\
a spell that requires Concentration or activate an-\
other effect that requires Concentration.\
Damage. If you take damage, you must succeed\
on a Constitution saving throw to maintain Concen-\
tration. The DC equals 10 or half the damage taken\
(round down), whichever number is higher, up to a\
maximum DC of 30.\
Incapacitated or Dead. Your Concentration ends\
if you have the Incapacitated condition or you die.\

Original layout: [[../../srd-5.2.1/pages/179|page 179]].
