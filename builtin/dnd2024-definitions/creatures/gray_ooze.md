---
splat_schema: 1
splat_id: 9aba72ea-1243-5435-b57d-530d93f4bf5b
splat_kind: creature
splat_key: dnd2024:creatures/gray_ooze
splat_name: Gray Ooze
splat_origin:
  key: dnd2024:reference/creature/gray_ooze
splat_sources:
- title: SRD 5.2.1
  locator: pp. 293
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: ooze
  alignment: Unaligned
  movement_text: 10 ft., Climb 10 ft.
  skills_text: Stealth +2
  resistances_text: Acid, Cold, Fire
  immunities_text: Blinded, Charmed, Deafened, Exhaustion, Frightened, Grappled, Prone, Restrained
  senses_text: Blindsight 60 ft.; Passive Perception 8
  languages_text: None
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 22 (3d8 + 9)
splat_stats:
  strength:
    value: 12
  save_strength:
    value: 1
  dexterity:
    value: 6
  save_dexterity:
    value: -2
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 6
  save_wisdom:
    value: -2
  charisma:
    value: 2
  save_charisma:
    value: -4
  armor_class:
    value: 9
  max_hp:
    value: 22
  initiative:
    value: -2
  speed:
    value: 10
    unit: ft
  climb_speed:
    value: 10
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/gray_ooze
  implementation: dnd2024:creature_gray_ooze
  parameters:
    skill_totals:
      stealth: 2
    damage_resistances:
    - acid
    - cold
    - fire
    damage_immunities: []
    condition_immunities:
    - blinded
    - charmed
    - deafened
    - exhaustion
    - frightened
    - grappled
    - prone
    - restrained
    senses:
    - sense: blindsight
      range_ft: 60
    passive_perception: 8
    languages:
      speak: true
      known: []
    features:
      amorphous:
        economy: passive
        op: movement.form
        minimum_opening_inches: 1
        extra_movement_cost: 0
      pseudopod:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 3
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: acid
            amount:
              dice:
                count: 2
                sides: 8
              add: 1
            average: 10
        - op: item.ac_penalty
          target:
            category: armor
            worn_by: target
            magical: false
          add: -1
          cumulative: true
          destroy_at_ac: 10
          remove_penalty_spell:
            key: dnd2024:spells/mending
      corrosive_form:
        economy: passive
        ammunition:
          trigger: attack.resolved
          op: item.destroy
          when:
            item_category: ammunition
            item_magical: false
            hit_actor: true
            damage_dealt_to_actor_above: 0
        weapon:
          trigger: damage.applied
          op: item.attack_penalty
          add: -1
          cumulative: true
          destroy_at_penalty: -5
          remove_penalty_spell:
            key: dnd2024:spells/mending
          when:
            item_category: weapon
            item_magical: false
            damage_dealt_to_actor_above: 0
            item_contacts_actor: true
        eat_material:
          materials:
          - nonmagical_metal
          - nonmagical_wood
          thickness_inches: 2
          duration:
            amount: 1
            unit: round
    damage_resolution:
      choice:
      - printed_average
      - roll_expression
      chooser: controller
      exclusive: true
    spellcasting_defaults:
      cast_level: lowest_possible_unless_overridden
      upcast_allowed: only_explicit_cast_level_overrides
      required_material_components_available: true
      casting_time_at_least_1_minute:
        uses_normal_time: true
        magic_action_each_turn: true
        concentration_required: true
      other_components: spell_definition_unless_waived
    spatial_resolution:
      provider: caller
      required_facts:
      - distance
      - area_membership
      - visibility
      - occupancy
      - movement_path
      validate_against_typed_constraints: true
