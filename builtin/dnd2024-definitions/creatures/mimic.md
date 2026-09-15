---
splat_schema: 1
splat_id: ce1a24e3-0409-5378-afef-531280f43679
splat_kind: creature
splat_key: dnd2024:creatures/mimic
splat_name: Mimic
splat_origin:
  key: dnd2024:reference/creature/mimic
splat_sources:
- title: SRD 5.2.1
  locator: pp. 308, 309
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: monstrosity
  alignment: Neutral
  movement_text: 20 ft.
  skills_text: Stealth +5
  immunities_text: Acid; Prone
  senses_text: Darkvision 60 ft.; Passive Perception 11
  languages_text: None
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 58 (9d8 + 18)
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
    value: 2
  intelligence:
    value: 5
  save_intelligence:
    value: -3
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 12
  max_hp:
    value: 58
  initiative:
    value: 3
  speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/mimic
  implementation: dnd2024:creature_mimic
  parameters:
    features:
      bite:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: choose_by_predicate
          test:
            target_grappled_by: self
          then:
          - op: damage
            average: 12
            amount:
              dice:
                count: 2
                sides: 8
              add: 3
            damage_type: piercing
          else:
          - op: damage
            average: 7
            amount:
              dice:
                count: 1
                sides: 8
              add: 3
            damage_type: piercing
        - op: damage
          average: 4
          amount:
            dice:
              count: 1
              sides: 8
            add: 0
          damage_type: acid
        reach_feet: 5
        advantage_when:
          target_grappled_by: self
      pseudopod:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 7
          amount:
            dice:
              count: 1
              sides: 8
            add: 3
          damage_type: bludgeoning
        - op: damage
          average: 4
          amount:
            dice:
              count: 1
              sides: 8
            add: 0
          damage_type: acid
        - op: if
          test:
            target_kind: creature
            target_size_max: large
          then:
          - op: condition
            condition: grappled
            escape_dc: 13
            source: self
            escape_check_disadvantage: true
        reach_feet: 5
      adhesive:
        economy: passive
        requires_form:
        - object
        trigger: self.touched
        op: adhere
        targets: touching_thing
        creature_effect:
          op: condition
          condition: grappled
          escape_dc: 13
          source: self
          size_max: huge
          escape_check_disadvantage: true
      shape_shift:
        economy: bonus_action
        op: shape_shift
        forms:
        - form: object
          size_choice:
          - medium
          - small
        - form: true_blob
        retain_statistics: true
        equipment_transformed: false
    skill_totals:
      stealth: 5
    defenses:
      immunities:
        damage_types:
        - acid
        conditions:
        - prone
    senses:
      passive_perception: 11
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
- id: dnd2024:creature_mimic
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
  - self.touched
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - grappling
  - resources
  - shapechanging
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: e21766dc7db90dd84bc96bb1fe498521ed50a8c25c94ff2c705c17cde204b6fb
  source_path: builtin/srd-5.2.1/creature/mimic.md
  pages:
  - 308
  - 309
  engine_gaps: &id001
  - dnd2024:creature_mimic
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 88044cc89c26b14565dc4ddb2c60f503f133e33a5668acd12ed0465154d2923c
splat_clauses:
- key: source_rules
  text: '# Mimic Medium Monstrosity, Neutral AC 12 Initiative +3 (13) HP 58 (9d8 + 18) Speed 20 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 17 +3 +3 Dex 12 +1 +1 Con 15 +2 +2 Int 5 -3 -3 Wis 13 +1 +1 Cha 8 -1 -1
    Skills Stealth +5 Immunities Acid; Prone Senses Darkvision 60 ft.; Passive Perception 11 Languages
    None CR 2 (XP 450; PB +2) ### Traits Adhesive (Object Form Only). The mimic adheres to anything that
    touches it. A Huge or smaller creature adhered to the mimic has the Grappled condition (escape DC
    13). Ability checks made to escape this grapple have Disadvantage. ### Actions Bite. Melee Attack
    Roll: +5 (with Advantage if the target is Grappled by the mimic), reach 5 ft. Hit: 7 (1d8 + 3) Piercing
    damage—or 12 (2d8 + 3) Piercing damage if the target is Grappled by the mimic—plus 4 (1d8) Acid damage.
    Pseudopod. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Bludgeoning damage plus 4 (1d8) Acid
    damage. If the target is a Large or smaller creature, it has the Grappled condition (escape DC 13).
    Ability checks made to escape this grapple have Disadvantage. ### Bonus Actions Shape-Shift. The mimic
    shape-shifts to resemble a Medium or Small object while retaining its game statistics, or it returns
    to its true blob form. Any equipment it is wearing or carrying isn’t transformed.'
  sha256: a1348dc4471c9ac477266e0edcd3e68f2572d4211938f3d60fd34c74c766c44f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mimic

<!-- SRD page 308 -->

Medium Monstrosity, Neutral\
AC 12 Initiative +3 (13)\
HP 58 (9d8 + 18)\
Speed 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 12 +1 +1 Con 15 +2 +2\
Int 5 −3 −3 Wis 13 +1 +1 Cha 8 −1 −1\
Skills Stealth +5\
Immunities Acid; Prone\
Senses Darkvision 60 ft.; Passive Perception 11\
Languages None\
CR 2 (XP 450; PB +2)\

<!-- SRD page 309 -->


### Traits

Adhesive (Object Form Only). The mimic adheres to\
anything that touches it. A Huge or smaller creature ad-\
hered to the mimic has the Grappled condition (escape\
DC 13). Ability checks made to escape this grapple\
have Disadvantage.\

### Actions

Bite. Melee Attack Roll: +5 (with Advantage if the target\
is Grappled by the mimic), reach 5 ft. Hit: 7 (1d8 + 3)\
Piercing damage—or 12 (2d8 + 3) Piercing damage\
if the target is Grappled by the mimic—plus 4 (1d8)\
Acid damage.\
Pseudopod. Melee Attack Roll: +5, reach 5 ft. Hit: 7\
(1d8 + 3) Bludgeoning damage plus 4 (1d8) Acid dam-\
age. If the target is a Large or smaller creature, it has\
the Grappled condition (escape DC 13). Ability checks\
made to escape this grapple have Disadvantage.\

### Bonus Actions

Shape-Shift. The mimic shape-shifts to resemble a Me-\
dium or Small object while retaining its game statistics,\
or it returns to its true blob form. Any equipment it is\
wearing or carrying isn’t transformed.\

Original layout: [[../../srd-5.2.1/pages/308|page 308]], [[../../srd-5.2.1/pages/309|page 309]].
