---
splat_schema: 1
splat_id: 447f69de-fddc-5113-8938-50e6f1ebe886
splat_kind: creature
splat_key: dnd2024:creatures/draconic_spirit
splat_name: Draconic Spirit
splat_origin:
  key: dnd2024:reference/creature/draconic_spirit
splat_sources:
- title: SRD 5.2.1
  locator: pp. 166
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: dragon
  alignment: Neutral
  movement_text: 30 ft., Fly 60 ft., Swim 30 ft.
  resistances_text: Acid, Cold, Fire, Lightning, Poison
  immunities_text: Charmed, Frightened, Poisoned
  senses_text: Blindsight 30 ft., Darkvision 60 ft.; Passive Perception 12
  languages_text: Draconic, understands the languages you know
  cr_text: None (XP 0; PB equals your Proficiency Bonus)
  hit_points_text: 50 + 10 for each spell level above 5
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 14
  save_charisma:
    value: 2
  armor_class:
    value: 14
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 60
    unit: ft
  swim_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/draconic_spirit
  implementation: dnd2024:creature_draconic_spirit
  parameters:
    features:
      shared_resistances:
        economy: passive
        trigger: summon.created
        choice:
          chooser: summoner
          options:
          - acid
          - cold
          - fire
          - lightning
          - poison
          count: 1
        grant_to: summoner
        resistance: chosen_type
        ends:
          event: summoning_spell.end
      rend:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus:
          input: summoner_spell_attack_modifier
        reach_feet: 10
        targets:
          count: 1
        hit:
        - op: damage
          damage_type: piercing
          amount:
            add:
            - dice:
                count: 1
                sides: 6
              add: 4
            - input: spell_level
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: rend
          count:
            floor_divide:
            - input: spell_level
            - 2
        - op: use_feature
          feature: breath_weapon
          count: 1
      breath_weapon:
        economy: action
        damage_type_choice:
          chooser: summoner
          timing: spell.cast
          options:
          - acid
          - cold
          - fire
          - lightning
          - poison
        op: save
        ability: dexterity
        dc:
          input: summoner_spell_save_dc
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            size_feet: 30
            origin: self
        failure:
        - op: damage
          damage_type:
            choice: breath_damage_type
          amount:
            add:
            - dice:
                count: 2
                sides: 6
              add: 0
        success:
        - op: damage_fraction
          numerator: 1
          denominator: 2
          round: down
    defenses:
      resistances:
        damage_types:
        - acid
        - cold
        - fire
        - lightning
        - poison
      immunities:
        conditions:
        - charmed
        - frightened
        - poisoned
    senses:
      passive_perception: 12
      special:
      - sense: blindsight
        range_feet: 30
      - sense: darkvision
        range_feet: 60
    communication:
      speaks:
      - draconic
      understands:
        union:
        - - draconic
        - languages_known_by: summoner
    proficiency_bonus:
      input: summoner_proficiency_bonus
    scaling_stats:
      ac:
        add:
        - 14
        - input: spell_level
      hit_points:
        add:
        - 50
        - multiply:
          - 10
          - subtract:
            - input: spell_level
            - 5
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
- id: dnd2024:creature_draconic_spirit
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
    summoner:
      type: participant
      required: true
    spell_level:
      type: integer
      required: true
    summoner_spell_attack_modifier:
      type: integer
      required: true
    summoner_spell_save_dc:
      type: integer
      required: true
    summoner_spellcasting_ability_modifier:
      type: integer
      required: true
    summoner_proficiency_bonus:
      type: integer
      required: true
  outputs:
  - resolved_feature
  - effects
  - resource_changes
  events:
  - summon.created
  - summoning_spell.end
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
  source_sha256: fc4d323c81b90c1ae4d392593ccd6c2b74b2c502c712b366a5e9a1b181785c8d
  source_path: builtin/srd-5.2.1/creature/draconic_spirit.md
  pages:
  - 166
  engine_gaps: &id001
  - dnd2024:creature_draconic_spirit
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 1cc8dcf1c79555c5c7f0d0a4bb2dc4c76a48fc1ea6a72d3bb55a25ad1ddfda96
splat_clauses:
- key: source_rules
  text: '# Draconic Spirit Large Dragon, Neutral AC 14 + the spell’s level HP 50 + 10 for each spell level
    above 5 Speed 30 ft., Fly 60 ft., Swim 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 14 +2 +2
    Con17 +3 +3 Int 10 +0 +0 Wis 14 +2 +2 Cha 14 +2 +2 Resistances Acid, Cold, Fire, Lightning, Poison
    Immunities Charmed, Frightened, Poisoned Senses Blindsight 30 ft., Darkvision 60 ft.; Passive Perception
    12 Languages Draconic, understands the languages you know CR None (XP 0; PB equals your Proficiency
    Bonus) ### Traits Shared Resistances. When you summon the spirit, choose one of its Resistances. You
    have Resistance to the chosen damage type until the spell ends. ### Actions Multiattack. The spirit
    makes a number of Rend attacks equal to half the spell’s level (round down), and it uses Breath Weapon.
    Rend. Melee Attack Roll: Bonus equals your spell attack modifier, reach 10 feet. Hit: 1d6 + 4 + the
    spell’s level Piercing damage. Breath Weapon. Dexterity Saving Throw: DC equals your spell save DC,
    each creature in a 30-foot Cone. Failure: 2d6 damage of a type this spirit has Resistance to (your
    choice when you cast the spell). Success: Half damage.'
  sha256: aab24bcc36ef77fbf5df7dd424593e716aa7d3b8df2246ead76978f9ca28fcc6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Draconic Spirit

<!-- SRD page 166 -->

Large Dragon, Neutral\
AC 14 + the spell’s level\
HP 50 + 10 for each spell level above 5\
Speed 30 ft., Fly 60 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 14 +2 +2 Con17 +3 +3\
Int 10 +0 +0 Wis 14 +2 +2 Cha 14 +2 +2\
Resistances Acid, Cold, Fire, Lightning, Poison\
Immunities Charmed, Frightened, Poisoned\
Senses Blindsight 30 ft., Darkvision 60 ft.;\
Passive Perception 12\
Languages Draconic, understands the languages you know\
CR None (XP 0; PB equals your Proficiency Bonus)\

### Traits

Shared Resistances. When you summon the spirit,\
choose one of its Resistances. You have Resistance to\
the chosen damage type until the spell ends.\

### Actions

Multiattack. The spirit makes a number of Rend attacks\
equal to half the spell’s level (round down), and it uses\
Breath Weapon.\
Rend. Melee Attack Roll: Bonus equals your spell attack\
modifier, reach 10 feet. Hit: 1d6 + 4 + the spell’s level\
Piercing damage.\
Breath Weapon. Dexterity Saving Throw: DC equals\
your spell save DC, each creature in a 30-foot Cone.\
Failure: 2d6 damage of a type this spirit has Resistance\
to (your choice when you cast the spell). Success:\
Half damage.\

Original layout: [[../../srd-5.2.1/pages/166|page 166]].