splat_contracts:
- id: dnd2024:creature_gray_ooze
  inputs:
    actor:
      type: participant
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    choices:
      required: false
      type: choice
    scene:
      required: false
      type: resolution
    event:
      required: false
      type: event
  outputs:
  - effects
  - roll_requests
  - state_changes
  events:
  - creature.feature_requested
  - nonmagical_ammunition_hits_and_deals_damage
  - nonmagical_weapon_deals_damage_and_contacts_actor
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - item_state
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 3a18cac86a0bb79e25b18d9bbd11dca100bd85f4dec7c2ce70a2df81f18169d7
  source_path: builtin/srd-5.2.1/creature/gray_ooze.md
  pages:
  - 293
  engine_gaps: &id001
  - dnd2024:creature_gray_ooze
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: ad9b8c2efbff4ae54eba42598f0f28ae6fb4f0fcb3f4f821e722c48df408277d
splat_clauses:
- key: source_rules
  text: '# Gray Ooze Medium Ooze, Unaligned AC 9 Initiative -2 (13) HP 22 (3d8 + 9) Speed 10 ft., Climb
    10 ft. MOD SAVE MOD SAVE MOD SAVE Str 12 +1 +1 Dex 6 -2 -2 Con 16 +3 +3 Int 1 -5 -5 Wis 6 -2 -2 Cha
    2 -4 -4 Skills Stealth +2 Resistances Acid, Cold, Fire Immunities Blinded, Charmed, Deafened, Exhaustion,
    Frightened, Grappled, Prone, Restrained Senses Blindsight 60 ft.; Passive Perception 8 Languages None
    CR 1/2 (XP 100; PB +2) ### Traits Amorphous. The ooze can move through a space as narrow as 1 inch
    without expending extra movement to do so. Corrosive Form. Nonmagical ammunition is destroyed immediately
    after hitting the ooze and dealing any damage. Any nonmagical weapon takes a cumulative -1 penalty
    to attack rolls immediately after dealing damage to the ooze and coming into contact with it. The
    weapon is destroyed if the penalty reaches -5. The penalty can be removed by casting the Mending spell
    on the weapon. The ooze can eat through 2-inch-thick, nonmagical metal or wood in 1 round. ### Actions
    Pseudopod. Melee Attack Roll: +3, reach 5 ft. Hit: 10 (2d8 + 1) Acid damage. Nonmagical armor worn
    by the target takes a -1 penalty to the AC it offers. The armor is destroyed if the penalty reduces
    its AC to 10. The penalty can be removed by casting the Mending spell on the armor.'
  sha256: 54112b3525c02325c1e2c48f615fa424d0a30cd2d57fbd571705c35947e835ea
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gray Ooze

<!-- SRD page 293 -->

Medium Ooze, Unaligned\
AC 9 Initiative −2 (13)\
HP 22 (3d8 + 9)\
Speed 10 ft., Climb 10 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 12 +1 +1 Dex 6 −2 −2 Con 16 +3 +3\
Int 1 −5 −5 Wis 6 −2 −2 Cha 2 −4 −4\
Skills Stealth +2\
Resistances Acid, Cold, Fire\
Immunities Blinded, Charmed, Deafened, Exhaustion,\
Frightened, Grappled, Prone, Restrained\
Senses Blindsight 60 ft.; Passive Perception 8\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Traits

Amorphous. The ooze can move through a space\
as narrow as 1 inch without expending extra move-\
ment to do so.\
Corrosive Form. Nonmagical ammunition is destroyed\
immediately after hitting the ooze and dealing any\
damage. Any nonmagical weapon takes a cumulative\
−1 penalty to attack rolls immediately after dealing\
damage to the ooze and coming into contact with it.\
The weapon is destroyed if the penalty reaches −5. The\
penalty can be removed by casting the Mending spell\
on the weapon.\
The ooze can eat through 2-inch-thick, nonmagical\
metal or wood in 1 round.\

### Actions

Pseudopod. Melee Attack Roll: +3, reach 5 ft. Hit: 10\
(2d8 + 1) Acid damage. Nonmagical armor worn by the\
target takes a −1 penalty to the AC it offers. The armor\
is destroyed if the penalty reduces its AC to 10. The\
penalty can be removed by casting the Mending spell\
on the armor.\

Original layout: [[../../srd-5.2.1/pages/293|page 293]].
