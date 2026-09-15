---
splat_schema: 1
splat_id: fdee7df5-944b-5b70-80bc-e693430ff2fd
splat_kind: creature
splat_key: dnd2024:creatures/rakshasa
splat_name: Rakshasa
splat_origin:
  key: dnd2024:reference/creature/rakshasa
splat_sources:
- title: SRD 5.2.1
  locator: pp. 317, 318
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: fiend
  alignment: Lawful Evil
  movement_text: 40 ft.
  skills_text: Deception +10, Insight +8, Perception +8
  vulnerabilities_text: Piercing damage from weapons wielded by creatures under the effect of a Bless
    spell
  immunities_text: Charmed, Frightened
  senses_text: Truesight 60 ft.; Passive Perception 18
  languages_text: Common, Infernal
  cr_text: 13 (XP 10,000; PB +5)
  hit_points_text: 221 (26d8 + 104)
splat_stats:
  strength:
    value: 14
  save_strength:
    value: 2
  dexterity:
    value: 17
  save_dexterity:
    value: 3
  constitution:
    value: 18
  save_constitution:
    value: 4
  intelligence:
    value: 13
  save_intelligence:
    value: 1
  wisdom:
    value: 16
  save_wisdom:
    value: 3
  charisma:
    value: 20
  save_charisma:
    value: 5
  armor_class:
    value: 17
  max_hp:
    value: 221
  initiative:
    value: 8
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/rakshasa
  implementation: dnd2024:creature_rakshasa
  parameters:
    features:
      cursed_touch:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 10
        targets:
          count: 1
        hit:
        - op: damage
          average: 12
          amount:
            dice:
              count: 2
              sides: 6
            add: 5
          damage_type: slashing
        - op: damage
          average: 19
          amount:
            dice:
              count: 3
              sides: 12
            add: 0
          damage_type: necrotic
        - op: if
          test:
            target_kind: creature
          then:
          - op: curse
            effects:
            - op: disable_rest_benefits
              rests:
              - short
              - long
        reach_feet: 5
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: cursed_touch
          count: 3
      spellcasting:
        economy: action
        op: cast_spell_choice
        ability: charisma
        spells:
        - spell:
            key: dnd2024:spells/detect_magic
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/detect_thoughts
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/disguise_self
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/mage_hand
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/minor_illusion
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/fly
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/invisibility
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/major_image
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/plane_shift
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        components_waived:
        - material
        save_dc: 18
      fiendish_restoration:
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
      greater_magic_resistance:
        economy: passive
        auto_success:
          roll: saving_throw
          against:
          - spell
          - magical_effect
        auto_miss:
          incoming_roll: spell_attack
        permission_required_for:
          spell_detection:
          - remote_observation
          - thoughts
          - creature_type
          - alignment
      baleful_command:
        economy: action
        resource:
          capacity: 1
          initial: 1
          recharge:
            event: turn.start
            participant: self
            roll:
              dice:
                count: 1
                sides: 6
              add: 0
            succeeds_on:
            - 5
            - 6
          cost: 1
          recovery_also:
          - short_rest
          - long_rest
        op: save
        ability: wisdom
        dc: 18
        targets:
          kind: creature
          count: all
          area:
            shape: emanation
            size_feet: 30
            origin: self
          relation: enemy
        failure:
        - op: damage
          average: 28
          amount:
            dice:
              count: 8
              sides: 6
            add: 0
          damage_type: psychic
        - op: condition
          condition: frightened
          ends:
            event: turn.start
            participant: self
            occurrence: next
        - op: condition
          condition: incapacitated
          ends:
            event: turn.start
            participant: self
            occurrence: next
        success: []
    skill_totals:
      deception: 10
      insight: 8
      perception: 8
    defenses:
      vulnerabilities:
        conditional:
        - damage_type: piercing
          source_kind: weapon
          weapon_wielder_has_spell_effect:
            key: dnd2024:spells/bless
      immunities:
        conditions:
        - charmed
        - frightened
    senses:
      passive_perception: 18
      special:
      - sense: truesight
        range_feet: 60
    communication:
      speaks:
      - common
      - infernal
      understands:
      - common
      - infernal
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
- id: dnd2024:creature_rakshasa
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
  - curses
  - dice
  - effects
  - resources
  - spellcasting
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: cb051153da7fcf1184bdd43b73443ab3a2b2388239bd43d36803526705e9e54f
  source_path: builtin/srd-5.2.1/creature/rakshasa.md
  pages:
  - 317
  - 318
  engine_gaps: &id001
  - dnd2024:creature_rakshasa
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 24789dc2b02fc178d57c3035eb3948c36df1622e631080b7a11ebeb6d756bd05
splat_clauses:
- key: source_rules
  text: '# Rakshasa Medium Fiend, Lawful Evil AC 17 Initiative +8 (18) HP 221 (26d8 + 104) Speed 40 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 14 +2 +2 Dex 17 +3 +3 Con 18 +4 +4 Int 13 +1 +1 Wis 16 +3 +3 Cha 20
    +5 +5 Skills Deception +10, Insight +8, Perception +8 Vulnerabilities Piercing damage from weapons
    wielded by creatures under the effect of a Bless spell Immunities Charmed, Frightened Senses Truesight
    60 ft.; Passive Perception 18 Languages Common, Infernal CR 13 (XP 10,000; PB +5) ### Traits Greater
    Magic Resistance. The rakshasa automatically succeeds on saving throws against spells and other magical
    effects, and the attack rolls of spells automatically miss it. Without the rakshasa’s permission,
    no spell can observe the rakshasa remotely or detect its thoughts, creature type, or alignment. Fiendish
    Restoration. If the rakshasa dies outside the Nine Hells, its body turns to ichor, and it gains a
    new body instantly, reviving with all its Hit Points somewhere in the Nine Hells. ### Actions Multiattack.
    The rakshasa makes three Cursed Touch attacks. Cursed Touch. Melee Attack Roll: +10, reach 5 ft. Hit:
    12 (2d6 + 5) Slashing damage plus 19 (3d12) Necrotic damage. If the target is a creature, it is cursed.
    While cursed, the target gains no benefit from finishing a Short or Long Rest. Baleful Command (Recharge
    5–6). Wisdom Saving Throw: DC 18, each enemy in a 30-foot Emanation originating from the rakshasa.
    Failure: 28 (8d6) Psychic damage, and the target has the Frightened and Incapacitated conditions until
    the start of the rakshasa’s next turn. Spellcasting. The rakshasa casts one of the following spells,
    requiring no Material components and using Charisma as the spellcasting ability (spell save DC 18):
    At Will: Detect Magic, Detect Thoughts, Disguise Self, Mage Hand, Minor Illusion 1/Day Each: Fly,
    Invisibility, Major Image, Plane Shift'
  sha256: 5d0cfc684a657c266bf32130d5566f4aca0d2e54246b5352939a9b9c2450410c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rakshasa

