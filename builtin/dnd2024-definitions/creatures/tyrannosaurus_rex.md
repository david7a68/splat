---
splat_schema: 1
splat_id: 4ab35862-e512-5bcd-8355-32675ab13c81
splat_kind: creature
splat_key: dnd2024:creatures/tyrannosaurus_rex
splat_name: Tyrannosaurus Rex
splat_origin:
  key: dnd2024:reference/creature/tyrannosaurus_rex
splat_sources:
- title: SRD 5.2.1
  locator: pp. 363
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: beast
  alignment: Unaligned
  tags_text: (Dinosaur)
  movement_text: 50 ft.
  skills_text: Perception +4
  senses_text: Passive Perception 14
  languages_text: None
  cr_text: 8 (XP 3,900; PB +3)
  hit_points_text: 136 (13d12 + 52)
splat_stats:
  strength:
    value: 25
  save_strength:
    value: 10
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 19
  save_constitution:
    value: 4
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 4
  charisma:
    value: 9
  save_charisma:
    value: -1
  armor_class:
    value: 13
  max_hp:
    value: 136
  initiative:
    value: 3
  speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/tyrannosaurus_rex
  implementation: dnd2024:creature_tyrannosaurus_rex
  parameters:
    skill_totals:
      perception: 4
    senses: []
    passive_perception: 14
    languages:
      speak: true
      known: []
    features:
      bite:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 10
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 4
                sides: 12
              add: 7
            average: 33
        - op: grapple
          escape_dc: 17
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
        attack_bonus: 10
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 15
        hit:
        - op: damage
          components:
          - type: bludgeoning
            amount:
              dice:
                count: 4
                sides: 8
              add: 7
            average: 25
        - op: condition.apply
          condition: prone
          when:
            target_kind: creature
            target_maximum_size: huge
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
- id: dnd2024:creature_tyrannosaurus_rex
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
  source_sha256: 398177586c9389c191e77a31b627a2d424980cec0afa17510cca53e621fa1eb8
  source_path: builtin/srd-5.2.1/creature/tyrannosaurus_rex.md
  pages:
  - 363
  engine_gaps: &id001
  - dnd2024:creature_tyrannosaurus_rex
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 760f1c39dc548e352aafb941aa9f59ab263718005aa64c7a97783f1981e1300b
splat_clauses:
- key: source_rules
  text: '# Tyrannosaurus Rex Huge Beast (Dinosaur), Unaligned AC 13 Initiative +3 (13) HP 136 (13d12 +
    52) Speed 50 ft. MOD SAVE MOD SAVE MOD SAVE Str 25 +7 +10 Dex 10 +0 +0 Con 19 +4 +4 Int 2 -4 -4 Wis
    12 +1 +4 Cha 9 -1 -1 Skills Perception +4 Senses Passive Perception 14 Languages None CR 8 (XP 3,900;
    PB +3) ### Actions Multiattack. The tyrannosaurus makes one Bite attack and one Tail attack. Bite.
    Melee Attack Roll: +10, reach 10 ft. Hit: 33 (4d12 + 7) Piercing damage. If the target is a Large
    or smaller creature, it has the Grappled condition (escape DC 17). While Grappled, the target has
    the Restrained condition and can’t be targeted by the tyrannosaurus’s Tail. Tail. Melee Attack Roll:
    +10, reach 15 ft. Hit: 25 (4d8 + 7) Bludgeoning damage. If the target is a Huge or smaller creature,
    it has the Prone condition.'
  sha256: 62684ca5584729e7d461613f2676bf7277750db16b4aa3b16475c0dc95715f63
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tyrannosaurus Rex

<!-- SRD page 363 -->

Huge Beast (Dinosaur), Unaligned\
AC 13 Initiative +3 (13)\
HP 136 (13d12 + 52)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str  25 +7 +10 Dex 10 +0 +0 Con 19 +4 +4\
Int 2 −4 −4 Wis 12 +1 +4 Cha 9 −1 −1\
Skills Perception +4\
Senses Passive Perception 14\
Languages None\
CR 8 (XP 3,900; PB +3)\

### Actions

Multiattack. The tyrannosaurus makes one Bite attack\
and one Tail attack.\
Bite. Melee Attack Roll: +10, reach 10 ft. Hit: 33 (4d12\
+ 7) Piercing damage. If the target is a Large or smaller\
creature, it has the Grappled condition (escape DC 17).\
While Grappled, the target has the Restrained condition\
and can’t be targeted by the tyrannosaurus’s Tail.\
Tail. Melee Attack Roll: +10, reach 15 ft. Hit: 25 (4d8\
+ 7) Bludgeoning damage. If the target is a Huge or\
smaller creature, it has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/363|page 363]].
