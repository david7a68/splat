---
splat_schema: 1
splat_id: 1abbab35-5152-58ae-b0dd-6fe0694d1179
splat_kind: creature
splat_key: dnd2024:creatures/noble
splat_name: Noble
splat_origin:
  key: dnd2024:reference/creature/noble
splat_sources:
- title: SRD 5.2.1
  locator: pp. 312
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  skills_text: Deception +5, Insight +4, Persuasion +5
  gear_text: Breastplate, Rapier
  senses_text: Passive Perception 12
  languages_text: Common plus two other languages
  cr_text: 1/8 (XP 25; PB +2)
  hit_points_text: 9 (2d8)
splat_stats:
  strength:
    value: 11
  save_strength:
    value: 0
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 12
  save_intelligence:
    value: 1
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 16
  save_charisma:
    value: 3
  armor_class:
    value: 15
  max_hp:
    value: 9
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/noble
  implementation: dnd2024:creature_noble
  parameters:
    features:
      rapier:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 3
        targets:
          count: 1
        hit:
        - op: damage
          average: 5
          amount:
            dice:
              count: 1
              sides: 8
            add: 1
          damage_type: piercing
        reach_feet: 5
      parry:
        economy: reaction
        trigger: melee_attack.hits_self
        requires:
          holding_weapon: true
        effects:
        - op: modify_ac
          add: 2
          scope: triggering_attack
          reevaluate_hit: true
    skill_totals:
      deception: 5
      insight: 4
      persuasion: 5
    defenses: {}
    senses:
      passive_perception: 12
      special: []
    gear:
    - item_name: Breastplate
      quantity: 1
    - item_name: Rapier
      quantity: 1
    communication:
      speaks:
      - common
      understands:
      - common
      additional_languages:
        count: 2
        chooser: gm
        distinct_from_known: true
        can_speak: true
        can_understand: true
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
- id: dnd2024:creature_noble
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
  - melee_attack.hits_self
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
  source_sha256: a13d73b501fd5f645bf438a180a9f94982188c36b7c6745ae606db2d723ed63f
  source_path: builtin/srd-5.2.1/creature/noble.md
  pages:
  - 312
  engine_gaps: &id001
  - dnd2024:creature_noble
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: f26ed5c085a390a85dba829cb957f2e45c3d5dd3e63c6fd194a6e833ec3b0d36
splat_clauses:
- key: source_rules
  text: '# Noble Medium or Small Humanoid, Neutral AC 15 Initiative +1 (11) HP 9 (2d8) Speed 30 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 11 +0 +0 Dex 12 +1 +1 Con 11 +0 +0 Int 12 +1 +1 Wis 14 +2 +2 Cha 16 +3
    +3 Skills Deception +5, Insight +4, Persuasion +5 Gear Breastplate, Rapier Senses Passive Perception
    12 Languages Common plus two other languages CR 1/8 (XP 25; PB +2) ### Actions Rapier. Melee Attack
    Roll: +3, reach 5 ft. Hit: 5 (1d8 + 1) Piercing damage. ### Reactions Parry. Trigger: The noble is
    hit by a melee attack roll while holding a weapon. Response: The noble adds 2 to its AC against that
    attack, possibly causing it to miss.'
  sha256: 6b4a651982a24b011aa4184408e782c21b4fb4e986677c570fdd622a16b7b5a5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Noble

<!-- SRD page 312 -->

Medium or Small Humanoid, Neutral\
AC 15 Initiative +1 (11)\
HP 9 (2d8)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +0 Dex 12 +1 +1 Con 11 +0 +0\
Int 12 +1 +1 Wis 14 +2 +2 Cha 16 +3 +3\
Skills Deception +5, Insight +4, Persuasion +5\
Gear Breastplate, Rapier\
Senses Passive Perception 12\
Languages Common plus two other languages\
CR 1/8 (XP 25; PB +2)\

### Actions

Rapier. Melee Attack Roll: +3, reach 5 ft. Hit: 5 (1d8\
+ 1) Piercing damage.\

### Reactions

Parry. Trigger: The noble is hit by a melee attack roll\
while holding a weapon. Response: The noble adds 2\
to its AC against that attack, possibly causing it to miss.\

Original layout: [[../../srd-5.2.1/pages/312|page 312]].
