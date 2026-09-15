---
splat_schema: 1
splat_id: 92583734-40ca-5435-8b21-6b5d73e2cfd5
splat_kind: creature
splat_key: dnd2024:creatures/pit_fiend
splat_name: Pit Fiend
splat_origin:
  key: dnd2024:reference/creature/pit_fiend
splat_sources:
- title: SRD 5.2.1
  locator: pp. 314, 315
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: fiend
  alignment: Lawful Evil
  tags_text: (Devil)
  movement_text: 30 ft., Fly 60 ft.
  skills_text: Perception +10, Persuasion +19
  resistances_text: Cold
  immunities_text: Fire, Poison; Poisoned
  senses_text: Truesight 120 ft.; Passive Perception 20
  languages_text: Infernal; telepathy 120 ft.
  cr_text: 20 (XP 25,000; PB +6)
  hit_points_text: 337 (27d10 + 189)
splat_stats:
  strength:
    value: 26
  save_strength:
    value: 8
  dexterity:
    value: 14
  save_dexterity:
    value: 8
  constitution:
    value: 24
  save_constitution:
    value: 7
  intelligence:
    value: 22
  save_intelligence:
    value: 6
  wisdom:
    value: 18
  save_wisdom:
    value: 10
  charisma:
    value: 24
  save_charisma:
    value: 7
  armor_class:
    value: 21
  max_hp:
    value: 337
  initiative:
    value: 14
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/pit_fiend
  implementation: dnd2024:creature_pit_fiend
  parameters:
    skill_totals:
      perception: 10
      persuasion: 19
    damage_resistances:
    - cold
    damage_immunities:
    - fire
    - poison
    condition_immunities:
    - poisoned
    senses:
    - sense: truesight
      range_ft: 120
    passive_perception: 20
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
      legendary_resistance:
        economy: passive
        trigger: saving_throw.failed
        optional: true
        op: saving_throw.set_result
        result: success
        uses:
          count: 4
          recovery: long_rest
      magic_resistance:
        economy: passive
        op: roll.advantage
        rolls:
        - saving_throw
        against:
        - spell
        - magical_effect
      bite:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 14
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
                count: 3
                sides: 6
              add: 8
            average: 18
        - op: saving_throw
          ability: constitution
          dc: 21
          targets:
            kind: target
            requires_creature: true
          failure:
          - op: condition.apply
            condition: poisoned
            duration:
              amount: 1
              unit: minute
            while_active:
            - op: healing.prohibit
            on_target_turn_start:
            - op: damage
              components:
              - type: poison
                amount:
                  dice:
                    count: 6
                    sides: 6
                  add: 0
                average: 21
            repeat_save:
              event: target.turn_end
              ability: constitution
              dc: 21
              success: effect.end
              automatic_success_after:
                amount: 1
                unit: minute
          success: []
      devilish_claw:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 14
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: necrotic
            amount:
              dice:
                count: 4
                sides: 8
              add: 8
            average: 26
      fiery_mace:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 14
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: force
            amount:
              dice:
                count: 4
                sides: 6
              add: 8
            average: 22
          - type: fire
            amount:
              dice:
                count: 6
                sides: 6
              add: 0
            average: 21
      fear_aura:
        economy: passive
        trigger: enemy.turn_start
        requires:
          actor_excluded_conditions:
          - incapacitated
        effect:
          op: saving_throw
          ability: wisdom
          dc: 21
          targets:
            kind: triggering_enemy
            count: all
            area:
              shape: emanation
              origin: actor
              size_ft: 20
          failure:
          - op: condition.apply
            condition: frightened
            duration:
              until: target.next_turn_start
          success:
          - op: feature.immunity
            source: actor
            feature: fear_aura
            duration:
              amount: 24
              unit: hour
      hellfire_spellcasting:
        economy: action
        op: spell_sequence
        ability: charisma
        save_dc: 21
        waived_components:
        - material
        base:
        - op: spell.cast
          spell:
            key: dnd2024:spells/fireball
          cast_level: 5
          count: 2
        replacement:
          maximum: 1
          choices:
          - op: spell.cast
            spell:
              key: dnd2024:spells/hold_monster
            cast_level: 7
          - op: spell.cast
            spell:
              key: dnd2024:spells/wall_of_fire
        recharge:
          die_sides: 6
          success_min: 4
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
        - count: 1
          feature: bite
        - count: 2
          feature: devilish_claw
        - count: 1
          feature: fiery_mace
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
- id: dnd2024:creature_pit_fiend
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
  - enemy.turn_start
  - saving_throw.failed
  - target.turn_end
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - resources
  - rest_recovery
  - saving_throws
  - spatial_constraints
  - spellcasting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 1c46fb765527ba87943cfb3eee088043dc1d2e51d690bca788fe92320cfd0220
  source_path: builtin/srd-5.2.1/creature/pit_fiend.md
  pages:
  - 314
  - 315
  engine_gaps: &id001
  - dnd2024:creature_pit_fiend
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 41df1f01d7c82294d7e10d5eba28e2fdd35bae665e23aebea0123773d226baea
splat_clauses:
- key: source_rules
  text: '# Pit Fiend Large Fiend (Devil), Lawful Evil AC 21 Initiative +14 (24) HP 337 (27d10 + 189) Speed
    30 ft., Fly 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 26 +8 +8 Dex 14 +2 +8 Con 24 +7 +7 Int 22 +6 +6
    Wis 18 +4 +10 Cha 24 +7 +7 Skills Perception +10, Persuasion +19 Resistances Cold Immunities Fire,
    Poison; Poisoned Senses Truesight 120 ft.; Passive Perception 20 Languages Infernal; telepathy 120
    ft. CR 20 (XP 25,000; PB +6) ### Traits Diabolical Restoration. If the pit fiend dies outside the
    Nine Hells, its body disappears in sulfurous smoke, and it gains a new body instantly, reviving with
    all its Hit Points somewhere in the Nine Hells. Fear Aura. The pit fiend emanates an aura in a 20-
    foot Emanation while it doesn’t have the Incapacitated condition. Wisdom Saving Throw: DC 21, any
    enemy that starts its turn in the aura. Failure: The target has the Frightened condition until the
    start of its next turn. Success: The target is immune to this pit fiend’s aura for 24 hours. Legendary
    Resistance (4/Day). If the pit fiend fails a saving throw, it can choose to succeed instead. Magic
    Resistance. The pit fiend has Advantage on saving throws against spells and other magical effects.
    ### Actions Multiattack. The pit fiend makes one Bite attack, two Devilish Claw attacks, and one Fiery
    Mace attack. Bite. Melee Attack Roll: +14, reach 10 ft. Hit: 18 (3d6 + 8) Piercing damage. If the
    target is a creature, it must make the following saving throw. Constitution Saving Throw: DC 21. Failure:
    The target has the Poisoned condition. While Poisoned, the target can’t regain Hit Points and takes
    21 (6d6) Poison damage at the start of each of its turns, and it repeats the save at the end of each
    of its turns, ending the effect on itself on a success. After 1 minute, it succeeds automatically.
    Devilish Claw. Melee Attack Roll: +14, reach 10 ft. Hit: 26 (4d8 + 8) Necrotic damage. Fiery Mace.
    Melee Attack Roll: +14, reach 10 ft. Hit: 22 (4d6 + 8) Force damage plus 21 (6d6) Fire damage. Hellfire
    Spellcasting (Recharge 4–6). The pit fiend casts Fireball (level 5 version) twice, requiring no Material
    components and using Charisma as the spellcasting ability (spell save DC 21). It can replace one Fireball
    with Hold Monster (level 7 version) or Wall of Fire.'
  sha256: 2b269d21f369d52892c9212e37f25a6c1aab08f429ec8be4fda11ed141e1aaf3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pit Fiend

