---
splat_schema: 1
splat_id: 356c36da-db91-55f8-851e-b672168583ef
splat_kind: creature
splat_key: dnd2024:creatures/nalfeshnee
splat_name: Nalfeshnee
splat_origin:
  key: dnd2024:reference/creature/nalfeshnee
splat_sources:
- title: SRD 5.2.1
  locator: pp. 310, 311
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: fiend
  alignment: Chaotic Evil
  tags_text: (Demon)
  movement_text: 20 ft., Fly 30 ft.
  resistances_text: Cold, Fire, Lightning
  immunities_text: Poison; Frightened, Poisoned
  senses_text: Truesight 120 ft.; Passive Perception 11
  languages_text: Abyssal; telepathy 120 ft.
  cr_text: 13 (XP 10,000; PB +5)
  hit_points_text: 184 (16d10 + 96)
splat_stats:
  strength:
    value: 21
  save_strength:
    value: 5
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 22
  save_constitution:
    value: 11
  intelligence:
    value: 19
  save_intelligence:
    value: 9
  wisdom:
    value: 12
  save_wisdom:
    value: 6
  charisma:
    value: 15
  save_charisma:
    value: 7
  armor_class:
    value: 18
  max_hp:
    value: 184
  initiative:
    value: 5
  speed:
    value: 20
    unit: ft
  fly_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/nalfeshnee
  implementation: dnd2024:creature_nalfeshnee
  parameters:
    features:
      magic_resistance:
        economy: passive
        advantage:
          roll: saving_throw
          against:
          - spell
          - magical_effect
      rend:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 10
        targets:
          count: 1
        hit:
        - op: damage
          average: 16
          amount:
            dice:
              count: 2
              sides: 10
            add: 5
          damage_type: slashing
        - op: damage
          average: 11
          amount:
            dice:
              count: 2
              sides: 10
            add: 0
          damage_type: force
        reach_feet: 10
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: rend
          count: 3
      demonic_restoration:
        economy: passive
        resource:
          capacity: 1
          recovery:
          - long_rest
          cost: 1
        trigger: self.died
        requires:
          death_plane_not: abyss
        op: revive
        new_body: true
        old_body: disappears
        delay:
          seconds: 0
        hit_points: maximum
        destination:
          plane: abyss
          location_choice: gm
      teleport:
        economy: action
        op: teleport
        targets: self
        range_feet: 120
        destination:
          unoccupied: true
          visible_to: self
      horror_nimbus:
        economy: bonus_action
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
            - 5
            - 6
          cost: 1
          recovery_also:
          - short_rest
          - long_rest
        op: save
        ability: wisdom
        dc: 15
        targets:
          kind: creature
          count: all
          area:
            shape: emanation
            size_feet: 15
            origin: self
        failure:
        - op: damage
          average: 28
          amount:
            dice:
              count: 8
              sides: 6
            add: 0
          damage_type: psychic
        - op: condition
          condition: frightened
          ends:
            any:
            - minutes: 1
            - event: target.damage_taken
            - event: target.turn.end
              self_out_of_target_line_of_sight: true
        success:
        - op: feature_immunity
          source: self
          feature: horror_nimbus
          duration:
            hours: 24
      pursuit:
        economy: reaction
        trigger: another_creature.move.end
        requires:
          triggering_creature_visible: true
          triggering_creature_distance_max_feet: 120
        op: use_feature
        feature: teleport
        destination_constraints:
          distance_to_triggering_creature_max_feet: 10
    defenses:
      resistances:
        damage_types:
        - cold
        - fire
        - lightning
      immunities:
        damage_types:
        - poison
        conditions:
        - frightened
        - poisoned
    senses:
      passive_perception: 11
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
- id: dnd2024:creature_nalfeshnee
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
  - another_creature.move.end
  - self.died
  - target.damage_taken
  - target.turn.end
  - turn.start
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - movement
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 77f7cc995fa311e9d79c520abdf78a37ec82b42647367c3ee6869f5d8afb7a0c
  source_path: builtin/srd-5.2.1/creature/nalfeshnee.md
  pages:
  - 310
  - 311
  engine_gaps: &id001
  - dnd2024:creature_nalfeshnee
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: ac9b4289a890ed86ad5d1daff5d6c91b542507e117a2dddee1ca84b51dc127e5
splat_clauses:
- key: source_rules
  text: '# Nalfeshnee Large Fiend (Demon), Chaotic Evil AC 18 Initiative +5 (15) HP 184 (16d10 + 96) Speed
    20 ft., Fly 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 21 +5 +5 Dex 10 +0 +0 Con 22 +6 +11 Int 19 +4 +9
    Wis 12 +1 +6 Cha 15 +2 +7 Resistances Cold, Fire, Lightning Immunities Poison; Frightened, Poisoned
    Senses Truesight 120 ft.; Passive Perception 11 Languages Abyssal; telepathy 120 ft. CR 13 (XP 10,000;
    PB +5) ### Traits Demonic Restoration. If the nalfeshnee dies outside the Abyss, its body dissolves
    into ichor, and it gains a new body instantly, reviving with all its Hit Points somewhere in the Abyss.
    Magic Resistance. The nalfeshnee has Advantage on saving throws against spells and other magical effects.
    ### Actions Multiattack. The nalfeshnee makes three Rend attacks. Rend. Melee Attack Roll: +10, reach
    10 ft. Hit: 16 (2d10 + 5) Slashing damage plus 11 (2d10) Force damage. Teleport. The nalfeshnee teleports
    up to 120 feet to an unoccupied space it can see. ### Bonus Actions Horror Nimbus (Recharge 5–6).
    Wisdom Saving Throw: DC 15, each creature in a 15-foot Emanation originating from the nalfeshnee.
    Failure: 28 (8d6) Psychic damage, and the target has the Frightened condition for 1 minute, until
    it takes damage, or until it ends its turn with the nalfeshnee out of line of sight. Success: The
    target is immune to this nalfeshnee’s Horror Nimbus for 24 hours. ### Reactions Pursuit. Trigger:
    Another creature the nalfeshnee can see ends its move within 120 feet of the nalfeshnee. Response:
    The nalfeshnee uses Teleport, but its destination space must be within 10 feet of the triggering creature.'
  sha256: 8409d5db133d3de7545c3bdf921a0c527f5f08db8c6bdb9ea215dfebc34c22c5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Nalfeshnee

