---
splat_schema: 1
splat_id: d7d62b80-e321-5292-b081-c21b13ca111a
splat_kind: creature
splat_key: dnd2024:creatures/giant_toad
splat_name: Giant Toad
splat_origin:
  key: dnd2024:reference/creature/giant_toad
splat_sources:
- title: SRD 5.2.1
  locator: pp. 354
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Swim 30 ft.
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: None
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 39 (6d10 + 6)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 11
  max_hp:
    value: 39
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
  swim_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_toad
  implementation: dnd2024:creature_giant_toad
  parameters:
    features:
      amphibious:
        economy: passive
        breathing_media:
        - air
        - water
      standing_leap:
        economy: passive
        long_jump_max_feet: 20
        high_jump_max_feet: 10
        running_start_required: false
      bite:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 4
        targets:
          count: 1
        hit:
        - op: damage
          average: 5
          amount:
            dice:
              count: 1
              sides: 6
            add: 2
          damage_type: piercing
        - op: damage
          average: 5
          amount:
            dice:
              count: 2
              sides: 4
            add: 0
          damage_type: poison
        - op: if
          test:
            target_kind: creature
            target_size_max: medium
          then:
          - op: condition
            condition: grappled
            escape_dc: 12
            source: self
        reach_feet: 5
      swallow:
        economy: action
        op: swallow
        targets:
          count: 1
          grappled_by: self
          size_max: medium
        on_swallow:
        - op: remove_condition
          condition: grappled
        - op: condition
          condition: blinded
        - op: condition
          condition: restrained
        while_swallowed:
          total_cover_against: external_attacks_and_effects
          self_cannot_use:
          - bite
        self_death:
          remove_conditions:
          - restrained
          escape:
            movement_cost_feet: 5
            exit_condition: prone
        digest:
          event: self.turn.end
          occurrence: each
          effects:
          - op: damage
            average: 10
            amount:
              dice:
                count: 3
                sides: 6
              add: 0
            damage_type: acid
        swallowed_capacity: 1
    defenses: {}
    senses:
      passive_perception: 10
      special:
      - sense: darkvision
        range_feet: 60
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
- id: dnd2024:creature_giant_toad
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
  - grappling
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: f359f1c148667823da016a71ab2f7adaa9f7f14519c9b5bc5e4c47b281159dd8
  source_path: builtin/srd-5.2.1/creature/giant_toad.md
  pages:
  - 354
  engine_gaps: &id001
  - dnd2024:creature_giant_toad
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 02b945f88003687a5994351dfad93948e8b4d4a7df008703748dd37c38a1434d
splat_clauses:
- key: source_rules
  text: '# Giant Toad Large Beast, Unaligned AC 11 Initiative +1 (11) HP 39 (6d10 + 6) Speed 30 ft., Swim
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 13 +1 +1 Con 13 +1 +1 Int 2 -4 -4 Wis 10 +0 +0
    Cha 3 -4 -4 Senses Darkvision 60 ft.; Passive Perception 10 Languages None CR 1 (XP 200; PB +2) ###
    Traits Amphibious. The toad can breathe air and water. Standing Leap. The toad’s Long Jump is up to
    20 feet and its High Jump is up to 10 feet with or without a running start. ### Actions Bite. Melee
    Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Piercing damage plus 5 (2d4) Poison damage. If the target
    is a Medium or smaller creature, it has the Grappled condition (escape DC 12). Swallow. The toad swallows
    a Medium or smaller target it is grappling. While swallowed, the target isn’t Grappled but has the
    Blinded and Restrained conditions, and it has Total Cover against attacks and other effects outside
    the toad. In addition, the target takes 10 (3d6) Acid damage at the end of each of the toad’s turns.
    The toad can have only one target swallowed at a time, and it can’t use Bite while it has a swallowed
    target. If the toad dies, a swallowed creature is no longer Restrained and can escape from the corpse
    using 5 feet of movement, exiting with the Prone condition.'
  sha256: 55dbcfa08f5a27821c64c60d235c2e7c38d0763c0e1224ec89abe7aef9b0da3a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Toad

<!-- SRD page 354 -->

Large Beast, Unaligned\
AC 11  Initiative +1 (11)\
HP 39 (6d10 + 6)\
Speed 30 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 13 +1 +1 Con 13 +1 +1\
Int 2 −4 −4 Wis 10 +0 +0 Cha 3 −4 −4\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages None\
CR 1 (XP 200; PB +2)\

### Traits

Amphibious. The toad can breathe air and water.\
Standing Leap. The toad’s Long Jump is up to 20 feet\
and its High Jump is up to 10 feet with or without a\
running start.\

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2)\
Piercing damage plus 5 (2d4) Poison damage. If the tar-\
get is a Medium or smaller creature, it has the Grappled\
condition (escape DC 12).\
Swallow. The toad swallows a Medium or smaller target\
it is grappling. While swallowed, the target isn’t Grap-\
pled but has the Blinded and Restrained conditions,\
and it has Total Cover against attacks and other effects\
outside the toad. In addition, the target takes 10 (3d6)\
Acid damage at the end of each of the toad’s turns. The\
toad can have only one target swallowed at a time, and\
it can’t use Bite while it has a swallowed target. If the\
toad dies, a swallowed creature is no longer Restrained\
and can escape from the corpse using 5 feet of move-\
ment, exiting with the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/354|page 354]].
