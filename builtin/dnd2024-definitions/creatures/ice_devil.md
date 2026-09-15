---
splat_schema: 1
splat_id: a33a2ae8-8473-5e89-90b2-20d2665bb846
splat_kind: creature
splat_key: dnd2024:creatures/ice_devil
splat_name: Ice Devil
splat_origin:
  key: dnd2024:reference/creature/ice_devil
splat_sources:
- title: SRD 5.2.1
  locator: pp. 300
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: fiend
  alignment: Lawful Evil
  tags_text: (Devil)
  movement_text: 40 ft.
  skills_text: Insight +7, Perception +7, Persuasion +9
  immunities_text: Cold, Fire, Poison; Poisoned
  senses_text: Blindsight 120 ft.; Passive Perception 17
  languages_text: Infernal; telepathy 120 ft.
  cr_text: 14 (XP 11,500; PB +5)
  hit_points_text: 228 (24d10 + 96)
splat_stats:
  strength:
    value: 21
  save_strength:
    value: 5
  dexterity:
    value: 14
  save_dexterity:
    value: 7
  constitution:
    value: 18
  save_constitution:
    value: 9
  intelligence:
    value: 18
  save_intelligence:
    value: 4
  wisdom:
    value: 15
  save_wisdom:
    value: 7
  charisma:
    value: 18
  save_charisma:
    value: 9
  armor_class:
    value: 18
  max_hp:
    value: 228
  initiative:
    value: 7
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/ice_devil
  implementation: dnd2024:creature_ice_devil
  parameters:
    skill_totals:
      insight: 7
      perception: 7
      persuasion: 9
    damage_immunities:
    - cold
    - fire
    - poison
    condition_immunities:
    - poisoned
    senses:
    - sense: blindsight
      range_ft: 120
    passive_perception: 17
    languages:
      speak: true
      known:
      - infernal
    telepathy:
      range_ft: 120
      two_way: true
    features:
      diabolical_restoration:
        economy: passive
        trigger: actor.died
        when:
          plane_not: nine_hells
        body: disappears
        revive:
          new_body: true
          plane: nine_hells
          location_choice: gm
          hp: maximum
          delay:
            amount: 0
            unit: second
      magic_resistance:
        economy: passive
        op: roll.advantage
        rolls:
        - saving_throw
        against:
        - spell
        - magical_effect
      ice_spear:
        economy: action
        op: attack
        modes:
        - melee
        - ranged
        attack_bonus: 10
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        range_ft:
          normal: 30
          long: 120
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 2
                sides: 8
              add: 5
            average: 14
          - type: cold
            amount:
              dice:
                count: 3
                sides: 6
              add: 0
            average: 10
        - op: effect.apply
          duration:
            until: target.next_turn_end
          rules:
            bonus_actions_allowed: false
            reactions_allowed: false
            speed_add_ft: -10
            turn_economy_choice:
            - movement
            - one_action
            both_allowed: false
        after_attack:
          when: ranged_attack_hit_or_miss
          op: item.return
          destination: actor_hand
          magical: true
      tail:
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
          - type: bludgeoning
            amount:
              dice:
                count: 3
                sides: 6
              add: 5
            average: 15
          - type: cold
            amount:
              dice:
                count: 4
                sides: 8
              add: 0
            average: 18
      ice_wall:
        economy: action
        op: spellcasting
        profile:
          ability: intelligence
          waived_components:
          - verbal
          - somatic
          - material
          save_dc: 17
        choose_count: 1
        choices:
        - op: spell.cast
          spell:
            key: dnd2024:spells/wall_of_ice
          cast_level: 8
        recharge:
          die_sides: 6
          success_min: 6
          success_max: 6
          check_event: actor.turn_start
          maximum_uses: 1
          initial_uses: 1
          also_recovers_after:
          - short_rest
          - long_rest
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 3
          feature: ice_spear
        replace_attack:
          maximum: 1
          choices:
          - op: feature.use
            feature: tail
    damage_resolution:
      choice:
      - printed_average
      - roll_expression
      chooser: controller
      exclusive: true
    spellcasting_defaults:
      cast_level: lowest_possible_unless_overridden
      upcast_allowed: only_explicit_cast_level_overrides
      required_material_components_available: true
      casting_time_at_least_1_minute:
        uses_normal_time: true
        magic_action_each_turn: true
        concentration_required: true
      other_components: spell_definition_unless_waived
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
- id: dnd2024:creature_ice_devil
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
  - actor.died
  - actor.turn_start
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - item_state
  - resources
  - rest_recovery
  - spatial_constraints
  - spellcasting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 654cc2431963f7857052679847ccc83bb2ee75a9da6cf115c20ff91e7462f760
  source_path: builtin/srd-5.2.1/creature/ice_devil.md
  pages:
  - 300
  engine_gaps: &id001
  - dnd2024:creature_ice_devil
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 939843d5d5c60bec7890e1b807598fa064466991e8ad87de4bb744a91e0709e2
splat_clauses:
- key: source_rules
  text: '# Ice Devil Large Fiend (Devil), Lawful Evil AC 18 Initiative +7 (17) HP 228 (24d10 + 96) Speed
    40 ft. MOD SAVE MOD SAVE MOD SAVE Str 21 +5 +5 Dex 14 +2 +7 Con 18 +4 +9 Int 18 +4 +4 Wis 15 +2 +7
    Cha 18 +4 +9 Skills Insight +7, Perception +7, Persuasion +9 Immunities Cold, Fire, Poison; Poisoned
    Senses Blindsight 120 ft.; Passive Perception 17 Languages Infernal; telepathy 120 ft. CR 14 (XP 11,500;
    PB +5) ### Traits Diabolical Restoration. If the devil dies outside the Nine Hells, its body disappears
    in sulfurous smoke, and it gains a new body instantly, reviving with all its Hit Points somewhere
    in the Nine Hells. Magic Resistance. The devil has Advantage on saving throws against spells and other
    magical effects. ### Actions Multiattack. The devil makes three Ice Spear attacks. It can replace
    one attack with a Tail attack. Ice Spear. Melee or Ranged Attack Roll: +10, reach 5 ft. or range 30/120
    ft. Hit: 14 (2d8 + 5) Piercing damage plus 10 (3d6) Cold damage. Until the end of its next turn, the
    target can’t take a Bonus Action or Reaction, its Speed decreases by 10 feet, and it can move or take
    one action on its turn, not both. Hit or Miss: The spear magically returns to the devil’s hand immediately
    after a ranged attack. Tail. Melee Attack Roll: +10, reach 10 ft. Hit: 15 (3d6 + 5) Bludgeoning damage
    plus 18 (4d8) Cold damage. Ice Wall (Recharge 6). The devil casts Wall of Ice (level 8 version), requiring
    no spell components and using Intelligence as the spellcasting ability (spell save DC 17).'
  sha256: 0697d0aee0cc1ba4ea0a3f83170dbe31b5872d400fb54dda5ae95cbc56410d82
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ice Devil

