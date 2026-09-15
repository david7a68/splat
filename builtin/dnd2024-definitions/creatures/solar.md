---
splat_schema: 1
splat_id: 9732fc05-ce97-5ce0-b0fb-baadafc54573
splat_kind: creature
splat_key: dnd2024:creatures/solar
splat_name: Solar
splat_origin:
  key: dnd2024:reference/creature/solar
splat_sources:
- title: SRD 5.2.1
  locator: pp. 326, 327
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: celestial
  alignment: Lawful Good
  tags_text: (Angel)
  movement_text: 50 ft., Fly 150 ft. (hover)
  skills_text: Perception +14
  immunities_text: Poison, Radiant; Charmed, Exhaustion, Frightened, Poisoned
  senses_text: Truesight 120 ft.; Passive Perception 24
  languages_text: All; telepathy 120 ft.
  cr_text: 21 (XP 33,000; PB +7)
  hit_points_text: 297 (22d10 + 176)
splat_stats:
  strength:
    value: 26
  save_strength:
    value: 8
  dexterity:
    value: 22
  save_dexterity:
    value: 6
  constitution:
    value: 26
  save_constitution:
    value: 8
  intelligence:
    value: 25
  save_intelligence:
    value: 7
  wisdom:
    value: 25
  save_wisdom:
    value: 7
  charisma:
    value: 30
  save_charisma:
    value: 10
  armor_class:
    value: 21
  max_hp:
    value: 297
  initiative:
    value: 20
  speed:
    value: 50
    unit: ft
  fly_speed:
    value: 150
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/solar
  implementation: dnd2024:creature_solar
  parameters:
    skill_totals:
      perception: 14
    damage_immunities:
    - poison
    - radiant
    condition_immunities:
    - charmed
    - exhaustion
    - frightened
    - poisoned
    senses:
    - sense: truesight
      range_ft: 120
    passive_perception: 24
    languages:
      speak: true
      known: []
      all: true
    telepathy:
      range_ft: 120
      two_way: true
    features:
      divine_awareness:
        economy: passive
        op: lie.detect
        requires: hear_lie
        automatic: true
      exalted_restoration:
        economy: passive
        trigger: actor.died
        when:
          plane_not: mount_celestia
        body: disappears
        revive:
          new_body: true
          plane: mount_celestia
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
      flying_sword:
        economy: action
        op: attack
        modes:
        - melee
        - ranged
        attack_bonus: 15
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        range_ft:
          normal: 120
          long: 120
        hit:
        - op: damage
          components:
          - type: slashing
            amount:
              dice:
                count: 4
                sides: 6
              add: 8
            average: 22
          - type: radiant
            amount:
              dice:
                count: 8
                sides: 8
              add: 0
            average: 36
        after_attack:
          when: ranged_attack_hit_or_miss
          op: item.return
          destination: actor_hand
          magical: true
          destination_choice:
          - actor_hand
          - hover_within_5_ft
      spellcasting:
        economy: action
        op: spellcasting
        profile:
          ability: charisma
          waived_components:
          - material
          spells:
          - spell:
              key: dnd2024:spells/detect_evil_and_good
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/commune
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/control_weather
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/dispel_evil_and_good
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/resurrection
            uses:
              count: 1
              recovery: long_rest
          save_dc: 25
      slaying_bow:
        economy: action
        op: saving_throw
        ability: dexterity
        dc: 21
        targets:
          kind: creature
          count: 1
          range_ft: 600
          visible_to_actor: true
        failure:
        - op: branch
          if:
            target_hp_at_most: 100
          then:
          - op: death
          else:
          - op: damage
            components:
            - type: piercing
              amount:
                dice:
                  count: 4
                  sides: 8
                add: 6
              average: 24
            - type: radiant
              amount:
                dice:
                  count: 8
                  sides: 8
                add: 0
              average: 36
        success: []
      divine_aid:
        economy: bonus_action
        op: spellcasting
        profile:
          ability: same_as_spellcasting
          waived_components: []
        choose_count: 1
        choices:
        - op: spell.cast
          spell:
            key: dnd2024:spells/cure_wounds
          cast_level: 2
        - op: spell.cast
          spell:
            key: dnd2024:spells/lesser_restoration
        - op: spell.cast
          spell:
            key: dnd2024:spells/remove_curse
        uses:
          count: 3
          recovery: long_rest
      blinding_gaze:
        economy: legendary_action
        op: saving_throw
        ability: constitution
        dc: 25
        targets:
          kind: creature
          count: 1
          range_ft: 120
          visible_to_actor: true
        failure:
        - op: condition.apply
          condition: blinded
          duration:
            amount: 1
            unit: minute
        success: []
        lockout_until: actor.next_turn_start
        legendary_cost: 1
      radiant_teleport:
        economy: legendary_action
        op: sequence
        steps:
        - op: teleport
          maximum_ft: 60
          destination:
            unoccupied: true
            visible_to_actor: true
        - op: saving_throw
          ability: dexterity
          dc: 25
          targets:
            kind: creature
            count: all
            area:
              shape: emanation
              origin: actor
              size_ft: 10
          failure:
          - op: damage
            components:
            - type: radiant
              amount:
                dice:
                  count: 2
                  sides: 10
                add: 0
              average: 11
          success:
          - op: damage
            from: failure_damage
            multiplier: 0.5
            round: down
        legendary_cost: 1
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 2
          feature: flying_sword
        replace_attack:
          maximum: 1
          choices:
          - op: feature.use
            feature: slaying_bow
    legendary_actions:
      uses: 3
      uses_in_lair: 3
      trigger: another_creature.turn_end_immediately_after
      cost_per_action: 1
      recover_all: actor.turn_start
      maximum_per_trigger: 1
      requires_actions_available: true
      excluded_conditions:
      - incapacitated
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
- id: dnd2024:creature_solar
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
  - another_creature.turn_end_immediately_after
  - creature.feature_requested
  - saving_throw.failed
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - item_state
  - resources
  - rest_recovery
  - saving_throws
  - spatial_constraints
  - spellcasting
