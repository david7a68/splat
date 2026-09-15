---
splat_schema: 1
splat_id: de897d10-b4b2-5e40-92b8-ea286c2d35cd
splat_kind: creature
splat_key: dnd2024:creatures/guardian_naga
splat_name: Guardian Naga
splat_origin:
  key: dnd2024:reference/creature/guardian_naga
splat_sources:
- title: SRD 5.2.1
  locator: pp. 296
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: celestial
  alignment: Lawful Good
  movement_text: 40 ft., Climb 40 ft., Swim 40 ft.
  skills_text: Arcana +11, History +11, Religion +11
  immunities_text: Poison; Charmed, Paralyzed, Poisoned, Restrained
  senses_text: Darkvision 60 ft.; Passive Perception 14
  languages_text: Celestial, Common
  cr_text: 10 (XP 5,900; PB +4)
  hit_points_text: 136 (16d10 + 48)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 18
  save_dexterity:
    value: 8
  constitution:
    value: 16
  save_constitution:
    value: 7
  intelligence:
    value: 16
  save_intelligence:
    value: 7
  wisdom:
    value: 19
  save_wisdom:
    value: 8
  charisma:
    value: 18
  save_charisma:
    value: 8
  armor_class:
    value: 18
  max_hp:
    value: 136
  initiative:
    value: 4
  speed:
    value: 40
    unit: ft
  climb_speed:
    value: 40
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/guardian_naga
  implementation: dnd2024:creature_guardian_naga
  parameters:
    features:
      bite:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 8
        targets:
          count: 1
        hit:
        - op: damage
          average: 17
          amount:
            dice:
              count: 2
              sides: 12
            add: 4
          damage_type: piercing
        - op: damage
          average: 22
          amount:
            dice:
              count: 4
              sides: 10
            add: 0
          damage_type: poison
        reach_feet: 10
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: bite
          count: 2
        replacement:
          max_attacks: all
          choices:
          - feature: poisonous_spittle
      spellcasting:
        economy: action
        op: cast_spell_choice
        ability: wisdom
        spells:
        - spell:
            key: dnd2024:spells/thaumaturgy
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/clairvoyance
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/cure_wounds
          frequency: daily
          uses: 1
          resource_scope: per_spell
          cast_level: 6
          recovery: long_rest
        - spell:
            key: dnd2024:spells/flame_strike
          frequency: daily
          uses: 1
          resource_scope: per_spell
          cast_level: 6
          recovery: long_rest
        - spell:
            key: dnd2024:spells/geas
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/true_seeing
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        components_waived:
        - somatic
        - material
        save_dc: 16
      celestial_restoration:
        economy: passive
        resource:
          capacity: 1
          recovery:
          - long_rest
          cost: 1
        trigger: self.died
        op: schedule_revive
        delay:
          days:
            dice:
              count: 1
              sides: 6
            add: 0
        hit_points: maximum
        canceled_by:
          spell:
            key: dnd2024:spells/dispel_evil_and_good
          target: self_remains
      poisonous_spittle:
        economy: action
        op: save
        ability: constitution
        dc: 16
        targets:
          kind: creature
          count: 1
          range_feet: 60
          visible_to: self
        failure:
        - op: damage
          average: 31
          amount:
            dice:
              count: 7
              sides: 8
            add: 0
          damage_type: poison
        - op: condition
          condition: blinded
          ends:
            event: turn.start
            participant: self
            occurrence: next
        success:
        - op: damage_fraction
          numerator: 1
          denominator: 2
          round: down
    skill_totals:
      arcana: 11
      history: 11
      religion: 11
    defenses:
      immunities:
        damage_types:
        - poison
        conditions:
        - charmed
        - paralyzed
        - poisoned
        - restrained
    senses:
      passive_perception: 14
      special:
      - sense: darkvision
        range_feet: 60
    communication:
      speaks:
      - celestial
      - common
      understands:
      - celestial
      - common
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
- id: dnd2024:creature_guardian_naga
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
  - turn.start
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - resources
  - spellcasting
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 99ea50a447ff88922427b5ebbb8d86a68b7feab18bcc64e900860d7fedb34c3b
  source_path: builtin/srd-5.2.1/creature/guardian_naga.md
  pages:
  - 296
  engine_gaps: &id001
  - dnd2024:creature_guardian_naga
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 49cf71cc4a264f207decba214ff9e6f2ee1a570c067863e6a42244a50a5783e2
splat_clauses:
- key: source_rules
  text: '# Guardian Naga Large Celestial, Lawful Good AC 18 Initiative +4 (14) HP 136 (16d10 + 48) Speed
    40 ft., Climb 40 ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 18 +4 +8 Con 16 +3 +7
    Int 16 +3 +7 Wis 19 +4 +8 Cha 18 +4 +8 Skills Arcana +11, History +11, Religion +11 Immunities Poison;
    Charmed, Paralyzed, Poisoned, Restrained Senses Darkvision 60 ft.; Passive Perception 14 Languages
    Celestial, Common CR 10 (XP 5,900; PB +4) ### Traits Celestial Restoration. If the naga dies, it returns
    to life in 1d6 days and regains all its Hit Points unless Dispel Evil and Good is cast on its remains.
    ### Actions Multiattack. The naga makes two Bite attacks. It can replace any attack with a use of
    Poisonous Spittle. Bite. Melee Attack Roll: +8, reach 10 ft. Hit: 17 (2d12 + 4) Piercing damage plus
    22 (4d10) Poison damage. Poisonous Spittle. Constitution Saving Throw: DC 16, one creature the naga
    can see within 60 feet. Failure: 31 (7d8) Poison damage, and the target has the Blinded condition
    until the start of the naga’s next turn. Success: Half damage only. Spellcasting. The naga casts one
    of the following spells, requiring no Somatic or Material components and using Wisdom as the spellcasting
    ability (spell save DC 16): At Will: Thaumaturgy 1/Day Each: Clairvoyance, Cure Wounds (level 6 version),
    Flame Strike (level 6 version), Geas, True Seeing'
  sha256: bd161763b958ac4e61707053f97141b361a6d616f395301bf8cbfed4c589f473
  status: contract_specified
