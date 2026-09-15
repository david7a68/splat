---
splat_schema: 1
splat_id: 58e39c3e-e714-5a0d-8b07-8f3519179edb
splat_kind: creature
splat_key: dnd2024:creatures/vampire_familiar
splat_name: Vampire Familiar
splat_origin:
  key: dnd2024:reference/creature/vampire_familiar
splat_sources:
- title: SRD 5.2.1
  locator: pp. 334
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral Evil
  size_options_text: Medium or Small
  movement_text: 30 ft., Climb 30 ft.
  skills_text: Perception +4, Persuasion +4, Stealth +7
  resistances_text: Necrotic
  immunities_text: Charmed (except from its vampire master)
  gear_text: Daggers (10)
  senses_text: Darkvision 60 ft.; Passive Perception 14
  languages_text: Common plus one other language
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 65 (10d8 + 20)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 16
  save_dexterity:
    value: 5
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 2
  charisma:
    value: 14
  save_charisma:
    value: 2
  armor_class:
    value: 15
  max_hp:
    value: 65
  initiative:
    value: 5
  speed:
    value: 30
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/vampire_familiar
  implementation: dnd2024:creature_vampire_familiar
  parameters:
    skill_totals:
      perception: 4
      persuasion: 4
      stealth: 7
    damage_resistances:
    - necrotic
    damage_immunities: []
    condition_immunities:
    - condition: charmed
      except_source: vampire_master
    gear:
    - item: daggers
      count: 10
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 14
    languages:
      speak: true
      known:
      - common
      additional_choice_count: 1
    features:
      umbral_dagger:
        economy: action
        op: attack
        modes:
        - melee
        - ranged
        attack_bonus: 5
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        range_ft:
          normal: 20
          long: 60
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 1
                sides: 4
              add: 3
            average: 5
          - type: necrotic
            amount:
              dice:
                count: 3
                sides: 4
              add: 0
            average: 7
        on_reduce_target_to_zero_hp:
          effects:
          - op: stable.set
            value: true
          - op: condition.apply
            condition: poisoned
            duration:
              amount: 1
              unit: hour
            while_active:
            - op: condition.apply
              condition: paralyzed
      deathless_agility:
        economy: bonus_action
        op: action.choose
        choices:
        - dash
        - disengage
      vampiric_connection:
        economy: passive
        when:
          same_plane_as: vampire_master
        master_can:
          telepathic_communication: true
          perceive_through_actor_senses: true
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 2
          feature: umbral_dagger
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
- id: dnd2024:creature_vampire_familiar
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
  source_sha256: d15ed621ec87a76fc0d7bb1524da668cba93ffba8ea329de899788f2363caf7c
  source_path: builtin/srd-5.2.1/creature/vampire_familiar.md
  pages:
  - 334
  engine_gaps: &id001
  - dnd2024:creature_vampire_familiar
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 28af8cc4524a15ed65bb54f6acf70b957887a8e0b6fbd034a93fc96cf642e20a
splat_clauses:
- key: source_rules
  text: '# Vampire Familiar Medium or Small Humanoid, Neutral Evil AC 15 Initiative +5 (15) HP 65 (10d8
    + 20) Speed 30 ft., Climb 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +3 Dex 16 +3 +5 Con15 +2 +2
    Int 10 +0 +0 Wis 10 +0 +2 Cha 14 +2 +2 Skills Perception +4, Persuasion +4, Stealth +7 Resistances
    Necrotic Immunities Charmed (except from its vampire master) Gear Daggers (10) Senses Darkvision 60
    ft.; Passive Perception 14 Languages Common plus one other language CR 3 (XP 700; PB +2) ### Traits
    Vampiric Connection. While the familiar and its vampire master are on the same plane of existence,
    the vampire can communicate with the familiar telepathically, and the vampire can perceive through
    the familiar’s senses. ### Actions Multiattack. The familiar makes two Umbral Dagger attacks. Umbral
    Dagger. Melee or Ranged Attack Roll: +5, reach 5 ft. or range 20/60 ft. Hit: 5 (1d4 + 3) Piercing
    damage plus 7 (3d4) Necrotic damage. If the target is reduced to 0 Hit Points by this attack, the
    target becomes Stable but has the Poisoned condition for 1 hour. While it has the Poisoned condition,
    the target has the Paralyzed condition. ### Bonus Actions Deathless Agility. The familiar takes the
    Dash or Disengage action.'
  sha256: 8765346c6e2e5e442b42942a0f3eac81cff046beed3594240e6306ed6bacead7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Vampire Familiar

<!-- SRD page 334 -->

Medium or Small Humanoid, Neutral Evil\
AC 15  Initiative +5 (15)\
HP 65 (10d8 + 20)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 16 +3 +5 Con15 +2 +2\
Int 10 +0 +0 Wis 10 +0 +2 Cha 14 +2 +2\
Skills Perception +4, Persuasion +4, Stealth +7\
Resistances Necrotic\
Immunities Charmed (except from its vampire master)\
Gear Daggers (10)\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages Common plus one other language\
CR 3 (XP 700; PB +2)\

### Traits

Vampiric Connection. While the familiar and its vam-\
pire master are on the same plane of existence, the\
vampire can communicate with the familiar telepathi-\
cally, and the vampire can perceive through the famil-\
iar’s senses.\

### Actions

Multiattack. The familiar makes two Umbral Dag-\
ger attacks.\
Umbral Dagger. Melee or Ranged Attack Roll: +5,\
reach 5 ft. or range 20/60 ft. Hit: 5 (1d4 + 3) Piercing\
damage plus 7 (3d4) Necrotic damage. If the target\
is reduced to 0 Hit Points by this attack, the target\
becomes Stable but has the Poisoned condition for 1\
hour. While it has the Poisoned condition, the target\
has the Paralyzed condition.\

### Bonus Actions

Deathless Agility. The familiar takes the Dash or Disen-\
gage action.\

Original layout: [[../../srd-5.2.1/pages/334|page 334]].
