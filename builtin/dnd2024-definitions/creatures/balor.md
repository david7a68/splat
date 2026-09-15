---
splat_schema: 1
splat_id: 8743edd3-aa74-5b44-99f7-b3b30b153480
splat_kind: creature
splat_key: dnd2024:creatures/balor
splat_name: Balor
splat_origin:
  key: dnd2024:reference/creature/balor
splat_sources:
- title: SRD 5.2.1
  locator: pp. 261
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: fiend
  alignment: Chaotic Evil
  tags_text: (Demon)
  movement_text: 40 ft., Fly 80 ft.
  skills_text: Perception +9
  resistances_text: Cold, Lightning
  immunities_text: Fire, Poison; Charmed, Frightened, Poisoned
  senses_text: Truesight 120 ft.; Passive Perception 19
  languages_text: Abyssal; telepathy 120 ft.
  cr_text: 19 (XP 22,000; PB +6)
  hit_points_text: 287 (23d12 + 138)
splat_stats:
  strength:
    value: 26
  save_strength:
    value: 8
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 22
  save_constitution:
    value: 12
  intelligence:
    value: 20
  save_intelligence:
    value: 5
  wisdom:
    value: 16
  save_wisdom:
    value: 9
  charisma:
    value: 22
  save_charisma:
    value: 6
  armor_class:
    value: 19
  max_hp:
    value: 287
  initiative:
    value: 14
  speed:
    value: 40
    unit: ft
  fly_speed:
    value: 80
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/balor
  implementation: dnd2024:creature_balor
  parameters:
    features:
      legendary_resistance:
        economy: passive
        resource:
          capacity: 3
          recovery: long_rest
          cost: 1
        trigger: saving_throw.failed
        optional: true
        op: replace_save_outcome
        outcome: success
      magic_resistance:
        economy: passive
        advantage:
          roll: saving_throw
          against:
          - spell
          - magical_effect
      flame_whip:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 14
        targets:
          count: 1
        hit:
        - op: damage
          average: 18
          amount:
            dice:
              count: 3
              sides: 6
            add: 8
          damage_type: force
        - op: damage
          average: 17
          amount:
            dice:
              count: 5
              sides: 6
            add: 0
          damage_type: fire
        - op: if
          test:
            target_kind: creature
            target_size_max: huge
          then:
          - op: pull
            distance_max_feet: 25
            direction: straight_toward_self
          - op: condition
            condition: prone
        reach_feet: 30
      lightning_blade:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 14
        targets:
          count: 1
        hit:
        - op: damage
          average: 21
          amount:
            dice:
              count: 3
              sides: 8
            add: 8
          damage_type: force
        - op: damage
          average: 22
          amount:
            dice:
              count: 4
              sides: 10
            add: 0
          damage_type: lightning
        - op: forbid_economy
          economy:
          - reaction
          ends:
            event: turn.start
            participant: self
            occurrence: next
        reach_feet: 10
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: flame_whip
          count: 1
        - op: use_feature
          feature: lightning_blade
          count: 1
      death_throes:
        economy: passive
        trigger: self.died
        op: sequence
        steps:
        - op: save
          ability: dexterity
          dc: 20
          targets:
            kind: creature
            count: all
            area:
              shape: emanation
              size_feet: 30
              origin: self
          failure:
          - op: damage
            average: 31
            amount:
              dice:
                count: 9
                sides: 6
              add: 0
            damage_type: fire
          - op: damage
            average: 31
            amount:
              dice:
                count: 9
                sides: 6
              add: 0
            damage_type: force
          success:
          - op: damage_fraction
            numerator: 1
            denominator: 2
            round: down
        - op: if
          test:
            death_plane_not: abyss
          then:
          - op: revive
            new_body: true
            hit_points: maximum
            delay:
              seconds: 0
            destination:
              plane: abyss
              location_choice: gm
      fire_aura:
        economy: passive
        trigger: self.turn.end
        targets:
          kind: creature
          count: all
          area:
            shape: emanation
            size_feet: 5
            origin: self
        effects:
        - op: damage
          average: 13
          amount:
            dice:
              count: 3
              sides: 8
            add: 0
          damage_type: fire
      teleport:
        economy: bonus_action
        op: teleport
        targets:
          choice:
          - self
          - willing_demon
          target_range_feet: 10
        range_feet: 60
        destination:
          unoccupied: true
          visible_to: self
    skill_totals:
      perception: 9
    defenses:
      resistances:
        damage_types:
        - cold
        - lightning
      immunities:
        damage_types:
        - fire
        - poison
        conditions:
        - charmed
        - frightened
        - poisoned
    senses:
      passive_perception: 19
      special:
      - sense: truesight
        range_feet: 120
    communication:
      speaks:
      - abyssal
      understands:
      - abyssal
      telepathy:
        range_feet: 120
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
- id: dnd2024:creature_balor
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
  - saving_throw.failed
  - self.died
  - self.turn.end
  - turn.start
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - legendary_actions
  - movement
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 7e2f462dfc30899c11089ef10a169aa5f92597f8a1535a5e359b7f011923af9d
  source_path: builtin/srd-5.2.1/creature/balor.md
  pages:
  - 261
  engine_gaps: &id001
  - dnd2024:creature_balor
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 8d27f077fbd09ddc7cd3bcc46919630a29d7281c40e413b3598a7fc58095c2cf
splat_clauses:
- key: source_rules
  text: '# Balor Huge Fiend (Demon), Chaotic Evil AC 19 Initiative +14 (24) HP 287 (23d12 + 138) Speed
    40 ft., Fly 80 ft. MOD SAVE MOD SAVE MOD SAVE Str 26 +8 +8 Dex 15 +2 +2 Con 22 +6 +12 Int 20 +5 +5
    Wis 16 +3 +9 Cha 22 +6 +6 Skills Perception +9 Resistances Cold, Lightning Immunities Fire, Poison;
    Charmed, Frightened, Poisoned Senses Truesight 120 ft.; Passive Perception 19 Languages Abyssal; telepathy
    120 ft. CR 19 (XP 22,000; PB +6) ### Traits Death Throes. The balor explodes when it dies. Dexterity
    Saving Throw: DC 20, each creature in a 30-foot Emanation originating from the balor. Failure: 31
    (9d6) Fire damage plus 31 (9d6) Force damage. Success: Half damage. Failure or Success: If the balor
    dies outside the Abyss, it gains a new body instantly, reviving with all its Hit Points somewhere
    in the Abyss. Fire Aura. At the end of each of the balor’s turns, each creature in a 5-foot Emanation
    originating from the balor takes 13 (3d8) Fire damage. Legendary Resistance (3/Day). If the balor
    fails a saving throw, it can choose to succeed instead. Magic Resistance. The balor has Advantage
    on saving throws against spells and other magical effects. ### Actions Multiattack. The balor makes
    one Flame Whip attack and one Lightning Blade attack. Flame Whip. Melee Attack Roll: +14, reach 30
    ft. Hit: 18 (3d6 + 8) Force damage plus 17 (5d6) Fire damage. If the target is a Huge or smaller creature,
    the balor pulls the target up to 25 feet straight toward itself, and the target has the Prone condition.
    Lightning Blade. Melee Attack Roll: +14, reach 10 ft. Hit: 21 (3d8 + 8) Force damage plus 22 (4d10)
    Lightning damage, and the target can’t take Reactions until the start of the balor’s next turn. ###
    Bonus Actions Teleport. The balor teleports itself or a willing demon within 10 feet of itself up
    to 60 feet to an unoccupied space the balor can see.'
  sha256: 67be1f6deb81d416cc612d8c0920af643a6f7bc434ae3416f21d085b83e45db4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Balor

