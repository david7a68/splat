---
splat_schema: 1
splat_id: 2f42c7a2-36ab-5b1d-9a9d-968c579a6c4c
splat_kind: spell
splat_key: dnd2024:spells/shocking_grasp
splat_name: Shocking Grasp
splat_origin:
  key: dnd2024:reference/spell/shocking_grasp
splat_sources:
- title: SRD 5.2.1
  locator: pp. 162
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: evocation
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: touch
splat_rule:
  mechanic: dnd2024:spells/shocking_grasp
  implementation: dnd2024:spell_shocking_grasp
  parameters:
    targets:
      kind: creature
      maximum: 1
      within_spell_range: true
    attack:
      kind: melee_spell
      hit:
        damage:
          dice:
            count:
              by_caster_level:
              - minimum: 1
                count: 1
              - minimum: 5
                count: 2
              - minimum: 11
                count: 3
              - minimum: 17
                count: 4
            sides: 8
            add: 0
          type: lightning
        prohibit: opportunity_attacks
        until: target_next_turn_start
splat_contracts:
- id: dnd2024:spell_shocking_grasp
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      type: participant_list
      required: true
    caster_level:
      type: integer
      required: true
    choices:
      required: false
      type: choice
    trigger:
      required: false
      type: event
  outputs:
  - effects
  - damage
  - attacks
  events:
  - spell.cast_completed
  - attack.roll
  - attack.hit
  - turn.start
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
  - attacks
splat_conversion:
  status: specified
  category: spell
  source_sha256: afb1d4ed51d5e64f4cce4f9ee74c7269f656c51e935c12e28d7f6f877a34fc6e
  source_path: builtin/srd-5.2.1/spell/shocking_grasp.md
  pages:
  - 162
  engine_gaps: &id001
  - dnd2024:spell_shocking_grasp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: f11ab66b28db7a6ed5d64ea1d5f7427e081f00629be50ce99056567d2cc6e468
splat_clauses:
- key: source_rules
  text: '# Shocking Grasp Evocation Cantrip (Sorcerer, Wizard) Casting Time: Action Range: Touch Components:
    V, S Duration: Instantaneous Lightning springs from you to a creature that you try to touch. Make
    a melee spell attack against the target. On a hit, the target takes 1d8 Lightning damage, and it can’t
    make Opportunity Attacks until the start of its next turn. Cantrip Upgrade. The damage increases by
    1d8 when you reach levels 5 (2d8), 11 (3d8), and 17 (4d8).'
  sha256: b857d243a0d1f905cc349017e01e7ca77a940512bd3b7cf05dce937e9ade0962
  status: contract_specified
splat_requires_mechanics: *id001
---

# Shocking Grasp

<!-- SRD page 162 -->

Evocation Cantrip (Sorcerer, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Instantaneous\
Lightning springs from you to a creature that you\
try to touch. Make a melee spell attack against the\
target. On a hit, the target takes 1d8 Lightning dam-\
age, and it can’t make Opportunity Attacks until the\
start of its next turn.\
Cantrip Upgrade. The damage increases by 1d8\
when you reach levels 5 (2d8), 11 (3d8), and 17\
(4d8).\

Original layout: [[../../srd-5.2.1/pages/162|page 162]].
