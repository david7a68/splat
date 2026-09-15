---
splat_schema: 1
splat_id: 95b24f2f-1a9c-5078-a67b-46d413130acd
splat_kind: creature
splat_key: dnd2024:creatures/troll_limb
splat_name: Troll Limb
splat_origin:
  key: dnd2024:reference/creature/troll_limb
splat_sources:
- title: SRD 5.2.1
  locator: pp. 333
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: giant
  alignment: Chaotic Evil
  movement_text: 20 ft.
  senses_text: Darkvision 60 ft.; Passive Perception 9
  languages_text: None
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 14 (4d6)
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
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 9
  save_wisdom:
    value: -1
  charisma:
    value: 1
  save_charisma:
    value: -5
  armor_class:
    value: 13
  max_hp:
    value: 14
  initiative:
    value: 1
  speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/troll_limb
  implementation: dnd2024:creature_troll_limb
  parameters:
    features:
      rend:
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
              count: 2
              sides: 4
            add: 4
          damage_type: slashing
        reach_feet: 5
      regeneration:
        economy: passive
        trigger: self.turn.start
        op: heal
        amount:
          constant: 5
        suppression:
          trigger: self.damage_taken
          damage_type_any:
          - acid
          - fire
          skip: next_turn
        death:
          trigger: self.turn.start
          requires:
            hit_points: 0
            regeneration_occurs: false
          exclusive: true
      troll_spawn:
        economy: passive
        senses_as:
          key: dnd2024:creatures/troll
        trigger: self.exists_for_24_hours
        requires:
          not_destroyed: true
        roll:
          dice:
            count: 1
            sides: 12
          add: 0
        table:
        - roll_min: 12
          roll_max: 12
          effect:
            op: replace_stat_block
            definition:
              key: dnd2024:creatures/troll
        - roll_min: 1
          roll_max: 11
          effect:
            op: wither_away
    defenses: {}
    senses:
      passive_perception: 9
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
- id: dnd2024:creature_troll_limb
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
  - self.damage_taken
  - self.exists_for_24_hours
  - self.turn.start
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - hit_points
  - resources
  - stat_block_replacement
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 6fa88395c916648adf081b64edbdeb0cb0f45f55b902f48206af73d49eb6d6c8
  source_path: builtin/srd-5.2.1/creature/troll_limb.md
  pages:
  - 333
  engine_gaps: &id001
  - dnd2024:creature_troll_limb
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 56f9544e0381d7b301c02906dd543d529ad323b8a4f8d661d926bfef6de0bfaa
splat_clauses:
- key: source_rules
  text: '# Troll Limb Small Giant, Chaotic Evil AC 13 Initiative +1 (11) HP 14 (4d6) Speed 20 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 12 +1 +1 Con 10 +0 +0 Int 1 -5 -5 Wis 9 -1 -1 Cha 1 -5 -5
    Senses Darkvision 60 ft.; Passive Perception 9 Languages None CR 1/2 (XP 100; PB +2) ### Traits Regeneration.
    The limb regains 5 Hit Points at the start of each of its turns. If the limb takes Acid or Fire damage,
    this trait doesn’t function on the limb’s next turn. The limb dies only if it starts its turn with
    0 Hit Points and doesn’t regenerate. Troll Spawn. The limb uncannily has the same senses as a whole
    troll. If the limb isn’t destroyed within 24 hours, roll 1d12. On a 12, the limb turns into a Troll.
    Otherwise, the limb withers away. ### Actions Rend. Melee Attack Roll: +6, reach 5 ft. Hit: 9 (2d4
    + 4) Slashing damage.'
  sha256: 0c74f2bd438f27ff1be87a30af8fac7a39224798c722e7a63307df506f1111d9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Troll Limb

<!-- SRD page 333 -->

Small Giant, Chaotic Evil\
AC 13  Initiative +1 (11)\
HP 14 (4d6)\
Speed 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 12 +1 +1 Con 10 +0 +0\
Int 1 −5 −5 Wis 9 −1 −1 Cha 1 −5 −5\
Senses Darkvision 60 ft.; Passive Perception 9\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Traits

Regeneration. The limb regains 5 Hit Points at the start\
of each of its turns. If the limb takes Acid or Fire dam-\
age, this trait doesn’t function on the limb’s next turn.\
The limb dies only if it starts its turn with 0 Hit Points\
and doesn’t regenerate.\
Troll Spawn. The limb uncannily has the same senses\
as a whole troll. If the limb isn’t destroyed within 24\
hours, roll 1d12. On a 12, the limb turns into a Troll.\
Otherwise, the limb withers away.\

### Actions

Rend. Melee Attack Roll: +6, reach 5 ft. Hit: 9 (2d4 +\
4) Slashing damage.\

Original layout: [[../../srd-5.2.1/pages/333|page 333]].
