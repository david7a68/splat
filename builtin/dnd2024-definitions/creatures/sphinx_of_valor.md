---
splat_schema: 1
splat_id: 9ebd4f75-7afe-5367-b0a2-cfc7a7a5309e
splat_kind: creature
splat_key: dnd2024:creatures/sphinx_of_valor
splat_name: Sphinx of Valor
splat_origin:
  key: dnd2024:reference/creature/sphinx_of_valor
splat_sources:
- title: SRD 5.2.1
  locator: pp. 328
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: celestial
  alignment: Lawful Neutral
  movement_text: 40 ft., Fly 60 ft.
  skills_text: Arcana +9, Perception +12, Religion +15
  resistances_text: Necrotic, Radiant
  immunities_text: Psychic; Charmed, Frightened
  senses_text: Truesight 120 ft.; Passive Perception 22
  languages_text: Celestial, Common
  cr_text: 17 (XP 18,000, or 20,000 in lair; PB +6)
  hit_points_text: 199 (19d10 + 95)
splat_stats:
  strength:
    value: 22
  save_strength:
    value: 6
  dexterity:
    value: 10
  save_dexterity:
    value: 6
  constitution:
    value: 20
  save_constitution:
    value: 11
  intelligence:
    value: 16
  save_intelligence:
    value: 9
  wisdom:
    value: 23
  save_wisdom:
    value: 12
  charisma:
    value: 18
  save_charisma:
    value: 4
  armor_class:
    value: 17
  max_hp:
    value: 199
  initiative:
    value: 12
  speed:
    value: 40
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/sphinx_of_valor
  implementation: dnd2024:creature_sphinx_of_valor
  parameters:
    skill_totals:
      arcana: 9
      perception: 12
      religion: 15
    damage_resistances:
    - necrotic
    - radiant
    damage_immunities:
    - psychic
    condition_immunities:
    - charmed
    - frightened
    senses:
    - sense: truesight
      range_ft: 120
    passive_perception: 22
    languages:
      speak: true
      known:
      - common
      - celestial
    features:
      legendary_resistance:
        economy: passive
        trigger: saving_throw.failed
        optional: true
        op: saving_throw.set_result
        result: success
        uses_in_lair: 4
        uses:
          count: 3
          recovery: long_rest
      claw:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 12
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: slashing
            amount:
              dice:
                count: 4
                sides: 6
              add: 6
            average: 20
      spellcasting:
        economy: action
        op: spellcasting
        profile:
          ability: wisdom
          waived_components:
          - material
          spells:
          - spell:
              key: dnd2024:spells/detect_evil_and_good
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/thaumaturgy
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/detect_magic
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/dispel_magic
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/greater_restoration
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/heroes_feast
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/zone_of_truth
            uses:
              count: 1
              recovery: long_rest
          save_dc: 20
      inscrutable:
        economy: passive
        magic_permissions:
          remote_observation: requires_actor_consent
          thought_detection: requires_actor_consent
        disadvantage_on_checks:
          ability: wisdom
          skill: insight
          purpose:
          - intentions
          - sincerity
      roar:
        economy: action
        op: sequence_by_use
        magical: true
        sequence_reset: long_rest
        effects:
        - op: saving_throw
          ability: wisdom
          dc: 20
          targets:
            kind: creature
            count: all
            area:
              shape: emanation
              origin: actor
              size_ft: 500
            allegiance: enemy
          failure:
          - op: condition.apply
            condition: frightened
            duration:
              amount: 1
              unit: minute
          success: []
        - op: saving_throw
          ability: wisdom
          dc: 20
          targets:
            kind: creature
            count: all
            area:
              shape: emanation
              origin: actor
              size_ft: 500
            allegiance: enemy
          failure:
          - op: condition.apply
            condition: paralyzed
            duration:
              amount: 1
              unit: minute
            repeat_save:
              event: target.turn_end
              ability: wisdom
              dc: 20
              success: effect.end
              automatic_success_after:
                amount: 1
                unit: minute
          success: []
        - op: saving_throw
          ability: constitution
          dc: 20
          targets:
            kind: creature
            count: all
            area:
              shape: emanation
              origin: actor
              size_ft: 500
            allegiance: enemy
          failure:
          - op: damage
            components:
            - type: thunder
              amount:
                dice:
                  count: 8
                  sides: 10
                add: 0
              average: 44
          - op: condition.apply
            condition: prone
          success:
          - op: damage
            from: failure_damage
            multiplier: 0.5
            round: down
        uses:
          count: 3
          recovery: long_rest
      arcane_prowl:
        economy: legendary_action
        op: sequence
        steps:
        - op: teleport
          optional: true
          maximum_ft: 30
          destination:
            unoccupied: true
            visible_to_actor: true
        - op: feature.use
          feature: claw
        legendary_cost: 1
      weight_of_years:
        economy: legendary_action
        op: saving_throw
        ability: constitution
        dc: 16
        targets:
          kind: creature
          count: 1
          range_ft: 120
          visible_to_actor: true
        failure:
        - op: exhaustion.add
          levels: 1
        - op: appearance.age
          years:
            dice:
              count: 3
              sides: 10
            add: 0
          duration:
            until: target.has_no_exhaustion
        success: []
        lockout_until: actor.next_turn_start
        legendary_cost: 1
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - feature: claw
          count: 2
        - feature: roar
          count: 1
    legendary_actions:
      uses: 3
      uses_in_lair: 4
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
- id: dnd2024:creature_sphinx_of_valor
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
  - actor.turn_start
  - another_creature.turn_end_immediately_after
  - creature.feature_requested
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
  source_sha256: 7cc8b5a490a8b17a58deb5d435c7dbcc8452fa85d589d7c66c7d790e832b7c8b
  source_path: builtin/srd-5.2.1/creature/sphinx_of_valor.md
  pages:
  - 328
  engine_gaps: &id001
  - dnd2024:creature_sphinx_of_valor
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 4fd0a6a7138453433b9dd65573f85289ba45dadd2d00aa1a2cf4ce5210a17870
splat_clauses:
- key: source_rules
  text: '# Sphinx of Valor Large Celestial, Lawful Neutral AC 17 Initiative +12 (22) HP 199 (19d10 + 95)
    Speed 40 ft., Fly 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 22 +6 +6 Dex 10 +0 +6 Con 20 +5 +11 Int 16
    +3 +9 Wis 23 +6 +12 Cha 18 +4 +4 Skills Arcana +9, Perception +12, Religion +15 Resistances Necrotic,
    Radiant Immunities Psychic; Charmed, Frightened Senses Truesight 120 ft.; Passive Perception 22 Languages
    Celestial, Common CR 17 (XP 18,000, or 20,000 in lair; PB +6) ### Traits Inscrutable. No magic can
    observe the sphinx remotely or detect its thoughts without its permission. Wisdom (Insight) checks
    made to ascertain its intentions or sincerity are made with Disadvantage. Legendary Resistance (3/Day,
    or 4/Day in Lair). If the sphinx fails a saving throw, it can choose to succeed instead. ### Actions
    Multiattack. The sphinx makes two Claw attacks and uses Roar. Claw. Melee Attack Roll: +12, reach
    5 ft. Hit: 20 (4d6 + 6) Slashing damage. Roar (3/Day). The sphinx emits a magical roar. Whenever it
    roars, the roar has a different effect, as detailed below (the sequence resets when it takes a Long
    Rest): First Roar. Wisdom Saving Throw: DC 20, each enemy in a 500-foot Emanation originating from
    the sphinx. Failure: The target has the Frightened condition for 1 minute. Second Roar. Wisdom Saving
    Throw: DC 20, each enemy in a 500-foot Emanation originating from the sphinx. Failure: The target
    has the Paralyzed condition, and it repeats the save at the end of each of its turns, ending the effect
    on itself on a success. After 1 minute, it succeeds automatically. Third Roar. Constitution Saving
    Throw: DC 20, each enemy in a 500-foot Emanation originating from the sphinx. Failure: 44 (8d10) Thunder
    damage, and the target has the Prone condition. Success: Half damage only. Spellcasting. The sphinx
    casts one of the following spells, requiring no Material components and using Wisdom as the spellcasting
    ability (spell save DC 20): At Will: Detect Evil and Good, Thaumaturgy 1/Day Each: Detect Magic, Dispel
    Magic, Greater Restoration, Heroes’ Feast, Zone of Truth ### Legendary Actions Legendary Action Uses:
    3 (4 in Lair). Immediately after another creature’s turn, the sphinx can expend a use to take one
    of the following actions. The sphinx regains all expended uses at the start of each of its turns.
    Arcane Prowl. The sphinx can teleport up to 30 feet to an unoccupied space it can see, and it makes
    one Claw attack. Weight of Years. Constitution Saving Throw: DC 16, one creature the sphinx can see
    within 120 feet. Failure: The target gains 1 Exhaustion level. While the target has any Exhaustion
    levels, it appears 3d10 years older. Failure or Success: The sphinx can’t take this action again until
    the start of its next turn.'
  sha256: 0cb0a365eee10a0163ed05f2b3ebd28f2d5a85cec39991e8e08767ac972f0ccd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sphinx of Valor