splat_conversion:
  status: specified
  category: creature
  source_sha256: fe34ee1d9ad835632fa3e8a40f355dad340f06423664e4b3d81f138cc4814160
  source_path: builtin/srd-5.2.1/creature/solar.md
  pages:
  - 326
  - 327
  engine_gaps: &id001
  - dnd2024:creature_solar
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: e5937f831eb9f80c29476a1a7ffa362968f8317bf37e5bd4d93ed0258b0013bf
splat_clauses:
- key: source_rules
  text: '# Solar Large Celestial (Angel), Lawful Good AC 21 Initiative +20 (30) HP 297 (22d10 + 176) Speed
    50 ft., Fly 150 ft. (hover) MOD SAVE MOD SAVE MOD SAVE Str 26 +8 +8 Dex 22 +6 +6 Con 26 +8 +8 Int
    25 +7 +7 Wis 25 +7 +7 Cha 30 +10 +10 Skills Perception +14 Immunities Poison, Radiant; Charmed, Exhaustion,
    Frightened, Poisoned Senses Truesight 120 ft.; Passive Perception 24 Languages All; telepathy 120
    ft. CR 21 (XP 33,000; PB +7) ### Traits Divine Awareness. The solar knows if it hears a lie. Exalted
    Restoration. If the solar dies outside Mount Celestia, its body disappears, and it gains a new body
    instantly, reviving with all its Hit Points somewhere in Mount Celestia. Legendary Resistance (4/Day).
    If the solar fails a saving throw, it can choose to succeed instead. Magic Resistance. The solar has
    Advantage on saving throws against spells and other magical effects. ### Actions Multiattack. The
    solar makes two Flying Sword attacks. It can replace one attack with a use of Slaying Bow. Flying
    Sword. Melee or Ranged Attack Roll: +15, reach 10 ft. or range 120 ft. Hit: 22 (4d6 + 8) Slashing
    damage plus 36 (8d8) Radiant damage. Hit or Miss: The sword magically returns to the solar’s hand
    or hovers within 5 feet of the solar immediately after a ranged attack. Slaying Bow. Dexterity Saving
    Throw: DC 21, one creature the solar can see within 600 feet. Failure: If the creature has 100 Hit
    Points or fewer, it dies. It otherwise takes 24 (4d8 + 6) Piercing damage plus 36 (8d8) Radiant damage.
    Spellcasting. The solar casts one of the following spells, requiring no Material components and using
    Charisma as the spellcasting ability (spell save DC 25): At Will: Detect Evil and Good 1/Day Each:
    Commune, Control Weather, Dispel Evil and Good, Resurrection ### Bonus Actions Divine Aid (3/Day).
    The solar casts Cure Wounds (level 2 version), Lesser Restoration, or Remove Curse, using the same
    spellcasting ability as Spellcasting. ### Legendary Actions Legendary Action Uses: 3. Immediately
    after another creature’s turn, the solar can expend a use to take one of the following actions. The
    solar regains all expended uses at the start of each of its turns. Blinding Gaze. Constitution Saving
    Throw: DC 25, one creature the solar can see within 120 feet. Failure: The target has the Blinded
    condition for 1 minute. Failure or Success: The solar can’t take this action again until the start
    of its next turn. Radiant Teleport. The solar teleports up to 60 feet to an unoccupied space it can
    see. Dexterity Saving Throw: DC 25, each creature in a 10-foot Emanation originating from the solar
    at its destination space. Failure: 11 (2d10) Radiant damage. Success: Half damage.'
  sha256: eede9ec9fea3ee638846ee94e2801607fba3998803be34c843c6dd9811867ec1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Solar

