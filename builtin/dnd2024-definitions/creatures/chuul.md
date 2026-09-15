---
splat_schema: 1
splat_id: a1021e18-4468-569c-9a73-fb1833b1c112
splat_kind: creature
splat_key: dnd2024:creatures/chuul
splat_name: Chuul
splat_origin:
  key: dnd2024:reference/creature/chuul
splat_sources:
- title: SRD 5.2.1
  locator: pp. 273, 274
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: aberration
  alignment: Chaotic Evil
  movement_text: 30 ft., Swim 30 ft.
  skills_text: Perception +4
  immunities_text: Poison; Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 14
  languages_text: Understands Deep Speech but can’t speak
  cr_text: 4 (XP 1,100; PB +2)
  hit_points_text: 76 (9d10 + 27)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 5
  save_intelligence:
    value: -3
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 16
  max_hp:
    value: 76
  initiative:
    value: 0
  speed:
    value: 30
    unit: ft
  swim_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/chuul
  implementation: dnd2024:creature_chuul
  parameters:
    skill_totals:
      perception: 4
    damage_immunities:
    - poison
    condition_immunities:
    - poisoned
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 14
    languages:
      speak: false
      known:
      - deep_speech
    features:
      amphibious:
        economy: passive
        op: breathing.allow
        media:
        - air
        - water
      sense_magic:
        economy: passive
        op: spell.effect
        spell:
          key: dnd2024:spells/detect_magic
        range_override_ft: 120
        magical: false
        continuous: true
      pincer:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 6
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
                count: 1
                sides: 10
              add: 4
            average: 9
        - op: grapple
          escape_dc: 14
          when:
            target_kind: creature
            target_maximum_size: large
          limb_slots: 2
          slots_per_target: 1
      paralyzing_tentacles:
        economy: action
        op: saving_throw
        ability: constitution
        dc: 13
        targets:
          kind: creature
          count: 1
          grappled_by: actor
        failure:
        - op: condition.apply
          condition: poisoned
          duration:
            amount: 1
            unit: minute
          while_active:
          - op: condition.apply
            condition: paralyzed
          repeat_save:
            event: target.turn_end
            ability: constitution
            dc: 13
            success: effect.end
            automatic_success_after:
              amount: 1
              unit: minute
        success: []
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - feature: pincer
          count: 2
        - feature: paralyzing_tentacles
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
- id: dnd2024:creature_chuul
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
  - creature.feature_requested
  - target.turn_end
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - saving_throws
  - spatial_constraints
  - spellcasting
splat_conversion:
  status: specified
  category: creature
  source_sha256: f8ec47de623fe1cd46f91b32986bda8fb7642a765ff7a2d5ce69809da586f199
  source_path: builtin/srd-5.2.1/creature/chuul.md
  pages:
  - 273
  - 274
  engine_gaps: &id001
  - dnd2024:creature_chuul
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: d7e57fd1df84fa9f4c2d23ba36f93ae2c81ee2369767ce46c74588c88640fe55
splat_clauses:
- key: source_rules
  text: '# Chuul Large Aberration, Chaotic Evil AC 16 Initiative +0 (10) HP 76 (9d10 + 27) Speed 30 ft.,
    Swim 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 10 +0 +0 Con 16 +3 +3 Int 5 -3 -3 Wis 11 +0
    +0 Cha 5 -3 -3 Skills Perception +4 Immunities Poison; Poisoned Senses Darkvision 60 ft.; Passive
    Perception 14 Languages Understands Deep Speech but can’t speak CR 4 (XP 1,100; PB +2) ### Traits
    Amphibious. The chuul can breathe air and water. Sense Magic. The chuul senses magic within 120 feet
    of itself. This trait otherwise works like the Detect Magic spell but isn’t itself magical. ### Actions
    Multiattack. The chuul makes two Pincer attacks and uses Paralyzing Tentacles. Pincer. Melee Attack
    Roll: +6, reach 10 ft. Hit: 9 (1d10 + 4) Bludgeoning damage. If the target is a Large or smaller creature,
    it has the Grappled condition (escape DC 14) from one of two pincers. Paralyzing Tentacles. Constitution
    Saving Throw: DC 13, one creature Grappled by the chuul. Failure: The target has the Poisoned condition
    and repeats the save at the end of each of its turns, ending the effect on itself on a success. After
    1 minute, it succeeds automatically. While Poisoned, the target has the Paralyzed condition.'
  sha256: cb2733492ecd1d1c703546acc0b87abec66ac97aa1a2b38f0d399179318486ac
  status: contract_specified
splat_requires_mechanics: *id001
---

# Chuul

<!-- SRD page 273 -->

Large Aberration, Chaotic Evil\
AC 16 Initiative +0 (10)\
HP 76 (9d10 + 27)\
Speed 30 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 10 +0 +0 Con 16 +3 +3\
Int 5 −3 −3 Wis 11 +0 +0 Cha 5 −3 −3\
Skills Perception +4\
Immunities Poison; Poisoned\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages Understands Deep Speech but can’t speak\
CR 4 (XP 1,100; PB +2)\

### Traits

Amphibious. The chuul can breathe air and water.\
Sense Magic. The chuul senses magic within 120 feet of\
itself. This trait otherwise works like the Detect Magic\
spell but isn’t itself magical.\

### Actions

Multiattack. The chuul makes two Pincer attacks and\
uses Paralyzing Tentacles.\
Pincer. Melee Attack Roll: +6, reach 10 ft. Hit: 9 (1d10\
+ 4) Bludgeoning damage. If the target is a Large or\
smaller creature, it has the Grappled condition (escape\
DC 14) from one of two pincers.\

<!-- SRD page 274 -->

Paralyzing Tentacles. Constitution Saving Throw: DC\
13, one creature Grappled by the chuul. Failure: The\
target has the Poisoned condition and repeats the save\
at the end of each of its turns, ending the effect on itself\
on a success. After 1 minute, it succeeds automatically.\
While Poisoned, the target has the Paralyzed condition.\

Original layout: [[../../srd-5.2.1/pages/273|page 273]], [[../../srd-5.2.1/pages/274|page 274]].