<!-- SRD page 310 -->

Large Fiend (Demon), Chaotic Evil\
AC 18 Initiative +5 (15)\
HP 184 (16d10 + 96)\
Speed 20 ft., Fly 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 21 +5 +5 Dex 10 +0 +0 Con 22 +6 +11\
Int 19 +4 +9 Wis 12 +1 +6 Cha 15 +2 +7\
Resistances Cold, Fire, Lightning\
Immunities Poison; Frightened, Poisoned\
Senses Truesight 120 ft.; Passive Perception 11\
Languages Abyssal; telepathy 120 ft.\
CR 13 (XP 10,000; PB +5)\

### Traits

Demonic Restoration. If the nalfeshnee dies outside the\
Abyss, its body dissolves into ichor, and it gains a new\
body instantly, reviving with all its Hit Points some-\
where in the Abyss.\
Magic Resistance. The nalfeshnee has Advantage on\
saving throws against spells and other magical effects.\

### Actions

Multiattack. The nalfeshnee makes three Rend attacks.\
Rend. Melee Attack Roll: +10, reach 10 ft. Hit: 16 (2d10\
+ 5) Slashing damage plus 11 (2d10) Force damage.\
Teleport. The nalfeshnee teleports up to 120 feet to an\
unoccupied space it can see.\

### Bonus Actions

Horror Nimbus (Recharge 5–6). Wisdom Saving\
Throw: DC 15, each creature in a 15-foot Emanation\
originating from the nalfeshnee. Failure: 28 (8d6) Psy-\
chic damage, and the target has the Frightened condi-\
tion for 1 minute, until it takes damage, or until it ends\
its turn with the nalfeshnee out of line of sight. Success:\

<!-- SRD page 311 -->

The target is immune to this nalfeshnee’s Horror Nim-\
bus for 24 hours.\

### Reactions

Pursuit. Trigger: Another creature the nalfeshnee can\
see ends its move within 120 feet of the nalfeshnee.\
Response: The nalfeshnee uses Teleport, but its des-\
tination space must be within 10 feet of the trigger-\
ing creature.\

Original layout: [[../../srd-5.2.1/pages/310|page 310]], [[../../srd-5.2.1/pages/311|page 311]].