<!-- SRD page 261 -->

Huge Fiend (Demon), Chaotic Evil\
AC 19 Initiative +14 (24)\
HP 287 (23d12 + 138)\
Speed 40 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 26 +8 +8 Dex 15 +2 +2 Con 22 +6 +12\
Int 20 +5 +5 Wis 16 +3 +9 Cha 22 +6 +6\
Skills Perception +9\
Resistances Cold, Lightning\
Immunities Fire, Poison; Charmed, Frightened, Poisoned\
Senses Truesight 120 ft.; Passive Perception 19\
Languages Abyssal; telepathy 120 ft.\
CR 19 (XP 22,000; PB +6)\

### Traits

Death Throes. The balor explodes when it dies. Dex-\
terity Saving Throw: DC 20, each creature in a 30-foot\
Emanation originating from the balor. Failure: 31 (9d6)\
Fire damage plus 31 (9d6) Force damage. Success: Half\
damage. Failure or Success: If the balor dies outside the\
Abyss, it gains a new body instantly, reviving with all its\
Hit Points somewhere in the Abyss.\
Fire Aura. At the end of each of the balor’s turns, each\
creature in a 5-foot Emanation originating from the ba-\
lor takes 13 (3d8) Fire damage.\
Legendary Resistance (3/Day). If the balor fails a saving\
throw, it can choose to succeed instead.\
Magic Resistance. The balor has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Multiattack. The balor makes one Flame Whip attack\
and one Lightning Blade attack.\
Flame Whip. Melee Attack Roll: +14, reach 30 ft. Hit:\
18 (3d6 + 8) Force damage plus 17 (5d6) Fire damage. If\
the target is a Huge or smaller creature, the balor pulls\
the target up to 25 feet straight toward itself, and the\
target has the Prone condition.\
Lightning Blade. Melee Attack Roll: +14, reach 10 ft.\
Hit: 21 (3d8 + 8) Force damage plus 22 (4d10) Light-\
ning damage, and the target can’t take Reactions until\
the start of the balor’s next turn.\

### Bonus Actions

Teleport. The balor teleports itself or a willing demon\
within 10 feet of itself up to 60 feet to an unoccupied\
space the balor can see.\

Original layout: [[../../srd-5.2.1/pages/261|page 261]].