splat_requires_mechanics: *id001
---

# Guardian Naga

<!-- SRD page 296 -->

Large Celestial, Lawful Good\
AC 18 Initiative +4 (14)\
HP 136 (16d10 + 48)\
Speed 40 ft., Climb 40 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 18 +4 +8 Con 16 +3 +7\
Int 16 +3 +7 Wis 19 +4 +8 Cha 18 +4 +8\
Skills Arcana +11, History +11, Religion +11\
Immunities Poison; Charmed, Paralyzed, Poisoned,\
Restrained\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages Celestial, Common\
CR 10 (XP 5,900; PB +4)\

### Traits

Celestial Restoration. If the naga dies, it returns to life\
in 1d6 days and regains all its Hit Points unless Dispel\
Evil and Good is cast on its remains.\

### Actions

Multiattack. The naga makes two Bite attacks. It can\
replace any attack with a use of Poisonous Spittle.\
Bite. Melee Attack Roll: +8, reach 10 ft. Hit: 17 (2d12\
+ 4) Piercing damage plus 22 (4d10) Poison damage.\
Poisonous Spittle. Constitution Saving Throw: DC 16,\
one creature the naga can see within 60 feet. Failure:\
31 (7d8) Poison damage, and the target has the Blinded\
condition until the start of the naga’s next turn. Success:\
Half damage only.\
Spellcasting. The naga casts one of the following spells,\
requiring no Somatic or Material components and using\
Wisdom as the spellcasting ability (spell save DC 16):\
At Will: Thaumaturgy\
1/Day Each: Clairvoyance, Cure Wounds (level 6 ver-\
sion), Flame Strike (level 6 version), Geas, True Seeing\

Original layout: [[../../srd-5.2.1/pages/296|page 296]].
