---
splat_schema: 1
splat_id: 13ac50b5-5032-5657-ba21-e3ceef91c9b6
splat_kind: creature
splat_key: dnd2024:creatures/salamander
splat_name: Salamander
splat_origin:
  key: dnd2024:reference/creature/salamander
splat_sources:
- title: SRD 5.2.1
  locator: pp. 321
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: elemental
  alignment: Neutral Evil
  movement_text: 30 ft., Climb 30 ft.
  vulnerabilities_text: Cold
  immunities_text: Fire
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Primordial (Ignan)
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 90 (12d10 + 24)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 11
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 12
  save_charisma:
    value: 1
  armor_class:
    value: 15
  max_hp:
    value: 90
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/salamander
  implementation: dnd2024:creature_salamander
  parameters:
    damage_vulnerabilities:
    - cold
    damage_immunities:
    - fire
    condition_immunities: []
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 10
    languages:
      speak: true
      known:
      - primordial
      primordial_dialects:
      - ignan
    features:
      flame_spear:
        economy: action
        op: attack
        modes:
        - melee
        - ranged
        attack_bonus: 7
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
                count: 2
                sides: 8
              add: 4
            average: 13
          - type: fire
            amount:
              dice:
                count: 2
                sides: 6
              add: 0
            average: 7
        after_attack:
          when: ranged_attack_hit_or_miss
          op: item.return
          destination: actor_hand
          magical: true
      fire_aura:
        economy: passive
        trigger: actor.turn_end
        targets:
          kind: creature
          count: all
          area:
            shape: emanation
            origin: actor
            size_ft: 5
          actor_selects: true
        effects:
        - op: damage
          components:
          - type: fire
            amount:
              dice:
                count: 2
                sides: 6
              add: 0
            average: 7
      constrict:
        economy: action
        op: saving_throw
        ability: strength
        dc: 15
        targets:
          kind: creature
          count: 1
          range_ft: 10
          maximum_size: large
          visible_to_actor: true
        failure:
        - op: damage
          components:
          - type: bludgeoning
            amount:
              dice:
                count: 2
                sides: 6
              add: 4
            average: 11
          - type: fire
            amount:
              dice:
                count: 2
                sides: 6
              add: 0
            average: 7
        - op: grapple
          escape_dc: 14
          while_grappled:
          - op: condition.apply
            condition: restrained
        success: []
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 2
          feature: flame_spear
        replace_attack:
          maximum: 1
          choices:
          - op: feature.use
            feature: constrict
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
- id: dnd2024:creature_salamander
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
  - actor.turn_end
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - item_state
  - saving_throws
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 015920bea6b0280afc1a26555683fb2e62c47b4e5d86b774289658ac4e2d9043
  source_path: builtin/srd-5.2.1/creature/salamander.md
  pages:
  - 321
  engine_gaps: &id001
  - dnd2024:creature_salamander
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 7855a5aad93118d8c666cabcbb0a7c09c0383c155763f5b8122caa82d9c8871a
splat_clauses:
- key: source_rules
  text: '# Salamander Large Elemental, Neutral Evil AC 15 Initiative +2 (12) HP 90 (12d10 + 24) Speed
    30 ft., Climb 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 14 +2 +2 Con 15 +2 +2 Int 11 +0 +0
    Wis 10 +0 +0 Cha 12 +1 +1 Vulnerabilities Cold Immunities Fire Senses Darkvision 60 ft.; Passive Perception
    10 Languages Primordial (Ignan) CR 5 (XP 1,800; PB +3) ### Traits Fire Aura. At the end of each of
    the salamander’s turns, each creature of the salamander’s choice in a 5-foot Emanation originating
    from the salamander takes 7 (2d6) Fire damage. ### Actions Multiattack. The salamander makes two Flame
    Spear attacks. It can replace one attack with a use of Constrict. Flame Spear. Melee or Ranged Attack
    Roll: +7, reach 5 ft. or range 20/60 ft. Hit: 13 (2d8 + 4) Piercing damage plus 7 (2d6) Fire damage.
    Hit or Miss: The spear magically returns to the salamander’s hand immediately after a ranged attack.
    Constrict. Strength Saving Throw: DC 15, one Large or smaller creature the salamander can see within
    10 feet. Failure: 11 (2d6 + 4) Bludgeoning damage plus 7 (2d6) Fire damage. The target has the Grappled
    condition (escape DC 14), and it has the Restrained condition until the grapple ends.'
  sha256: 316ff08bf58ae974230e7783ecaef4eb07db415c11e4004a462ae15101cbe208
  status: contract_specified
splat_requires_mechanics: *id001
---

# Salamander

<!-- SRD page 321 -->

Large Elemental, Neutral Evil\
AC 15  Initiative +2 (12)\
HP 90 (12d10 + 24)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 14 +2 +2 Con 15 +2 +2\
Int 11 +0 +0 Wis 10 +0 +0 Cha 12 +1 +1\
Vulnerabilities Cold\
Immunities Fire\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Primordial (Ignan)\
CR 5 (XP 1,800; PB +3)\

### Traits

Fire Aura. At the end of each of the salamander’s turns,\
each creature of the salamander’s choice in a 5-foot\
Emanation originating from the salamander takes 7\
(2d6) Fire damage.\

### Actions

Multiattack. The salamander makes two Flame\
Spear attacks. It can replace one attack with a use of\
Constrict.\
Flame Spear. Melee or Ranged Attack Roll: +7, reach 5\
ft. or range 20/60 ft. Hit: 13 (2d8 + 4) Piercing damage\
plus 7 (2d6) Fire damage. Hit or Miss: The spear magi-\
cally returns to the salamander’s hand immediately after\
a ranged attack.\
Constrict. Strength Saving Throw: DC 15, one Large or\
smaller creature the salamander can see within 10 feet.\
Failure: 11 (2d6 + 4) Bludgeoning damage plus 7 (2d6)\
Fire damage. The target has the Grappled condition (es-\
cape DC 14), and it has the Restrained condition until\
the grapple ends.\

Original layout: [[../../srd-5.2.1/pages/321|page 321]].
