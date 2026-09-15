---
splat_schema: 1
splat_id: b9dcdb24-96a6-5bce-841a-eb4a862eb34c
splat_kind: creature
splat_key: dnd2024:creatures/mummy_lord
splat_name: Mummy Lord
splat_origin:
  key: dnd2024:reference/creature/mummy_lord
splat_sources:
- title: SRD 5.2.1
  locator: pp. 309, 310
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: undead
  alignment: Lawful Evil
  size_options_text: Medium or Small
  tags_text: (Cleric)
  movement_text: 30 ft.
  skills_text: History +5, Perception +9, Religion +5
  vulnerabilities_text: Fire
  immunities_text: Necrotic, Poison; Charmed, Exhaustion, Frightened, Paralyzed, Poisoned
  senses_text: Truesight 60 ft.; Passive Perception 19
  languages_text: Common plus three other languages
  cr_text: 15 (XP 13,000, or 15,000 in lair; PB +5)
  hit_points_text: 187 (25d8 + 75)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 11
  save_intelligence:
    value: 5
  wisdom:
    value: 19
  save_wisdom:
    value: 9
  charisma:
    value: 16
  save_charisma:
    value: 3
  armor_class:
    value: 17
  max_hp:
    value: 187
  initiative:
    value: 10
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/mummy_lord
  implementation: dnd2024:creature_mummy_lord
  parameters:
    features:
      legendary_resistance:
        economy: trait
        resource:
          kind: daily
          uses: 3
          recovery: long_rest
          lair_uses: 4
        operation: replace_save_result
        trigger: self.save_failed
        optional: true
        result: success
      magic_resistance:
        economy: trait
        operation: passive_rule
        saving_throw_advantage_against:
        - spells
        - magical_effects
      undead_restoration:
        economy: trait
        operation: passive_rule
        trigger: self.destroyed
        requires:
          heart_intact: true
        new_body:
          delay_seconds: 86400
          hp: maximum
          location:
            in: self.lair
            unoccupied: true
        heart_object:
          size: tiny
          ac: 17
          hp: 10
          damage_immunities:
            all_except:
            - fire
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_choice
          count: 1
          options:
          - rotting_fist
          - channel_negative_energy
        - operation: use_feature
          feature: dreadful_glare
      rotting_fist:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 9
          reach_ft: 5
        hit:
          damage:
          - average: 15
            roll:
              count: 2
              sides: 10
              modifier: 4
            type: bludgeoning
          - average: 10
            roll:
              count: 3
              sides: 6
              modifier: 0
            type: necrotic
          curse:
            target_kind: creature
            can_regain_hp: false
            long_rest_benefits: false
            maximum_hp_reduction:
              interval_seconds: 86400
              roll:
                count: 3
                sides: 6
                modifier: 0
          on_reduced_to_zero_hp:
            die: true
            turn_to_dust: true
      channel_negative_energy:
        economy: action
        operation: attack
        attack:
          mode: ranged
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 9
          range_ft: 60
          long_range_ft: null
        hit:
          damage:
          - average: 25
            roll:
              count: 6
              sides: 6
              modifier: 4
            type: necrotic
      dreadful_glare:
        economy: action
        operation: saving_throw
        ability: wisdom
        dc: 17
        targets:
          kind: creature
          count: 1
          range_ft: 60
          visible: true
        failure:
          damage:
          - average: 25
            roll:
              count: 6
              sides: 6
              modifier: 4
            type: psychic
          conditions:
          - condition: paralyzed
            until: self.next_turn_end
        success:
          effects: []
      spellcasting:
        economy: action
        operation: spellcasting
        ability: wisdom
        components_waived:
        - material
        save_dc: 17
        supplied_spell_attack_total: 9
        options:
        - spell:
            key: dnd2024:spells/dispel_magic
          usage:
            kind: at_will
        - spell:
            key: dnd2024:spells/thaumaturgy
          usage:
            kind: at_will
        - spell:
            key: dnd2024:spells/animate_dead
          usage:
            kind: daily
            uses: 1
            pool: per_spell
            recovery: long_rest
        - spell:
            key: dnd2024:spells/harm
          usage:
            kind: daily
            uses: 1
            pool: per_spell
            recovery: long_rest
        - spell:
            key: dnd2024:spells/insect_plague
          usage:
            kind: daily
            uses: 1
            pool: per_spell
            recovery: long_rest
          cast_level: 7
        material_components_available: true
      whirlwind_of_sand:
        economy: reaction
        operation: sequence
        trigger: self.hit_by_attack_roll
        effects:
        - operation: modify_ac
          bonus: 2
          scope: triggering_attack
          reevaluate_hit: true
        - operation: teleport
          maximum_ft: 60
          unoccupied: true
          visible: true
        - operation: apply_conditions
          targets:
            kind: creature
            selection: self_choice
            visible: true
            within_destination_ft: 5
          conditions:
          - condition: blinded
            until: self.next_turn_end
      dread_command:
        economy: legendary_action
        legendary_cost: 1
        cooldown_until: self.next_turn_start
        operation: cast_spell
        spell:
          key: dnd2024:spells/command
        cast_level: 2
        ability: wisdom
        save_dc: 17
      glare:
        economy: legendary_action
        legendary_cost: 1
        cooldown_until: self.next_turn_start
        operation: use_feature
        feature: dreadful_glare
      necrotic_strike:
        economy: legendary_action
        legendary_cost: 1
        operation: attack_choice
        count: 1
        options:
        - rotting_fist
        - channel_negative_energy
    skill_totals:
      history: 5
      perception: 9
      religion: 5
    damage_vulnerabilities:
    - fire
    damage_immunities:
    - necrotic
    - poison
    condition_immunities:
    - charmed
    - exhaustion
    - frightened
    - paralyzed
    - poisoned
    senses:
      truesight: 60
      passive_perception: 19
    languages:
      speaks:
      - common
      understands: []
      additional_languages:
        count: 3
        chooser: creator
        can_speak: true
    legendary_actions:
      uses: 3
      recovery: self.turn_start
      trigger: other_creature.turn_end
      cost_per_action: 1
      lair_uses: 4
      requires:
        not_incapacitated: true
      maximum_actions_per_trigger: 1
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
- id: dnd2024:creature_mummy_lord
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
  - turn.ended
  - creature.destroyed
  - rest.completed
  - saving_throw.resolved
  - attack.resolved
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - spellcasting
  - teleportation
  - spatial_queries
  - limited_resources
  - curses
  - legendary_actions
  - hit_points
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: e859f75c881c7cf87c0a6002d3d1de4a579ec047fa1c2bab92dde2c8d22c2c2d
  source_path: builtin/srd-5.2.1/creature/mummy_lord.md
  pages:
  - 309
  - 310
  engine_gaps: &id001
  - dnd2024:creature_mummy_lord
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 3c0b7f236cd79d5faa606acf9e0326ede393fc1482228d801d428d2680113995
splat_clauses:
- key: source_rules
  text: '# Mummy Lord Medium or Small Undead (Cleric), Lawful Evil AC 17 Initiative +10 (20) HP 187 (25d8
    + 75) Speed 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 10 +0 +0 Con 17 +3 +3 Int 11 +0 +5
    Wis 19 +4 +9 Cha 16 +3 +3 Skills History +5, Perception +9, Religion +5 Vulnerabilities Fire Immunities
    Necrotic, Poison; Charmed, Exhaustion, Frightened, Paralyzed, Poisoned Senses Truesight 60 ft.; Passive
    Perception 19 Languages Common plus three other languages CR 15 (XP 13,000, or 15,000 in lair; PB
    +5) ### Traits Legendary Resistance (3/Day, or 4/Day in Lair). If the mummy fails a saving throw,
    it can choose to succeed instead. Magic Resistance. The mummy has Advantage on saving throws against
    spells and other magical effects. Undead Restoration. If destroyed, the mummy gains a new body in
    24 hours if its heart is intact, reviving with all its Hit Points. The new body appears in an unoccupied
    space within the mummy’s lair. The heart is a Tiny object that has AC 17, HP 10, and Immunity to all
    damage except Fire. ### Actions Multiattack. The mummy makes one Rotting Fist or Channel Negative
    Energy attack, and it uses Dreadful Glare. Rotting Fist. Melee Attack Roll: +9, reach 5 ft. Hit: 15
    (2d10 + 4) Bludgeoning damage plus 10 (3d6) Necrotic damage. If the target is a creature, it is cursed.
    While cursed, the target can’t regain Hit Points, it gains no benefit from finishing a Long Rest,
    and its Hit Point maximum decreases by 10 (3d6) every 24 hours that elapse. A creature dies and turns
    to dust if reduced to 0 Hit Points by this attack. Channel Negative Energy. Ranged Attack Roll: +9,
    range 60 ft. Hit: 25 (6d6 + 4) Necrotic damage. Dreadful Glare. Wisdom Saving Throw: DC 17, one creature
    the mummy can see within 60 feet. Failure: 25 (6d6 + 4) Psychic damage, and the target has the Paralyzed
    condition until the end of the mummy’s next turn. Spellcasting. The mummy casts one of the following
    spells, requiring no Material components and using Wisdom as the spellcasting ability (spell save
    DC 17, +9 to hit with spell attacks): At Will: Dispel Magic, Thaumaturgy 1/Day Each: Animate Dead,
    Harm, Insect Plague (level 7 version) ### Reactions Whirlwind of Sand. Trigger: The mummy is hit by
    an attack roll. Response: The mummy adds 2 to its AC against the attack, possibly causing the attack
    to miss, and the mummy teleports up to 60 feet to an unoccupied space it can see. Each creature of
    its choice that it can see within 5 feet of its destination space has the Blinded condition until
    the end of the mummy’s next turn. ### Legendary Actions Legendary Action Uses: 3 (4 in Lair). Immediately
    after another creature’s turn, the mummy can expend a use to take one of the following actions. The
    mummy regains all expended uses at the start of each of its turns. Dread Command. The mummy casts
    Command (level 2 version), using the same spellcasting ability as Spellcasting. The mummy can’t take
    this action again until the start of its next turn. Glare. The mummy uses Dreadful Glare. The mummy
    can’t take this action again until the start of its next turn. Necrotic Strike. The mummy makes one
    Rotting Fist or Channel Negative Energy attack.'
  sha256: 722401de0e3a249c74ea5a5a7bdb1bd94657c730d615890054e87fc7dcf99514
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mummy Lord

