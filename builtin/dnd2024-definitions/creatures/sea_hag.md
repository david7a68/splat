---
splat_schema: 1
splat_id: eab8c6aa-2a19-5d7b-a4d0-a327cfe7bc90
splat_kind: creature
splat_key: dnd2024:creatures/sea_hag
splat_name: Sea Hag
splat_origin:
  key: dnd2024:reference/creature/sea_hag
splat_sources:
- title: SRD 5.2.1
  locator: pp. 322
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: fey
  alignment: Chaotic Evil
  movement_text: 30 ft., Swim 40 ft.
  senses_text: Darkvision 60 ft.; Passive Perception 11
  languages_text: Common, Giant, Primordial (Aquan)
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 52 (7d8 + 21)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 12
  save_intelligence:
    value: 1
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 13
  save_charisma:
    value: 1
  armor_class:
    value: 14
  max_hp:
    value: 52
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/sea_hag
  implementation: dnd2024:creature_sea_hag
  parameters:
    features:
      amphibious:
        economy: trait
        operation: passive_rule
        breathing:
        - air
        - water
      coven_magic:
        economy: trait
        operation: spellcasting
        requires:
          hag_allies_within_ft: 30
          minimum_allies: 2
        ability: intelligence
        save_dc: 11
        components_waived:
        - material
        casting_time: spell_normal
        options:
        - spell:
            key: dnd2024:spells/augury
          usage:
            kind: rest
            uses: 1
            recovery: long_rest
            pool: per_spell
        - spell:
            key: dnd2024:spells/find_familiar
          usage:
            kind: rest
            uses: 1
            recovery: long_rest
            pool: per_spell
        - spell:
            key: dnd2024:spells/identify
          usage:
            kind: rest
            uses: 1
            recovery: long_rest
            pool: per_spell
        - spell:
            key: dnd2024:spells/locate_object
          usage:
            kind: rest
            uses: 1
            recovery: long_rest
            pool: per_spell
        - spell:
            key: dnd2024:spells/scrying
          usage:
            kind: rest
            uses: 1
            recovery: long_rest
            pool: per_spell
        - spell:
            key: dnd2024:spells/unseen_servant
          usage:
            kind: rest
            uses: 1
            recovery: long_rest
            pool: per_spell
        material_components_available: true
      vile_appearance:
        economy: trait
        operation: saving_throw
        ability: wisdom
        dc: 11
        targets:
          kind: creature
          count: 1
          range_ft: 30
          types:
          - beast
          - humanoid
          can_see_true_form: true
        failure:
          conditions:
          - condition: frightened
            until: target.next_turn_start
        trigger: target.turn_start
        success:
          immunity:
            effect: this_vile_appearance
            duration_seconds: 86400
      claw:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 5
          reach_ft: 5
        hit:
          damage:
          - average: 10
            roll:
              count: 2
              sides: 6
              modifier: 3
            type: slashing
      death_glare:
        economy: action
        resource:
          kind: recharge
          die: 6
          success_faces:
          - 5
          - 6
          check: self.turn_start
          rest_recovery:
          - short
          - long
          capacity: 1
          initial_uses: 1
        operation: saving_throw
        ability: wisdom
        dc: 11
        targets:
          kind: creature
          count: 1
          range_ft: 30
          visible: true
          required_conditions:
          - frightened
        failure:
          if:
            target_current_hp_at_most: 20
          then:
            set_hp: 0
          else:
            damage:
            - roll:
                count: 3
                sides: 8
                modifier: 0
              type: psychic
        success:
          effects: []
      illusory_appearance:
        economy: action
        operation: cast_spell
        spell:
          key: dnd2024:spells/disguise_self
        ability: constitution
        save_dc: 13
        duration_seconds: 86400
    senses:
      darkvision: 60
      passive_perception: 11
    languages:
      speaks:
      - common
      - giant
      - language: primordial
        dialects:
        - aquan
      understands: []
    resolution_conventions:
      distance_unit: foot
      duration_unit: second
      damage_roll_mode:
        choice:
        - listed_average
        - roll_dice
        chooser: gm
      supplied_totals: before_additional_effect_modifiers
      multiattack_nested_economy_cost: 0
      nested_feature_resource_cost: normal
      spell_level_unless_specified: lowest_possible
      spell_upcast_without_permission: false
      spell_casting_time_at_least_one_minute: normal_casting_procedure
      geometry:
        input: adjudication
        assumptions:
        - range_and_visibility_verified
        - area_membership_verified
        - path_and_destination_legal
      unlisted_success_branch: no_effect
      unlisted_duration: until_removed_by_applicable_rule