<!-- SRD page 317 -->

Medium Fiend, Lawful Evil\
AC 17 Initiative +8 (18)\
HP 221 (26d8 + 104)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 17 +3 +3 Con 18 +4 +4\
Int 13 +1 +1 Wis 16 +3 +3 Cha 20 +5 +5\
Skills Deception +10, Insight +8, Perception +8\
Vulnerabilities Piercing damage from weapons wielded by\
creatures under the effect of a Bless spell\
Immunities Charmed, Frightened\
Senses Truesight 60 ft.; Passive Perception 18\
Languages Common, Infernal\
CR 13 (XP 10,000; PB +5)\

<!-- SRD page 318 -->


### Traits

Greater Magic Resistance. The rakshasa automatically\
succeeds on saving throws against spells and other\
magical effects, and the attack rolls of spells automat-\
ically miss it. Without the rakshasa’s permission, no\
spell can observe the rakshasa remotely or detect its\
thoughts, creature type, or alignment.\
Fiendish Restoration. If the rakshasa dies outside the\
Nine Hells, its body turns to ichor, and it gains a new\
body instantly, reviving with all its Hit Points some-\
where in the Nine Hells.\

### Actions

Multiattack. The rakshasa makes three Cursed\
Touch attacks.\
Cursed Touch. Melee Attack Roll: +10, reach 5 ft. Hit:\
12 (2d6 + 5) Slashing damage plus 19 (3d12) Necrotic\
damage. If the target is a creature, it is cursed. While\
cursed, the target gains no benefit from finishing a\
Short or Long Rest.\
Baleful Command (Recharge 5–6). Wisdom Saving\
Throw: DC 18, each enemy in a 30-foot Emanation orig-\
inating from the rakshasa. Failure: 28 (8d6) Psychic dam-\
age, and the target has the Frightened and Incapacitated\
conditions until the start of the rakshasa’s next turn.\
Spellcasting. The rakshasa casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 18):\
At Will: Detect Magic, Detect Thoughts, Disguise Self,\
Mage Hand, Minor Illusion\
1/Day Each: Fly, Invisibility, Major Image, Plane Shift\

Original layout: [[../../srd-5.2.1/pages/317|page 317]], [[../../srd-5.2.1/pages/318|page 318]].
