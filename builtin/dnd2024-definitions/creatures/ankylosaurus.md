---
splat_schema: 1
splat_id: 6675cf17-efcf-52a9-a89f-e9b7a8bac680
splat_kind: creature
splat_key: dnd2024:creatures/ankylosaurus
splat_name: Ankylosaurus
splat_origin:
  key: dnd2024:reference/creature/ankylosaurus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 344
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: beast
  alignment: Unaligned
  tags_text: (Dinosaur)
  movement_text: 30 ft.
  senses_text: Passive Perception 11
  languages_text: None
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 68 (8d12 + 16)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 6
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 15
  save_constitution:
    value: 2
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
    value: 15
  max_hp:
    value: 68
  initiative:
    value: 0
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/ankylosaurus
  implementation: dnd2024:creature_ankylosaurus
  parameters:
    features:
      tail:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 6
        targets:
          count: 1
        hit:
        - op: damage
          average: 9
          amount:
            dice:
              count: 1
              sides: 10
            add: 4
          damage_type: bludgeoning
        - op: if
          test:
            target_kind: creature
            target_size_max: huge
          then:
          - op: condition
            condition: prone
        reach_feet: 10
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: tail
          count: 2
    defenses: {}
    senses:
      passive_perception: 11
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
- id: dnd2024:creature_ankylosaurus
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
  source_sha256: 3da1ca12b45066d7149f3ed4ddc17da56acf27597cdad1c92ff3e8816a97ad4f
  source_path: builtin/srd-5.2.1/creature/ankylosaurus.md
  pages:
  - 344
  engine_gaps: &id001
  - dnd2024:creature_ankylosaurus
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: f3762f7bdb636fad85f1e27e4d0e730cafd18526d60fafcd77cb718635741550
splat_clauses:
- key: source_rules
  text: '# Ankylosaurus Huge Beast (Dinosaur), Unaligned AC 15 Initiative +0 (10) HP 68 (8d12 + 16) Speed
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +6 Dex 11 +0 +0 Con 15 +2 +2 Int 2 -4 -4 Wis 12 +1 +1
    Cha 5 -3 -3 Senses Passive Perception 11 Languages None CR 3 (XP 700; PB +2) ### Actions Multiattack.
    The ankylosaurus makes two Tail attacks. Tail. Melee Attack Roll: +6, reach 10 ft. Hit: 9 (1d10 +
    4) Bludgeoning damage. If the target is a Huge or smaller creature, it has the Prone condition.'
  sha256: 4c34604001a792db093be65b811a29fc142d7d9cca3aeba49c14dc1bad1d295f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ankylosaurus

<!-- SRD page 344 -->

Huge Beast (Dinosaur), Unaligned\
AC 15   Initiative +0 (10)\
HP 68 (8d12 + 16)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +6 Dex 11 +0 +0 Con 15 +2 +2\
Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
Senses Passive Perception 11\
Languages None\
CR 3 (XP 700; PB +2)\

### Actions

Multiattack. The ankylosaurus makes two Tail attacks.\
Tail. Melee Attack Roll: +6, reach 10 ft. Hit: 9 (1d10\
+ 4) Bludgeoning damage. If the target is a Huge or\
smaller creature, it has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/344|page 344]].