<!-- SRD page 328 -->

Large Celestial, Lawful Neutral\
AC 17 Initiative +12 (22)\
HP 199 (19d10 + 95)\
Speed 40 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 22 +6 +6 Dex 10 +0 +6 Con 20 +5 +11\
Int 16 +3 +9 Wis 23 +6 +12 Cha 18 +4 +4\
Skills Arcana +9, Perception +12, Religion +15\
Resistances Necrotic, Radiant\
Immunities Psychic; Charmed, Frightened\
Senses Truesight 120 ft.; Passive Perception 22\
Languages Celestial, Common\
CR 17 (XP 18,000, or 20,000 in lair; PB +6)\

### Traits

Inscrutable. No magic can observe the sphinx remotely\
or detect its thoughts without its permission. Wisdom\
(Insight) checks made to ascertain its intentions or sin-\
cerity are made with Disadvantage.\
Legendary Resistance (3/Day, or 4/Day in Lair). If\
the sphinx fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The sphinx makes two Claw attacks and\
uses Roar.\
Claw. Melee Attack Roll: +12, reach 5 ft. Hit: 20 (4d6\
+ 6) Slashing damage.\
Roar (3/Day). The sphinx emits a magical roar. When-\
ever it roars, the roar has a different effect, as detailed\
below (the sequence resets when it takes a Long Rest):\
First Roar. Wisdom Saving Throw: DC 20, each enemy\
in a 500-foot Emanation originating from the sphinx.\
Failure: The target has the Frightened condition\
for 1 minute.\
Second Roar. Wisdom Saving Throw: DC 20, each\
enemy in a 500-foot Emanation originating from the\
sphinx. Failure: The target has the Paralyzed condi-\
tion, and it repeats the save at the end of each of its\
turns, ending the effect on itself on a success. After 1\
minute, it succeeds automatically.\
Third Roar. Constitution Saving Throw: DC 20, each\
enemy in a 500-foot Emanation originating from\
the sphinx. Failure: 44 (8d10) Thunder damage, and\
the target has the Prone condition. Success: Half\
damage only.\
Spellcasting. The sphinx casts one of the following\
spells, requiring no Material components and using\
Wisdom as the spellcasting ability (spell save DC 20):\
At Will: Detect Evil and Good, Thaumaturgy\
1/Day Each: Detect Magic, Dispel Magic, Greater Res-\
toration, Heroes’ Feast, Zone of Truth\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the sphinx can expend a use to\
take one of the following actions. The sphinx regains all\
expended uses at the start of each of its turns.\
Arcane Prowl. The sphinx can teleport up to 30 feet\
to an unoccupied space it can see, and it makes one\
Claw attack.\
Weight of Years. Constitution Saving Throw: DC 16,\
one creature the sphinx can see within 120 feet. Failure:\
The target gains 1 Exhaustion level. While the target has\
any Exhaustion levels, it appears 3d10 years older. Fail-\
ure or Success: The sphinx can’t take this action again\
until the start of its next turn.\

Original layout: [[../../srd-5.2.1/pages/328|page 328]].
