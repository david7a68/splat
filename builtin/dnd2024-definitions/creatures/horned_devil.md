---
splat_schema: 1
splat_id: 95f04799-9101-5d44-ae70-280d00dde719
splat_kind: creature
splat_key: dnd2024:creatures/horned_devil
splat_name: Horned Devil
splat_origin:
  key: dnd2024:reference/creature/horned_devil
splat_sources:
- title: SRD 5.2.1
  locator: pp. 299, 300
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: fiend
  alignment: Lawful Evil
  tags_text: (Devil)
  movement_text: 30 ft., Fly 60 ft.
  resistances_text: Cold
  immunities_text: Fire, Poison; Poisoned
  senses_text: Darkvision 150 ft. (unimpeded by magical Darkness); Passive Perception 13
  languages_text: Infernal; telepathy 120 ft.
  cr_text: 11 (XP 7,200; PB +4)
  hit_points_text: 199 (19d10 + 95)
splat_stats:
  strength:
    value: 22
  save_strength:
    value: 10
  dexterity:
    value: 17
  save_dexterity:
    value: 7
  constitution:
    value: 21
  save_constitution:
    value: 5
  intelligence:
    value: 12
  save_intelligence:
    value: 1
  wisdom:
    value: 16
  save_wisdom:
    value: 7
  charisma:
    value: 18
  save_charisma:
    value: 8
  armor_class:
    value: 18
  max_hp:
    value: 199
  initiative:
    value: 7
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/horned_devil
  implementation: dnd2024:creature_horned_devil
  parameters:
    features:
      magic_resistance:
        economy: passive
        advantage:
          roll: saving_throw
          against:
          - spell
          - magical_effect
      searing_fork:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 10
        targets:
          count: 1
        hit:
        - op: damage
          average: 15
          amount:
            dice:
              count: 2
              sides: 8
            add: 6
          damage_type: piercing
        - op: damage
          average: 9
          amount:
            dice:
              count: 2
              sides: 8
            add: 0
          damage_type: fire
        reach_feet: 10
      hurl_flame:
        economy: action
        op: attack
        attack_modes:
        - ranged
        attack_bonus: 8
        targets:
          count: 1
        hit:
        - op: damage
          average: 26
          amount:
            dice:
              count: 5
              sides: 8
            add: 4
          damage_type: fire
        - op: if
          test:
            target_kind: object
            flammable: true
            worn: false
            carried: false
          then:
          - op: ignite
        range_feet: 150
      multiattack:
        economy: action
        op: repeat_choice
        count: 3
        choices:
        - feature: searing_fork
        - feature: hurl_flame
        replacement:
          max_attacks: 1
          choices:
          - feature: infernal_tail
      diabolical_restoration:
        economy: passive
        resource:
          capacity: 1
          recovery:
          - long_rest
          cost: 1
        trigger: self.died
        requires:
          death_plane_not: nine_hells
        op: revive
        new_body: true
        old_body: disappears
        delay:
          seconds: 0
        hit_points: maximum
        destination:
          plane: nine_hells
          location_choice: gm
      infernal_tail:
        economy: action
        op: save
        ability: dexterity
        dc: 17
        targets:
          kind: creature
          count: 1
          range_feet: 10
          visible_to: self
        failure:
        - op: damage
          average: 10
          amount:
            dice:
              count: 1
              sides: 8
            add: 6
          damage_type: necrotic
        - op: if
          test:
            target_has_infernal_wound: false
          then:
          - op: infernal_wound
            stacking: one_per_target
            periodic:
              event: target.turn.start
              effects:
              - op: lose_hit_points
                average: 10
                amount:
                  dice:
                    count: 3
                    sides: 6
                  add: 0
            ends_any:
            - minutes: 1
            - event: spell.restores_hit_points
              target: affected
            - economy: action
              actor: target_or_creature_within_5_feet
              check:
                ability: wisdom
                skill: medicine
                dc: 17
              outcome: success
        success: []
    defenses:
      resistances:
        damage_types:
        - cold
      immunities:
        damage_types:
        - fire
        - poison
        conditions:
        - poisoned
    senses:
      passive_perception: 13
      special:
      - sense: darkvision
        range_feet: 150
        magical_darkness_unimpeded: true
    communication:
      speaks:
      - infernal
      understands:
      - infernal
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
- id: dnd2024:creature_horned_devil
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
  - spell.restores_hit_points
  - target.turn.start
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
  source_sha256: 48f96f08a602761d8959352ef51584a758f8ec1b7e32659e82f912df640d0562
  source_path: builtin/srd-5.2.1/creature/horned_devil.md
  pages:
  - 299
  - 300
  engine_gaps: &id001
  - dnd2024:creature_horned_devil
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 1a964419ca793396445f1a182c32451dfed245319f7d6069d01237b05f23cf6d
splat_clauses:
- key: source_rules
  text: '# Horned Devil Large Fiend (Devil), Lawful Evil AC 18 Initiative +7 (17) HP 199 (19d10 + 95)
    Speed 30 ft., Fly 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 22 +6 +10 Dex 17 +3 +7 Con 21 +5 +5 Int 12
    +1 +1 Wis 16 +3 +7 Cha 18 +4 +8 Resistances Cold Immunities Fire, Poison; Poisoned Senses Darkvision
    150 ft. (unimpeded by magical Darkness); Passive Perception 13 Languages Infernal; telepathy 120 ft.
    CR 11 (XP 7,200; PB +4) ### Traits Diabolical Restoration. If the devil dies outside the Nine Hells,
    its body disappears in sulfurous smoke, and it gains a new body instantly, reviving with all its Hit
    Points somewhere in the Nine Hells. Magic Resistance. The devil has Advantage on saving throws against
    spells and other magical effects. ### Actions Multiattack. The devil makes three attacks, using Searing
    Fork or Hurl Flame in any combination. It can replace one attack with a use of Infernal Tail. Searing
    Fork. Melee Attack Roll: +10, reach 10 ft. Hit: 15 (2d8 + 6) Piercing damage plus 9 (2d8) Fire damage.
    Hurl Flame. Ranged Attack Roll: +8, range 150 ft. Hit: 26 (5d8 + 4) Fire damage. If the target is
    a flammable object that isn’t being worn or carried, it starts burning. Infernal Tail. Dexterity Saving
    Throw: DC 17, one creature the devil can see within 10 feet. Failure: 10 (1d8 + 6) Necrotic damage,
    and the target receives an infernal wound if it doesn’t have one. While wounded, the target loses
    10 (3d6) Hit Points at the start of each of its turns. The wound closes after 1 minute, after a spell
    restores Hit Points to the target, or after the target or a creature within 5 feet of it takes an
    action to stanch the wound, doing so by succeeding on a DC 17 Wisdom (Medicine) check.'
  sha256: 0bd91e3b47655de0edf594adb3bfb7664b373c6b5506bf73949c530421d2b65c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Horned Devil

