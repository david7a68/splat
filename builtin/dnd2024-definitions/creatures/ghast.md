---
splat_schema: 1
splat_id: 273b9cd7-f749-5b98-a216-65592f4c1360
splat_kind: creature
splat_key: dnd2024:creatures/ghast
splat_name: Ghast
splat_origin:
  key: dnd2024:reference/creature/ghast
splat_sources:
- title: SRD 5.2.1
  locator: pp. 287
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: undead
  alignment: Chaotic Evil
  movement_text: 30 ft.
  resistances_text: Necrotic
  immunities_text: Poison; Charmed, Exhaustion, Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Common
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 36 (8d8)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 17
  save_dexterity:
    value: 3
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 11
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 2
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 13
  max_hp:
    value: 36
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/ghast
  implementation: dnd2024:creature_ghast
  parameters:
    features:
      bite:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 7
          amount:
            dice:
              count: 1
              sides: 8
            add: 3
          damage_type: piercing
        - op: damage
          average: 9
          amount:
            dice:
              count: 2
              sides: 8
            add: 0
          damage_type: necrotic
        reach_feet: 5
      claw:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 10
          amount:
            dice:
              count: 2
              sides: 6
            add: 3
          damage_type: slashing
        - op: if
          test:
            target_kind: creature
            target_type_not: undead
          then:
          - op: save
            ability: constitution
            dc: 10
            targets:
              participant: target
            failure:
            - op: condition
              condition: paralyzed
              ends:
                event: turn.end
                participant: target
                occurrence: next
            success: []
        reach_feet: 5
      stench:
        economy: passive
        trigger: creature.turn.start
        op: save
        ability: constitution
        dc: 10
        targets:
          kind: creature
          count: all
          area:
            shape: emanation
            size_feet: 5
            origin: self
        failure:
        - op: condition
          condition: poisoned
          ends:
            event: turn.start
            participant: target
            occurrence: next
        success:
        - op: feature_immunity
          feature: stench
          source: self
          duration:
            hours: 24
    defenses:
      resistances:
        damage_types:
        - necrotic
      immunities:
        damage_types:
        - poison
        conditions:
        - charmed
        - exhaustion
        - poisoned
    senses:
      passive_perception: 10
      special:
      - sense: darkvision
        range_feet: 60
    communication:
      speaks:
      - common
      understands:
      - common
    stat_block_conventions:
      attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
      save_dc: supplied_total
      damage_mode_choice:
      - fixed_average
      - rolled_dice
      condition_source: self
      multiattack_child_economy_cost: 0
      multiattack_child_resource_costs_retained: true
      spellcasting:
        default_level: spell_minimum
        spell_slots_expended: false
        required_materials: available
        casting_time_at_least_one_minute:
          uses_normal_casting_duration: true
          magic_action_each_turn: true
          concentration_required: true
      geometry_assumptions:
        chooser: gm
        supplies:
        - range
        - visibility
        - line_of_sight
        - area_membership
        - reachable_locations
        - unoccupied_locations
        - terrain
        - plane
        - form_appearance
splat_contracts:
- id: dnd2024:creature_ghast
  inputs:
    self:
      type: participant
      required: true
    targets:
      required: false
      type: participant_list
    feature:
      type: choice
      required: true
    event:
      required: false
      type: event
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - resolved_feature
  - effects
  - resource_changes
  events:
  - creature.turn.start
  - turn.end
  - turn.start
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 6aa15f013f66af5305a209ed6ad3ca25cb4de569451b5ae746e16a9157b04fa7
  source_path: builtin/srd-5.2.1/creature/ghast.md
  pages:
  - 287
  engine_gaps: &id001
  - dnd2024:creature_ghast
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: e296275b5238a7a1b6f9a10b624f695e4610c448d281e201d84467fd83e275cf
splat_clauses:
- key: source_rules
  text: '# Ghast Medium Undead, Chaotic Evil AC 13 Initiative +3 (13) HP 36 (8d8) Speed 30 ft. MOD SAVE
    MOD SAVE MOD SAVE Str 16 +3 +3 Dex 17 +3 +3 Con 10 +0 +0 Int 11 +0 +0 Wis 10 +0 +2 Cha 8 -1 -1 Resistances
    Necrotic Immunities Poison; Charmed, Exhaustion, Poisoned Senses Darkvision 60 ft.; Passive Perception
    10 Languages Common CR 2 (XP 450; PB +2) ### Traits Stench. Constitution Saving Throw: DC 10, any
    creature that starts its turn in a 5-foot Emanation originating from the ghast. Failure: The target
    has the Poisoned condition until the start of its next turn. Success: The target is immune to this
    ghast’s Stench for 24 hours. ### Actions Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3)
    Piercing damage plus 9 (2d8) Necrotic damage. Claw. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6
    + 3) Slashing damage. If the target is a non-Undead creature, it is subjected to the following effect.
    Constitution Saving Throw: DC 10. Failure: The target has the Paralyzed condition until the end of
    its next turn.'
  sha256: 8b77bbe80fe209aea45dda17c1eb4d6fb98f05fcf43e670f2a1ae3ee136ff5a7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ghast

<!-- SRD page 287 -->

Medium Undead, Chaotic Evil\
AC 13 Initiative +3 (13)\
HP 36 (8d8)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 17 +3 +3 Con 10 +0 +0\
Int 11 +0 +0 Wis 10 +0 +2 Cha 8 −1 −1\
Resistances Necrotic\
Immunities Poison; Charmed, Exhaustion, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Common\
CR 2 (XP 450; PB +2)\

### Traits

Stench. Constitution Saving Throw: DC 10, any creature\
that starts its turn in a 5-foot Emanation originating from\
the ghast. Failure: The target has the Poisoned condi-\
tion until the start of its next turn. Success: The target is\
immune to this ghast’s Stench for 24 hours.\

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3)\
Piercing damage plus 9 (2d8) Necrotic damage.\
Claw. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6\
+ 3) Slashing damage. If the target is a non-Undead\
creature, it is subjected to the following effect. Consti-\
tution Saving Throw: DC 10. Failure: The target has the\
Paralyzed condition until the end of its next turn.\

Original layout: [[../../srd-5.2.1/pages/287|page 287]].