<!-- SRD page 326 -->

Large Celestial (Angel), Lawful Good\
AC 21 Initiative +20 (30)\
HP 297 (22d10 + 176)\
Speed 50 ft., Fly 150 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 26 +8 +8 Dex 22 +6 +6 Con 26 +8 +8\
Int 25 +7 +7 Wis 25 +7 +7 Cha 30 +10 +10\
Skills Perception +14\
Immunities Poison, Radiant; Charmed, Exhaustion,\
Frightened, Poisoned\
Senses Truesight 120 ft.; Passive Perception 24\
Languages All; telepathy 120 ft.\
CR 21 (XP 33,000; PB +7)\

### Traits

Divine Awareness. The solar knows if it hears a lie.\
Exalted Restoration. If the solar dies outside Mount\
Celestia, its body disappears, and it gains a new body\
instantly, reviving with all its Hit Points somewhere in\
Mount Celestia.\
Legendary Resistance (4/Day). If the solar fails a saving\
throw, it can choose to succeed instead.\
Magic Resistance. The solar has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Multiattack. The solar makes two Flying Sword attacks.\
It can replace one attack with a use of Slaying Bow.\
Flying Sword. Melee or Ranged Attack Roll: +15, reach\
10 ft. or range 120 ft. Hit: 22 (4d6 + 8) Slashing damage\
plus 36 (8d8) Radiant damage. Hit or Miss: The sword\
magically returns to the solar’s hand or hovers within 5\
feet of the solar immediately after a ranged attack.\
Slaying Bow. Dexterity Saving Throw: DC 21, one crea-\
ture the solar can see within 600 feet. Failure: If the\
creature has 100 Hit Points or fewer, it dies. It other-\
wise takes 24 (4d8 + 6) Piercing damage plus 36 (8d8)\
Radiant damage.\
Spellcasting. The solar casts one of the following spells,\
requiring no Material components and using Charisma\
as the spellcasting ability (spell save DC 25):\
At Will: Detect Evil and Good\
1/Day Each: Commune, Control Weather, Dispel Evil\
and Good, Resurrection\

### Bonus Actions

Divine Aid (3/Day). The solar casts Cure Wounds (level\
2 version), Lesser Restoration, or Remove Curse, using\
the same spellcasting ability as Spellcasting.\

<!-- SRD page 327 -->


### Legendary Actions

Legendary Action Uses: 3. Immediately after another\
creature’s turn, the solar can expend a use to take one of\
the following actions. The solar regains all expended uses\
at the start of each of its turns.\
Blinding Gaze. Constitution Saving Throw: DC 25, one\
creature the solar can see within 120 feet. Failure: The\
target has the Blinded condition for 1 minute. Failure or\
Success: The solar can’t take this action again until the\
start of its next turn.\
Radiant Teleport. The solar teleports up to 60 feet to an\
unoccupied space it can see. Dexterity Saving Throw:\
DC 25, each creature in a 10-foot Emanation originat-\
ing from the solar at its destination space. Failure: 11\
(2d10) Radiant damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/326|page 326]], [[../../srd-5.2.1/pages/327|page 327]].