<!-- SRD page 314 -->

Large Fiend (Devil), Lawful Evil\
AC 21 Initiative +14 (24)\
HP 337 (27d10 + 189)\
Speed 30 ft., Fly 60 ft.\

<!-- SRD page 315 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 26 +8 +8 Dex 14 +2 +8 Con 24 +7 +7\
Int 22 +6 +6 Wis 18 +4 +10 Cha 24 +7 +7\
Skills Perception +10, Persuasion +19\
Resistances Cold\
Immunities Fire, Poison; Poisoned\
Senses Truesight 120 ft.; Passive Perception 20\
Languages Infernal; telepathy 120 ft.\
CR 20 (XP 25,000; PB +6)\

### Traits

Diabolical Restoration. If the pit fiend dies outside the\
Nine Hells, its body disappears in sulfurous smoke, and\
it gains a new body instantly, reviving with all its Hit\
Points somewhere in the Nine Hells.\
Fear Aura. The pit fiend emanates an aura in a 20-\
foot Emanation while it doesn’t have the Incapacitated\
condition. Wisdom Saving Throw: DC 21, any enemy\
that starts its turn in the aura. Failure: The target has\
the Frightened condition until the start of its next turn.\
Success: The target is immune to this pit fiend’s aura\
for 24 hours.\
Legendary Resistance (4/Day). If the pit fiend fails a\
saving throw, it can choose to succeed instead.\
Magic Resistance. The pit fiend has Advantage on sav-\
ing throws against spells and other magical effects.\

### Actions

Multiattack. The pit fiend makes one Bite attack, two\
Devilish Claw attacks, and one Fiery Mace attack.\
Bite. Melee Attack Roll: +14, reach 10 ft. Hit: 18 (3d6\
+ 8) Piercing damage. If the target is a creature, it must\
make the following saving throw. Constitution Saving\
Throw: DC 21. Failure: The target has the Poisoned\
condition. While Poisoned, the target can’t regain Hit\
Points and takes 21 (6d6) Poison damage at the start of\
each of its turns, and it repeats the save at the end of\
each of its turns, ending the effect on itself on a suc-\
cess. After 1 minute, it succeeds automatically.\
Devilish Claw. Melee Attack Roll: +14, reach 10 ft. Hit:\
26 (4d8 + 8) Necrotic damage.\
Fiery Mace. Melee Attack Roll: +14, reach 10 ft. Hit: 22\
(4d6 + 8) Force damage plus 21 (6d6) Fire damage.\
Hellfire Spellcasting (Recharge 4–6). The pit fiend\
casts Fireball (level 5 version) twice, requiring no Mate-\
rial components and using Charisma as the spellcasting\
ability (spell save DC 21). It can replace one Fireball\
with Hold Monster (level 7 version) or Wall of Fire.\

Original layout: [[../../srd-5.2.1/pages/314|page 314]], [[../../srd-5.2.1/pages/315|page 315]].
