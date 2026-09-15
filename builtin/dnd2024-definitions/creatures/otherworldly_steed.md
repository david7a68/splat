---
splat_schema: 1
splat_id: b0baef5c-6b59-58df-8e5d-5c7da31d6d49
splat_kind: creature
splat_key: dnd2024:creatures/otherworldly_steed
splat_name: Otherworldly Steed
splat_origin:
  key: dnd2024:reference/creature/otherworldly_steed
splat_sources:
- title: SRD 5.2.1
  locator: pp. 131
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: celestial
  alignment: Fey, or Fiend (Your Choice), Neutral
  movement_text: 60 ft., Fly 60 ft. (requires level 4+ spell)
  senses_text: Passive Perception 11
  languages_text: Telepathy 1 mile (works only with you)
  cr_text: None (XP 0; PB equals your Proficiency Bonus)
  hit_points_text: 5 + 10 per spell level (the steed has a number of Hit Dice [d10s] equal to the spell’s
    level)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 6
  save_intelligence:
    value: -2
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 10
  speed:
    value: 60
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/otherworldly_steed
  implementation: dnd2024:creature_otherworldly_steed
  parameters:
    features:
      life_bond:
        economy: passive
        trigger: summoner.hit_points_regained
        requires:
          source_spell_level_min: 1
          summoner_distance_max_feet: 5
        op: heal
        amount:
          event_value: summoner_hp_regained
      otherworldly_slam:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus:
          input: summoner_spell_attack_modifier
        reach_feet: 5
        targets:
          count: 1
        hit:
        - op: damage
          damage_type:
            by_self_type:
              celestial: radiant
              fey: psychic
              fiend: necrotic
          amount:
            add:
            - dice:
                count: 1
                sides: 8
              add: 0
            - input: spell_level
      fell_glare:
        economy: bonus_action
        requires_type:
        - fiend
        resource:
          capacity: 1
          cost: 1
          recovery: long_rest
        op: save
        ability: wisdom
        dc:
          input: summoner_spell_save_dc
        targets:
          kind: creature
          count: 1
          range_feet: 60
          visible_to: self
        failure:
        - op: condition
          condition: frightened
          ends:
            event: turn.end
            participant: summoner
            occurrence: next
        success: []
      fey_step:
        economy: bonus_action
        resource:
          capacity: 1
          cost: 1
          recovery: long_rest
        requires_type:
        - fey
        op: teleport
        targets:
        - self
        - rider
        range_feet: 60
        destination:
          unoccupied: true
          chooser: summoner
      healing_touch:
        economy: bonus_action
        resource:
          capacity: 1
          cost: 1
          recovery: long_rest
        requires_type:
        - celestial
        targets:
          kind: creature
          count: 1
          range_feet: 5
        op: heal
        amount:
          add:
          - dice:
              count: 2
              sides: 8
            add: 0
          - input: spell_level
    defenses: {}
    senses:
      passive_perception: 11
      special: []
    communication:
      speaks: []
      understands: []
      telepathy:
        range_feet: 5280
        target: summoner
    proficiency_bonus:
      input: summoner_proficiency_bonus
    scaling_stats:
      ac:
        add:
        - 10
        - input: spell_level
      hit_points:
        add:
        - 5
        - multiply:
          - 10
          - input: spell_level
      hit_dice:
        count:
          input: spell_level
        sides: 10
      fly_speed_feet:
        if:
          spell_level_min: 4
        then: 60
        else: 0
    type_choice:
      chooser: summoner
      options:
      - celestial
      - fey
      - fiend
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
- id: dnd2024:creature_otherworldly_steed
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
  - summoner.hit_points_regained
  - turn.end
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - hit_points
  - movement
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: c6d4edd7d7a5dfbc538966b387a47defe1e5ea2f456c1154a591ed09910dae01
  source_path: builtin/srd-5.2.1/creature/otherworldly_steed.md
  pages:
  - 131
  engine_gaps: &id001
  - dnd2024:creature_otherworldly_steed
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 28c5690e62d7f14340e7c66351db50512056d09a16ead59264045fee3b6b9ebc
splat_clauses:
- key: source_rules
  text: '# Otherworldly Steed Large Celestial, Fey, or Fiend (Your Choice), Neutral AC 10 + 1 per spell
    level HP 5 + 10 per spell level (the steed has a number of Hit Dice [d10s] equal to the spell’s level)
    Speed 60 ft., Fly 60 ft. (requires level 4+ spell) MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 12
    +1 +1 Con14 +2 +2 Int 6 -2 -2 Wis 12 +1 +1 Cha 8 -1 -1 Senses Passive Perception 11 Languages Telepathy
    1 mile (works only with you) CR None (XP 0; PB equals your Proficiency Bonus) ### Traits Life Bond.
    When you regain Hit Points from a level 1+ spell, the steed regains the same number of Hit Points
    if you’re within 5 feet of it. ### Actions Otherworldly Slam. Melee Attack Roll: Bonus equals your
    spell attack modifier, reach 5 ft. Hit: 1d8 plus the spell’s level of Radiant (Celestial), Psychic
    (Fey), or Necrotic (Fiend) damage. ### Bonus Actions Fell Glare (Fiend Only; Recharges after a Long
    Rest). Wisdom Saving Throw: DC equals your spell save DC, one creature within 60 feet the steed can
    see. Failure: The target has the Frightened condition until the end of your next turn. Fey Step (Fey
    Only; Recharges after a Long Rest). The steed teleports, along with its rider, to an unoccupied space
    of your choice up to 60 feet away from itself. Healing Touch (Celestial Only; Recharges after a Long
    Rest). One creature within 5 feet of the steed regains a number of Hit Points equal to 2d8 plus the
    spell’s level.'
  sha256: 2278797af22313a8bbf171857e6b897691f3e97a9dca85ed306cbab3b6b87622
  status: contract_specified
splat_requires_mechanics: *id001
---

# Otherworldly Steed

<!-- SRD page 131 -->

Large Celestial, Fey, or Fiend (Your Choice), Neutral\
AC 10 + 1 per spell level\
HP 5 + 10 per spell level (the steed has a number of Hit\
Dice [d10s] equal to the spell’s level)\
Speed 60 ft., Fly 60 ft. (requires level 4+ spell)\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 12 +1 +1 Con14 +2 +2\
Int 6 −2 −2 Wis 12 +1 +1 Cha 8 −1 −1\
Senses Passive Perception 11\
Languages Telepathy 1 mile (works only with you)\
CR None (XP 0; PB equals your Proficiency Bonus)\

### Traits

Life Bond. When you regain Hit Points from a level 1+\
spell, the steed regains the same number of Hit Points if\
you’re within 5 feet of it.\

### Actions

Otherworldly Slam. Melee Attack Roll: Bonus equals\
your spell attack modifier, reach 5 ft. Hit: 1d8 plus the\
spell’s level of Radiant (Celestial), Psychic (Fey), or Ne-\
crotic (Fiend) damage.\

### Bonus Actions

Fell Glare (Fiend Only; Recharges after a Long Rest).\
Wisdom Saving Throw: DC equals your spell save DC,\
one creature within 60 feet the steed can see. Failure:\
The target has the Frightened condition until the end of\
your next turn.\
Fey Step (Fey Only; Recharges after a Long Rest). The\
steed teleports, along with its rider, to an unoccupied\
space of your choice up to 60 feet away from itself.\
Healing Touch (Celestial Only; Recharges after a Long\
Rest). One creature within 5 feet of the steed regains a\
number of Hit Points equal to 2d8 plus the spell’s level.\

Original layout: [[../../srd-5.2.1/pages/131|page 131]].
