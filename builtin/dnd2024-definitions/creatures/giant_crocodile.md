---
splat_schema: 1
splat_id: 2ffcad9b-b09c-5269-97ed-5a5993a212fe
splat_kind: creature
splat_key: dnd2024:creatures/giant_crocodile
splat_name: Giant Crocodile
splat_origin:
  key: dnd2024:reference/creature/giant_crocodile
splat_sources:
- title: SRD 5.2.1
  locator: pp. 350
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Swim 50 ft.
  skills_text: Stealth +5
  senses_text: Passive Perception 10
  languages_text: None
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 85 (9d12 + 27)
splat_stats:
  strength:
    value: 21
  save_strength:
    value: 5
  dexterity:
    value: 9
  save_dexterity:
    value: -1
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 14
  max_hp:
    value: 85
  initiative:
    value: -1
  speed:
    value: 30
    unit: ft
  swim_speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_crocodile
  implementation: dnd2024:creature_giant_crocodile
  parameters:
    skill_totals:
      stealth: 5
    senses: []
    passive_perception: 10
    languages:
      speak: true
      known: []
    features:
      hold_breath:
        economy: passive
        op: breathing.hold
        duration:
          amount: 1
          unit: hour
      bite:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 8
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 3
                sides: 10
              add: 5
            average: 21
        - op: grapple
          escape_dc: 15
          when:
            target_kind: creature
            target_maximum_size: large
          while_grappled:
          - op: condition.apply
            condition: restrained
          - op: targeting.prohibit
            feature: tail
            source: actor
      tail:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 8
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: bludgeoning
            amount:
              dice:
                count: 3
                sides: 8
              add: 5
            average: 18
        - op: condition.apply
          condition: prone
          when:
            target_kind: creature
            target_maximum_size: large
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 1
          feature: bite
        - count: 1
          feature: tail
    damage_resolution:
      choice:
      - printed_average
      - roll_expression
      chooser: controller
      exclusive: true
    spatial_resolution:
      provider: caller
      required_facts:
      - distance
      - area_membership
      - visibility
      - occupancy
      - movement_path
      validate_against_typed_constraints: true
splat_contracts:
- id: dnd2024:creature_giant_crocodile
  inputs:
    actor:
      type: participant
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    choices:
      required: false
      type: choice
    scene:
      required: false
      type: resolution
    event:
      required: false
      type: event
  outputs:
  - effects
  - roll_requests
  - state_changes
  events:
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 978fbd6426c52a931e411bea8ac2f3f28624dd4fdd7181beb88ca052567b71f0
  source_path: builtin/srd-5.2.1/creature/giant_crocodile.md
  pages:
  - 350
  engine_gaps: &id001
  - dnd2024:creature_giant_crocodile
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: a392879e39a47b20752ef47abcce23c00c7d76d7869c280bfa58b9181349f343
splat_clauses:
- key: source_rules
  text: '# Giant Crocodile Huge Beast, Unaligned AC 14 Initiative -1 (9) HP 85 (9d12 + 27) Speed 30 ft.,
    Swim 50 ft. MOD SAVE MOD SAVE MOD SAVE Str 21 +5 +5 Dex 9 -1 -1 Con 17 +3 +3 Int 2 -4 -4 Wis 10 +0
    +0 Cha 7 -2 -2 Skills Stealth +5 Senses Passive Perception 10 Languages None CR 5 (XP 1,800; PB +3)
    ### Traits Hold Breath. The crocodile can hold its breath for 1 hour. ### Actions Multiattack. The
    crocodile makes one Bite attack and one Tail attack. Bite. Melee Attack Roll: +8, reach 5 ft. Hit:
    21 (3d10 + 5) Piercing damage. If the target is a Large or smaller creature, it has the Grappled condition
    (escape DC 15). While Grappled, the target has the Restrained condition and can’t be targeted by the
    crocodile’s Tail. Tail. Melee Attack Roll: +8, reach 10 ft. Hit: 18 (3d8 + 5) Bludgeoning damage.
    If the target is a Large or smaller creature, it has the Prone condition.'
  sha256: 0bc167e980c8ad0b7b6e51727e69afd5f22096b13c7e62f342c4f28af6571af1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Crocodile

<!-- SRD page 350 -->

Huge Beast, Unaligned\
AC 14  Initiative −1 (9)\
HP 85 (9d12 + 27)\
Speed 30 ft., Swim 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 21 +5 +5 Dex 9 −1 −1 Con 17 +3 +3\
Int 2 −4 −4 Wis 10 +0 +0 Cha 7 −2 −2\
Skills Stealth +5\
Senses Passive Perception 10\
Languages None\
CR 5 (XP 1,800; PB +3)\

### Traits

Hold Breath. The crocodile can hold its breath\
for 1 hour.\

### Actions

Multiattack. The crocodile makes one Bite attack and\
one Tail attack.\
Bite. Melee Attack Roll: +8, reach 5 ft. Hit: 21 (3d10\
+ 5) Piercing damage. If the target is a Large or smaller\
creature, it has the Grappled condition (escape DC 15).\
While Grappled, the target has the Restrained condition\
and can’t be targeted by the crocodile’s Tail.\
Tail. Melee Attack Roll: +8, reach 10 ft. Hit: 18 (3d8\
+ 5) Bludgeoning damage. If the target is a Large or\
smaller creature, it has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/350|page 350]].