<!-- SRD page 309 -->

Medium or Small Undead (Cleric), Lawful Evil\
AC 17 Initiative +10 (20)\
HP 187 (25d8 + 75)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 10 +0 +0 Con 17 +3 +3\
Int 11 +0 +5 Wis 19 +4 +9 Cha 16 +3 +3\
Skills History +5, Perception +9, Religion +5\
Vulnerabilities Fire\
Immunities Necrotic, Poison; Charmed, Exhaustion,\
Frightened, Paralyzed, Poisoned\
Senses Truesight 60 ft.; Passive Perception 19\
Languages Common plus three other languages\
CR 15 (XP 13,000, or 15,000 in lair; PB +5)\

<!-- SRD page 310 -->


### Traits

Legendary Resistance (3/Day, or 4/Day in Lair). If\
the mummy fails a saving throw, it can choose to suc-\
ceed instead.\
Magic Resistance. The mummy has Advantage on sav-\
ing throws against spells and other magical effects.\
Undead Restoration. If destroyed, the mummy gains a\
new body in 24 hours if its heart is intact, reviving with\
all its Hit Points. The new body appears in an unoccu-\
pied space within the mummy’s lair. The heart is a Tiny\
object that has AC 17, HP 10, and Immunity to all dam-\
age except Fire.\

