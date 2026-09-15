---
splat_schema: 1
splat_id: 77fa8634-4077-5e9f-970a-36508e19b9f4
splat_kind: creature
splat_key: dnd2024:creatures/glabrezu
splat_name: Glabrezu
splat_origin:
  key: dnd2024:reference/creature/glabrezu
splat_sources:
- title: SRD 5.2.1
  locator: pp. 289
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: fiend
  alignment: Chaotic Evil
  tags_text: (Demon)
  movement_text: 40 ft.
  skills_text: Deception +7, Perception +7
  resistances_text: Cold, Fire, Lightning
  immunities_text: Poison; Poisoned
  senses_text: Truesight 120 ft.; Passive Perception 17
  languages_text: Abyssal; telepathy 120 ft.
  cr_text: 9 (XP 5,000; PB +4)
  hit_points_text: 189 (18d10 + 90)
splat_stats:
  strength:
    value: 20
  save_strength:
    value: 9
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 21
  save_constitution:
    value: 9
  intelligence:
    value: 19
  save_intelligence:
    value: 4
  wisdom:
    value: 17
  save_wisdom:
    value: 7
  charisma:
    value: 16
  save_charisma:
    value: 7
  armor_class:
    value: 17
  max_hp:
    value: 189
  initiative:
    value: 6
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/glabrezu
  implementation: dnd2024:creature_glabrezu
  parameters:
    skill_totals:
      deception: 7
      perception: 7
    damage_resistances:
    - cold
    - fire
    - lightning
    damage_immunities:
    - poison
    condition_immunities:
    - poisoned
    senses:
    - sense: truesight
      range_ft: 120
    passive_perception: 17
    languages:
      speak: true
      known:
      - abyssal
    telepathy:
      range_ft: 120
      two_way: true
    features:
      demonic_restoration:
        economy: passive
        trigger: actor.died
        when:
          plane_not: abyss
        body: disappears
        revive:
          new_body: true
          plane: abyss
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
      pincer:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 9
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: slashing
            amount:
              dice:
                count: 2
                sides: 10
              add: 5
            average: 16
        - op: grapple
          escape_dc: 15
          when:
            target_kind: creature
            target_maximum_size: medium
          limb_slots: 2
          slots_per_target: 1
      spellcasting:
        economy: action
        op: spellcasting
        profile:
          ability: intelligence
          waived_components:
          - material
          spells:
          - spell:
              key: dnd2024:spells/darkness
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/detect_magic
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/dispel_magic
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/confusion
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/fly
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/power_word_stun
            uses:
              count: 1
              recovery: long_rest
          save_dc: 16
      pummel:
        economy: action
        op: saving_throw
        ability: dexterity
        dc: 17
        targets:
          kind: creature
          count: 1
          grappled_by: actor
        failure:
        - op: damage
          components:
          - type: bludgeoning
            amount:
              dice:
                count: 3
                sides: 6
              add: 5
            average: 15
        success:
        - op: damage
          from: failure_damage
          multiplier: 0.5
          round: down
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - feature: pincer
          count: 2
        - choose:
          - pummel
          - spellcasting
          count: 1
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
- id: dnd2024:creature_glabrezu
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
  - creature.feature_requested
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
  source_sha256: a5a39888af01431ac3485a2b6f531ea62378b103990b06f76e3f92ae0a2f6979
  source_path: builtin/srd-5.2.1/creature/glabrezu.md
  pages:
  - 289
  engine_gaps: &id001
  - dnd2024:creature_glabrezu
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 9418ea3e5dbb7c5bbd2ecffa4d4b4eb85979278f8fa8b32b520a6eb6ea313c5e
splat_clauses:
- key: source_rules
  text: '# Glabrezu Large Fiend (Demon), Chaotic Evil AC 17 Initiative +6 (16) HP 189 (18d10 + 90) Speed
    40 ft. MOD SAVE MOD SAVE MOD SAVE Str 20 +5 +9 Dex 15 +2 +2 Con 21 +5 +9 Int 19 +4 +4 Wis 17 +3 +7
    Cha 16 +3 +7 Skills Deception +7, Perception +7 Resistances Cold, Fire, Lightning Immunities Poison;
    Poisoned Senses Truesight 120 ft.; Passive Perception 17 Languages Abyssal; telepathy 120 ft. CR 9
    (XP 5,000; PB +4) ### Traits Demonic Restoration. If the glabrezu dies outside the Abyss, its body
    dissolves into ichor, and it gains a new body instantly, reviving with all its Hit Points somewhere
    in the Abyss. Magic Resistance. The glabrezu has Advantage on saving throws against spells and other
    magical effects. ### Actions Multiattack. The glabrezu makes two Pincer attacks and uses Pummel or
    Spellcasting. Pincer. Melee Attack Roll: +9, reach 10 ft. Hit: 16 (2d10 + 5) Slashing damage. If the
    target is a Medium or smaller creature, it has the Grappled condition (escape DC 15) from one of two
    pincers. Pummel. Dexterity Saving Throw: DC 17, one creature Grappled by the glabrezu. Failure: 15
    (3d6 + 5) Bludgeoning damage. Success: Half damage. Spellcasting. The glabrezu casts one of the following
    spells, requiring no Material components and using Intelligence as the spellcasting ability (spell
    save DC 16): At Will: Darkness, Detect Magic, Dispel Magic 1/Day Each: Confusion, Fly, Power Word
    Stun'
  sha256: c2f326b8079b2670277069ec2feea9d4e4fb2465df72e8f257a2e1a553e72753
  status: contract_specified
splat_requires_mechanics: *id001
---

# Glabrezu

<!-- SRD page 289 -->

Large Fiend (Demon), Chaotic Evil\
AC 17 Initiative +6 (16)\
HP 189 (18d10 + 90)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 20 +5 +9 Dex 15 +2 +2 Con 21 +5 +9\
Int 19 +4 +4 Wis 17 +3 +7 Cha 16 +3 +7\
Skills Deception +7, Perception +7\
Resistances Cold, Fire, Lightning\
Immunities Poison; Poisoned\
Senses Truesight 120 ft.; Passive Perception 17\
Languages Abyssal; telepathy 120 ft.\
CR 9 (XP 5,000; PB +4)\

### Traits

Demonic Restoration. If the glabrezu dies outside the\
Abyss, its body dissolves into ichor, and it gains a new\
body instantly, reviving with all its Hit Points some-\
where in the Abyss.\
Magic Resistance. The glabrezu has Advantage on sav-\
ing throws against spells and other magical effects.\

### Actions

Multiattack. The glabrezu makes two Pincer attacks\
and uses Pummel or Spellcasting.\
Pincer. Melee Attack Roll: +9, reach 10 ft. Hit: 16 (2d10\
+ 5) Slashing damage. If the target is a Medium or\
smaller creature, it has the Grappled condition (escape\
DC 15) from one of two pincers.\
Pummel. Dexterity Saving Throw: DC 17, one creature\
Grappled by the glabrezu. Failure: 15 (3d6 + 5) Blud-\
geoning damage. Success: Half damage.\
Spellcasting. The glabrezu casts one of the following\
spells, requiring no Material components and using In-\
telligence as the spellcasting ability (spell save DC 16):\
At Will: Darkness, Detect Magic, Dispel Magic\
1/Day Each: Confusion, Fly, Power Word Stun\

Original layout: [[../../srd-5.2.1/pages/289|page 289]].