<!-- SRD page 300 -->

Large Fiend (Devil), Lawful Evil\
AC 18 Initiative +7 (17)\
HP 228 (24d10 + 96)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 21 +5 +5 Dex 14 +2 +7 Con 18 +4 +9\
Int 18 +4 +4 Wis 15 +2 +7 Cha 18 +4 +9\
Skills Insight +7, Perception +7, Persuasion +9\
Immunities Cold, Fire, Poison; Poisoned\
Senses Blindsight 120 ft.; Passive Perception 17\
Languages Infernal; telepathy 120 ft.\
CR 14 (XP 11,500; PB +5)\

### Traits

Diabolical Restoration. If the devil dies outside the\
Nine Hells, its body disappears in sulfurous smoke, and\
it gains a new body instantly, reviving with all its Hit\
Points somewhere in the Nine Hells.\
Magic Resistance. The devil has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Multiattack. The devil makes three Ice Spear attacks. It\
can replace one attack with a Tail attack.\
Ice Spear. Melee or Ranged Attack Roll: +10, reach 5\
ft. or range 30/120 ft. Hit: 14 (2d8 + 5) Piercing damage\
plus 10 (3d6) Cold damage. Until the end of its next\
turn, the target can’t take a Bonus Action or Reaction,\
its Speed decreases by 10 feet, and it can move or take\
one action on its turn, not both. Hit or Miss: The spear\
magically returns to the devil’s hand immediately after a\
ranged attack.\
Tail. Melee Attack Roll: +10, reach 10 ft. Hit: 15 (3d6\
+ 5) Bludgeoning damage plus 18 (4d8) Cold damage.\
Ice Wall (Recharge 6). The devil casts Wall of Ice (level\
8 version), requiring no spell components and using In-\
telligence as the spellcasting ability (spell save DC 17).\

Original layout: [[../../srd-5.2.1/pages/300|page 300]].
