---
splat_schema: 1
splat_id: ce8d49a6-733a-5061-9adb-1c1c5c142862
splat_kind: creature
splat_key: dnd2024:creatures/magma_mephit
splat_name: Magma Mephit
splat_origin:
  key: dnd2024:reference/creature/magma_mephit
splat_sources:
- title: SRD 5.2.1
  locator: pp. 307
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: elemental
  alignment: Neutral Evil
  movement_text: 30 ft., Fly 30 ft.
  skills_text: Stealth +3
  vulnerabilities_text: Cold
  immunities_text: Fire, Poison; Exhaustion, Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Primordial (Ignan, Terran)
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 18 (4d6 + 4)
splat_stats:
  strength:
    value: 8
  save_strength:
    value: -1
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 7
  save_intelligence:
    value: -2
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 10
  save_charisma:
    value: 0
  armor_class:
    value: 11
  max_hp:
    value: 18
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/magma_mephit
  implementation: dnd2024:creature_magma_mephit
  parameters:
    features:
      claw:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 3
        targets:
          count: 1
        hit:
        - op: damage
          average: 3
          amount:
            dice:
              count: 1
              sides: 4
            add: 1
          damage_type: slashing
        - op: damage
          average: 3
          amount:
            dice:
              count: 1
              sides: 6
            add: 0
          damage_type: fire
        reach_feet: 5
      fire_breath:
        economy: action
        resource:
          capacity: 1
          initial: 1
          recharge:
            event: turn.start
            participant: self
            roll:
              dice:
                count: 1
                sides: 6
              add: 0
            succeeds_on:
            - 6
          cost: 1
          recovery_also:
          - short_rest
          - long_rest
        op: save
        ability: dexterity
        dc: 11
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            size_feet: 15
            origin: self
        failure:
        - op: damage
          average: 7
          amount:
            dice:
              count: 2
              sides: 6
            add: 0
          damage_type: fire
        success:
        - op: damage_fraction
          numerator: 1
          denominator: 2
          round: down
      death_burst:
        economy: passive
        trigger: self.died
        op: save
        ability: dexterity
        dc: 11
        targets:
          kind: creature
          count: all
          area:
            shape: emanation
            size_feet: 5
            origin: self
        failure:
        - op: damage
          average: 7
          amount:
            dice:
              count: 2
              sides: 6
            add: 0
          damage_type: fire
        success:
        - op: damage_fraction
          numerator: 1
          denominator: 2
          round: down
    skill_totals:
      stealth: 3
    defenses:
      vulnerabilities:
        damage_types:
        - cold
      immunities:
        damage_types:
        - fire
        - poison
        conditions:
        - exhaustion
        - poisoned
    senses:
      passive_perception: 10
      special:
      - sense: darkvision
        range_feet: 60
    communication:
      speaks:
      - primordial
      understands:
      - primordial
      dialects:
      - ignan
      - terran
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
- id: dnd2024:creature_magma_mephit
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
  - self.died
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
  source_sha256: b079e5c99a0adfb5306172d2d7d6ea68149c238390624e69f6a8f5939d52b963
  source_path: builtin/srd-5.2.1/creature/magma_mephit.md
  pages:
  - 307
  engine_gaps: &id001
  - dnd2024:creature_magma_mephit
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 15facb81d73549d624a00babb3a730a7e11fc8d44e98c098c26c6b381753dc63
splat_clauses:
- key: source_rules
  text: '# Magma Mephit Small Elemental, Neutral Evil AC 11 Initiative +1 (11) HP 18 (4d6 + 4) Speed 30
    ft., Fly 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 8 -1 -1 Dex 12 +1 +1 Con 12 +1 +1 Int 7 -2 -2 Wis 10
    +0 +0 Cha 10 +0 +0 Skills Stealth +3 Vulnerabilities Cold Immunities Fire, Poison; Exhaustion, Poisoned
    Senses Darkvision 60 ft.; Passive Perception 10 Languages Primordial (Ignan, Terran) CR 1/2 (XP 100;
    PB +2) ### Traits Death Burst. The mephit explodes when it dies. Dexterity Saving Throw: DC 11, each
    creature in a 5-foot Emanation originating from the mephit. Failure: 7 (2d6) Fire damage. Success:
    Half damage. ### Actions Claw. Melee Attack Roll: +3, reach 5 ft. Hit: 3 (1d4 + 1) Slashing damage
    plus 3 (1d6) Fire damage. Fire Breath (Recharge 6). Dexterity Saving Throw: DC 11, each creature in
    a 15-foot Cone. Failure: 7 (2d6) Fire damage. Success: Half damage.'
  sha256: cfb519d0531bc0d5030b5c01be9f970f79dee8339050e1de006adcfc20a01547
  status: contract_specified
splat_requires_mechanics: *id001
---

# Magma Mephit

<!-- SRD page 307 -->

Small Elemental, Neutral Evil\
AC 11 Initiative +1 (11)\
HP 18 (4d6 + 4)\
Speed 30 ft., Fly 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 8 −1 −1 Dex 12 +1 +1 Con 12 +1 +1\
Int 7 −2 −2 Wis 10 +0 +0 Cha 10 +0 +0\
Skills Stealth +3\
Vulnerabilities Cold\
Immunities Fire, Poison; Exhaustion, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Primordial (Ignan, Terran)\
CR 1/2 (XP 100; PB +2)\

### Traits

Death Burst. The mephit explodes when it dies. Dex-\
terity Saving Throw: DC 11, each creature in a 5-foot\
Emanation originating from the mephit. Failure: 7 (2d6)\
Fire damage. Success: Half damage.\

### Actions

Claw. Melee Attack Roll: +3, reach 5 ft. Hit: 3 (1d4 + 1)\
Slashing damage plus 3 (1d6) Fire damage.\
Fire Breath (Recharge 6). Dexterity Saving Throw: DC\
11, each creature in a 15-foot Cone. Failure: 7 (2d6) Fire\
damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/307|page 307]].