<!-- SRD page 299 -->

Large Fiend (Devil), Lawful Evil\
AC 18 Initiative +7 (17)\
HP 199 (19d10 + 95)\
Speed 30 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 22 +6 +10 Dex 17 +3 +7 Con 21 +5 +5\
Int 12 +1 +1 Wis 16 +3 +7 Cha 18 +4 +8\
Resistances Cold\
Immunities Fire, Poison; Poisoned\
Senses Darkvision 150 ft. (unimpeded by magical\
Darkness); Passive Perception 13\
Languages Infernal; telepathy 120 ft.\
CR 11 (XP 7,200; PB +4)\

### Traits

Diabolical Restoration. If the devil dies outside the\
Nine Hells, its body disappears in sulfurous smoke, and\
it gains a new body instantly, reviving with all its Hit\
Points somewhere in the Nine Hells.\
Magic Resistance. The devil has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Multiattack. The devil makes three attacks, using Sear-\
ing Fork or Hurl Flame in any combination. It can re-\
place one attack with a use of Infernal Tail.\

<!-- SRD page 300 -->

Searing Fork. Melee Attack Roll: +10, reach 10 ft. Hit:\
15 (2d8 + 6) Piercing damage plus 9 (2d8) Fire damage.\
Hurl Flame. Ranged Attack Roll: +8, range 150 ft. Hit:\
26 (5d8 + 4) Fire damage. If the target is a flammable\
object that isn’t being worn or carried, it starts burning.\
Infernal Tail. Dexterity Saving Throw: DC 17, one crea-\
ture the devil can see within 10 feet. Failure: 10 (1d8\
+ 6) Necrotic damage, and the target receives an infer-\
nal wound if it doesn’t have one. While wounded, the\
target loses 10 (3d6) Hit Points at the start of each of\
its turns. The wound closes after 1 minute, after a spell\
restores Hit Points to the target, or after the target or a\
creature within 5 feet of it takes an action to stanch the\
wound, doing so by succeeding on a DC 17 Wisdom\
(Medicine) check.\

Original layout: [[../../srd-5.2.1/pages/299|page 299]], [[../../srd-5.2.1/pages/300|page 300]].