splat_contracts:
- id: dnd2024:creature_sea_hag
  inputs:
    actor:
      type: participant
      required: true
    feature:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    adjudication:
      required: false
      type: resolution
    trigger:
      required: false
      type: event
  outputs:
  - effects
  events:
  - creature.feature_requested
  - turn.started
  - rest.completed
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - spellcasting
  - spatial_queries
  - limited_resources
  - hit_points
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: cfed41ebbb1715ab4ea4dfa6e6f46eed6b1b0971f733214f6de89e1cc2275925
  source_path: builtin/srd-5.2.1/creature/sea_hag.md
  pages:
  - 322
  engine_gaps: &id001
  - dnd2024:creature_sea_hag
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 8028a7ef89e92881ad40e0dae6688f514a16d7f94d865c1dad3b3ebe605be57a
splat_clauses:
- key: source_rules
  text: '# Sea Hag Medium Fey, Chaotic Evil AC 14 Initiative +1 (11) HP 52 (7d8 + 21) Speed 30 ft., Swim
    40 ft. MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 13 +1 +1 Con 16 +3 +3 Int 12 +1 +1 Wis 12 +1 +1
    Cha 13 +1 +1 Senses Darkvision 60 ft.; Passive Perception 11 Languages Common, Giant, Primordial (Aquan)
    CR 2 (XP 450; PB +2) ### Traits Amphibious. The hag can breathe air and water. Coven Magic. While
    within 30 feet of at least two hag allies, the hag can cast one of the following spells, requiring
    no Material components, using the spell’s normal casting time, and using Intelligence as the spellcasting
    ability (spell save DC 11): Augury, Find Familiar, Identify, Locate Object, Scrying, or Unseen Servant.
    The hag must finish a Long Rest before using this trait to cast that spell again. Vile Appearance.
    Wisdom Saving Throw: DC 11, any Beast or Humanoid that starts its turn within 30 feet of the hag and
    can see the hag’s true form. Failure: The target has the Frightened condition until the start of its
    next turn. Success: The target is immune to this hag’s Vile Appearance for 24 hours. ### Actions Claw.
    Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Slashing damage. Death Glare (Recharge 5–6).
    Wisdom Saving Throw: DC 11, one Frightened creature the hag can see within 30 feet. Failure: If the
    target has 20 Hit Points or fewer, it drops to 0 Hit Points. Otherwise, the target takes 13 (3d8)
    Psychic damage. Illusory Appearance. The hag casts Disguise Self, using Constitution as the spellcasting
    ability (spell save DC 13). The spell’s duration is 24 hours.'
  sha256: f422a8b1a5d2f76f9ba65611ba9b36788cd814ae65a5c95a60fab452387a08e8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sea Hag

<!-- SRD page 322 -->

Medium Fey, Chaotic Evil\
AC 14  Initiative +1 (11)\
HP 52 (7d8 + 21)\
Speed 30 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 13 +1 +1 Con 16 +3 +3\
Int 12 +1 +1 Wis 12 +1 +1 Cha 13 +1 +1\
Senses Darkvision 60 ft.; Passive Perception 11\
Languages Common, Giant, Primordial (Aquan)\
CR 2 (XP 450; PB +2)\

### Traits

Amphibious. The hag can breathe air and water.\
Coven Magic. While within 30 feet of at least two hag\
allies, the hag can cast one of the following spells,\
requiring no Material components, using the spell’s\
normal casting time, and using Intelligence as the spell-\
casting ability (spell save DC 11): Augury, Find Familiar,\
Identify, Locate Object, Scrying, or Unseen Servant.\
The hag must finish a Long Rest before using this trait to\
cast that spell again.\
Vile Appearance. Wisdom Saving Throw: DC 11, any\
Beast or Humanoid that starts its turn within 30 feet of\
the hag and can see the hag’s true form. Failure: The\
target has the Frightened condition until the start of its\
next turn. Success: The target is immune to this hag’s\
Vile Appearance for 24 hours.\

### Actions

Claw. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6\
+ 3) Slashing damage.\
Death Glare (Recharge 5–6). Wisdom Saving Throw:\
DC 11, one Frightened creature the hag can see within\
30 feet. Failure: If the target has 20 Hit Points or fewer,\
it drops to 0 Hit Points. Otherwise, the target takes 13\
(3d8) Psychic damage.\
Illusory Appearance. The hag casts Disguise Self, using\
Constitution as the spellcasting ability (spell save DC\
13). The spell’s duration is 24 hours.\

Original layout: [[../../srd-5.2.1/pages/322|page 322]].