### Actions

Multiattack. The mummy makes one Rotting Fist\
or Channel Negative Energy attack, and it uses\
Dreadful Glare.\
Rotting Fist. Melee Attack Roll: +9, reach 5 ft. Hit: 15\
(2d10 + 4) Bludgeoning damage plus 10 (3d6) Necrotic\
damage. If the target is a creature, it is cursed. While\
cursed, the target can’t regain Hit Points, it gains no\
benefit from finishing a Long Rest, and its Hit Point\
maximum decreases by 10 (3d6) every 24 hours that\
elapse. A creature dies and turns to dust if reduced to 0\
Hit Points by this attack.\
Channel Negative Energy. Ranged Attack Roll: +9,\
range 60 ft. Hit: 25 (6d6 + 4) Necrotic damage.\
Dreadful Glare. Wisdom Saving Throw: DC 17, one\
creature the mummy can see within 60 feet. Failure: 25\
(6d6 + 4) Psychic damage, and the target has the Para-\
lyzed condition until the end of the mummy’s next turn.\
Spellcasting. The mummy casts one of the following\
spells, requiring no Material components and using\
Wisdom as the spellcasting ability (spell save DC 17, +9\
to hit with spell attacks):\
At Will: Dispel Magic, Thaumaturgy\
1/Day Each: Animate Dead, Harm, Insect Plague (level\
7 version)\

### Reactions

Whirlwind of Sand. Trigger: The mummy is hit by an\
attack roll. Response: The mummy adds 2 to its AC\
against the attack, possibly causing the attack to miss,\
and the mummy teleports up to 60 feet to an unoccu-\
pied space it can see. Each creature of its choice that\
it can see within 5 feet of its destination space has\
the Blinded condition until the end of the mummy’s\
next turn.\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the mummy can expend a use to\
take one of the following actions. The mummy regains all\
expended uses at the start of each of its turns.\
Dread Command. The mummy casts Command (level\
2 version), using the same spellcasting ability as Spell-\
casting. The mummy can’t take this action again until\
the start of its next turn.\
Glare. The mummy uses Dreadful Glare. The mummy\
can’t take this action again until the start of its\
next turn.\
Necrotic Strike. The mummy makes one Rotting Fist or\
Channel Negative Energy attack.\

Original layout: [[../../srd-5.2.1/pages/309|page 309]], [[../../srd-5.2.1/pages/310|page 310]].
