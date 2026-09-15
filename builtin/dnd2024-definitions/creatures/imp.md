---
splat_schema: 1
splat_id: 7e990459-90d3-5c1a-bd68-288c41d1cfe0
splat_kind: creature
splat_key: dnd2024:creatures/imp
splat_name: Imp
splat_origin:
  key: dnd2024:reference/creature/imp
splat_sources:
- title: SRD 5.2.1
  locator: pp. 300, 301
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: fiend
  alignment: Lawful Evil
  tags_text: (Devil)
  movement_text: 20 ft., Fly 40 ft.
  skills_text: Deception +4, Insight +3, Stealth +5
  resistances_text: Cold
  immunities_text: Fire, Poison; Poisoned
  senses_text: Darkvision 120 ft. (unimpeded by magical Darkness); Passive Perception 11
  languages_text: Common, Infernal
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 21 (6d4 + 6)
splat_stats:
  strength:
    value: 6
  save_strength:
    value: -2
  dexterity:
    value: 17
  save_dexterity:
    value: 3
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 11
  save_intelligence:
    value: 0
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 14
  save_charisma:
    value: 2
  armor_class:
    value: 13
  max_hp:
    value: 21
  initiative:
    value: 3
  speed:
    value: 20
    unit: ft
  fly_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/imp
  implementation: dnd2024:creature_imp
  parameters:
    features:
      magic_resistance:
        economy: passive
        advantage:
          roll: saving_throw
          against:
          - spell
          - magical_effect
      sting:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 6
          amount:
            dice:
              count: 1
              sides: 6
            add: 3
          damage_type: piercing
        - op: damage
          average: 7
          amount:
            dice:
              count: 2
              sides: 6
            add: 0
          damage_type: poison
        reach_feet: 5
      invisibility:
        economy: action
        op: cast_spell
        spell:
          key: dnd2024:spells/invisibility
        ability: charisma
        components_waived:
        - verbal
        - somatic
        - material
        targets: self
        cast_level: spell_minimum
      shape_shift:
        economy: action
        op: shape_shift
        forms:
        - form: rat
          speeds_feet:
            walk: 20
        - form: raven
          speeds_feet:
            walk: 20
            fly: 60
        - form: spider
          speeds_feet:
            walk: 20
            climb: 20
        - form: 'true'
        retain_statistics_except:
        - speed
        equipment_transformed: false
    skill_totals:
      deception: 4
      insight: 3
      stealth: 5
    defenses:
      resistances:
        damage_types:
        - cold
      immunities:
        damage_types:
        - fire
        - poison
        conditions:
        - poisoned
    senses:
      passive_perception: 11
      special:
      - sense: darkvision
        range_feet: 120
        magical_darkness_unimpeded: true
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
- id: dnd2024:creature_imp
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
  - shapechanging
  - spellcasting
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 9ed613d61ad6e883d9a253e9ebe8d4688bc6e054cc9ef7f1b266527e8a29214d
  source_path: builtin/srd-5.2.1/creature/imp.md
  pages:
  - 300
  - 301
  engine_gaps: &id001
  - dnd2024:creature_imp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: e30f5fd8fda6dc9e5595c2f64714d25322a033d785d6c5563565705f175a350b
splat_clauses:
- key: source_rules
  text: '# Imp Tiny Fiend (Devil), Lawful Evil AC 13 Initiative +3 (13) HP 21 (6d4 + 6) Speed 20 ft.,
    Fly 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 6 -2 -2 Dex 17 +3 +3 Con 13 +1 +1 Int 11 +0 +0 Wis 12 +1
    +1 Cha 14 +2 +2 Skills Deception +4, Insight +3, Stealth +5 Resistances Cold Immunities Fire, Poison;
    Poisoned Senses Darkvision 120 ft. (unimpeded by magical Darkness); Passive Perception 11 Languages
    Common, Infernal CR 1 (XP 200; PB +2) ### Traits Magic Resistance. The imp has Advantage on saving
    throws against spells and other magical effects. ### Actions Sting. Melee Attack Roll: +5, reach 5
    ft. Hit: 6 (1d6 + 3) Piercing damage plus 7 (2d6) Poison damage. Invisibility. The imp casts Invisibility
    on itself, requiring no spell components and using Charisma as the spellcasting ability. Shape-Shift.
    The imp shape-shifts to resemble a rat (Speed 20 ft.), a raven (20 ft., Fly 60 ft.), or a spider (20
    ft., Climb 20 ft.), or it returns to its true form. Its game statistics are the same in each form,
    except for its Speed. Any equipment it is wearing or carrying isn’t transformed.'
  sha256: 44646926b1c4cadbf1848200989b1327fc45d1f9a402d5bb4ea23e57253fb376
  status: contract_specified
splat_requires_mechanics: *id001
---

# Imp

<!-- SRD page 300 -->

Tiny Fiend (Devil), Lawful Evil\
AC 13  Initiative +3 (13)\
HP 21 (6d4 + 6)\
Speed 20 ft., Fly 40 ft.\

<!-- SRD page 301 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 6 −2 −2 Dex 17 +3 +3 Con 13 +1 +1\
Int 11 +0 +0 Wis 12 +1 +1 Cha 14 +2 +2\
Skills Deception +4, Insight +3, Stealth +5\
Resistances Cold\
Immunities Fire, Poison; Poisoned\
Senses Darkvision 120 ft. (unimpeded by magical\
Darkness); Passive Perception 11\
Languages Common, Infernal\
CR 1 (XP 200; PB +2)\

### Traits

Magic Resistance. The imp has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Sting. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3)\
Piercing damage plus 7 (2d6) Poison damage.\
Invisibility. The imp casts Invisibility on itself, requiring\
no spell components and using Charisma as the spell-\
casting ability.\
Shape-Shift. The imp shape-shifts to resemble a rat\
(Speed 20 ft.), a raven (20 ft., Fly 60 ft.), or a spider\
(20 ft., Climb 20 ft.), or it returns to its true form. Its\
game statistics are the same in each form, except for\
its Speed. Any equipment it is wearing or carrying isn’t\
transformed.\

Original layout: [[../../srd-5.2.1/pages/300|page 300]], [[../../srd-5.2.1/pages/301|page 301]].
