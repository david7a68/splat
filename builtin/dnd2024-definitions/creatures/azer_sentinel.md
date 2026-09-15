---
splat_schema: 1
splat_id: 03f09722-b0fb-58dd-b1ec-e2e30de53d62
splat_kind: creature
splat_key: dnd2024:creatures/azer_sentinel
splat_name: Azer Sentinel
splat_origin:
  key: dnd2024:reference/creature/azer_sentinel
splat_sources:
- title: SRD 5.2.1
  locator: pp. 261
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: elemental
  alignment: Lawful Neutral
  movement_text: 30 ft.
  immunities_text: Fire, Poison; Poisoned
  senses_text: Passive Perception 11
  languages_text: Primordial (Ignan)
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 39 (6d8 + 12)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 15
  save_constitution:
    value: 4
  intelligence:
    value: 12
  save_intelligence:
    value: 1
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 10
  save_charisma:
    value: 0
  armor_class:
    value: 17
  max_hp:
    value: 39
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/azer_sentinel
  implementation: dnd2024:creature_azer_sentinel
  parameters:
    features:
      burning_hammer:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 8
          amount:
            dice:
              count: 1
              sides: 10
            add: 3
          damage_type: bludgeoning
        - op: damage
          average: 3
          amount:
            dice:
              count: 1
              sides: 6
            add: 0
          damage_type: fire
        reach_feet: 5
      fire_aura:
        economy: passive
        trigger: self.turn.end
        requires:
          self_incapacitated: false
        targets:
          kind: creature
          count: all
          area:
            shape: emanation
            size_feet: 5
            origin: self
          selection: self_choice
        effects:
        - op: damage
          average: 5
          amount:
            dice:
              count: 1
              sides: 10
            add: 0
          damage_type: fire
      illumination:
        economy: passive
        bright_radius_feet: 10
        dim_additional_feet: 10
    defenses:
      immunities:
        damage_types:
        - fire
        - poison
        conditions:
        - poisoned
    senses:
      passive_perception: 11
      special: []
    communication:
      speaks:
      - primordial
      understands:
      - primordial
      dialects:
      - ignan
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
- id: dnd2024:creature_azer_sentinel
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
  - self.turn.end
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
  source_sha256: df1e8217fe6e6e32e563dab698fe2adf33975ef5dfefa410cc4d96eee7baf966
  source_path: builtin/srd-5.2.1/creature/azer_sentinel.md
  pages:
  - 261
  engine_gaps: &id001
  - dnd2024:creature_azer_sentinel
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 823b9024edcb5d895eb7a43e2c0bddd0d3a6aaa01536f0e8b27b76828bd53355
splat_clauses:
- key: source_rules
  text: '# Azer Sentinel Medium Elemental, Lawful Neutral AC 17 Initiative +1 (11) HP 39 (6d8 + 12) Speed
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +3 Dex 12 +1 +1 Con 15 +2 +4 Int 12 +1 +1 Wis 13 +1 +1
    Cha 10 +0 +0 Immunities Fire, Poison; Poisoned Senses Passive Perception 11 Languages Primordial (Ignan)
    CR 2 (XP 450; PB +2) ### Traits Fire Aura. At the end of each of the azer’s turns, each creature of
    the azer’s choice in a 5-foot Emanation originating from the azer takes 5 (1d10) Fire damage unless
    the azer has the Incapacitated condition. Illumination. The azer sheds Bright Light in a 10-foot radius
    and Dim Light for an additional 10 feet. ### Actions Burning Hammer. Melee Attack Roll: +5, reach
    5 ft. Hit: 8 (1d10 + 3) Bludgeoning damage plus 3 (1d6) Fire damage.'
  sha256: 54b40ad604405e53701584118d18a4c14949d30e89319928f544bff91f1f130b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Azer Sentinel

<!-- SRD page 261 -->

Medium Elemental, Lawful Neutral\
AC 17 Initiative +1 (11)\
HP 39 (6d8 + 12)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 12 +1 +1 Con 15 +2 +4\
Int 12 +1 +1 Wis 13 +1 +1 Cha 10 +0 +0\
Immunities Fire, Poison; Poisoned\
Senses Passive Perception 11\
Languages Primordial (Ignan)\
CR 2 (XP 450; PB +2)\

### Traits

Fire Aura. At the end of each of the azer’s turns, each\
creature of the azer’s choice in a 5-foot Emanation orig-\
inating from the azer takes 5 (1d10) Fire damage unless\
the azer has the Incapacitated condition.\
Illumination. The azer sheds Bright Light in a 10-foot\
radius and Dim Light for an additional 10 feet.\

### Actions

Burning Hammer. Melee Attack Roll: +5, reach 5 ft.\
Hit: 8 (1d10 + 3) Bludgeoning damage plus 3 (1d6)\
Fire damage.\

Original layout: [[../../srd-5.2.1/pages/261|page 261]].
