---
splat_schema: 1
splat_id: d7a5ef27-6d5b-5565-a145-b8e10c96067f
splat_kind: creature
splat_key: dnd2024:creatures/plesiosaurus
splat_name: Plesiosaurus
splat_origin:
  key: dnd2024:reference/creature/plesiosaurus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 358, 359
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  tags_text: (Dinosaur)
  movement_text: 20 ft., Swim 40 ft.
  skills_text: Perception +3, Stealth +4
  senses_text: Passive Perception 13
  languages_text: None
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 68 (8d10 + 24)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 13
  max_hp:
    value: 68
  initiative:
    value: 2
  speed:
    value: 20
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/plesiosaurus
  implementation: dnd2024:creature_plesiosaurus
  parameters:
    features:
      bite:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 6
        targets:
          count: 1
        hit:
        - op: damage
          average: 11
          amount:
            dice:
              count: 2
              sides: 6
            add: 4
          damage_type: piercing
        reach_feet: 10
      hold_breath:
        economy: passive
        duration:
          hours: 1
    skill_totals:
      perception: 3
      stealth: 4
    defenses: {}
    senses:
      passive_perception: 13
      special: []
    communication:
      speaks: []
      understands: []
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
- id: dnd2024:creature_plesiosaurus
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
  events: []
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
  source_sha256: 6c889afee8c414995a8d56aebe37cbfa8e358c7558443cfb4fc0e679cb677627
  source_path: builtin/srd-5.2.1/creature/plesiosaurus.md
  pages:
  - 358
  - 359
  engine_gaps: &id001
  - dnd2024:creature_plesiosaurus
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: f1f34696c098657a9b07ddd31c52b636fe918a56dd66fe417783a23358b74301
splat_clauses:
- key: source_rules
  text: '# Plesiosaurus Large Beast (Dinosaur), Unaligned AC 13 Initiative +2 (12) HP 68 (8d10 + 24) Speed
    20 ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 15 +2 +2 Con 16 +3 +3 Int 2 -4 -4
    Wis 12 +1 +1 Cha 5 -3 -3 Skills Perception +3, Stealth +4 Senses Passive Perception 13 Languages None
    CR 2 (XP 450; PB +2) ### Traits Hold Breath. The plesiosaurus can hold its breath for 1 hour. ###
    Actions Bite. Melee Attack Roll: +6, reach 10 ft. Hit: 11 (2d6 + 4) Piercing damage.'
  sha256: 5974992fb861f7cadb4bba528f3ae2abe8f82de262bd60e2fab24cbee039a185
  status: contract_specified
splat_requires_mechanics: *id001
---

# Plesiosaurus

<!-- SRD page 358 -->

Large Beast (Dinosaur), Unaligned\
AC 13  Initiative +2 (12)\
HP 68 (8d10 + 24)\
Speed 20 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 15 +2 +2 Con 16 +3 +3\
Int 2 −4 −4 Wis 12 +1 +1 Cha  5 −3 −3\
Skills Perception +3, Stealth +4\
Senses Passive Perception 13\
Languages None\
CR 2 (XP 450; PB +2)\

### Traits

Hold Breath. The plesiosaurus can hold its breath\
for 1 hour.\

<!-- SRD page 359 -->


### Actions

Bite. Melee Attack Roll: +6, reach 10 ft. Hit: 11 (2d6 +\
4) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/358|page 358]], [[../../srd-5.2.1/pages/359|page 359]].
