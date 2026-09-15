---
splat_schema: 1
splat_id: 753c9e67-77c6-5e3d-882a-c7e898292576
splat_kind: creature
splat_key: dnd2024:creatures/shadow
splat_name: Shadow
splat_origin:
  key: dnd2024:reference/creature/shadow
splat_sources:
- title: SRD 5.2.1
  locator: pp. 322, 323
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: undead
  alignment: Chaotic Evil
  movement_text: 40 ft.
  skills_text: Stealth +6
  resistances_text: Acid, Cold, Fire, Lightning, Thunder
  vulnerabilities_text: Radiant
  immunities_text: Necrotic, Poison; Exhaustion, Frightened, Grappled, Paralyzed, Petrified, Poisoned,
    Prone, Restrained, Unconscious
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: None
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 27 (5d8 + 5)
splat_stats:
  strength:
    value: 6
  save_strength:
    value: -2
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 6
  save_intelligence:
    value: -2
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 12
  max_hp:
    value: 27
  initiative:
    value: 2
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/shadow
  implementation: dnd2024:creature_shadow
  parameters:
    skill_totals:
      stealth: 6
    damage_vulnerabilities:
    - radiant
    damage_resistances:
    - acid
    - cold
    - fire
    - lightning
    - thunder
    damage_immunities:
    - necrotic
    - poison
    condition_immunities:
    - exhaustion
    - frightened
    - grappled
    - paralyzed
    - petrified
    - poisoned
    - prone
    - restrained
    - unconscious
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 10
    languages:
      speak: true
      known: []
    features:
      amorphous:
        economy: passive
        op: movement.form
        minimum_opening_inches: 1
        extra_movement_cost: 0
      sunlight_weakness:
        economy: passive
        op: roll.disadvantage
        when:
          in_sunlight: true
        rolls:
        - all_d20_tests
      draining_swipe:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 4
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: necrotic
            amount:
              dice:
                count: 1
                sides: 6
              add: 2
            average: 5
        - op: ability_score.reduce
          ability: strength
          amount:
            dice:
              count: 1
              sides: 4
            add: 0
          death_at_zero: true
        on_humanoid_slain:
          op: undead.create
          definition:
            key: dnd2024:creatures/shadow
          delay:
            amount:
              dice:
                count: 1
                sides: 4
              add: 0
            unit: hour
      shadow_stealth:
        economy: bonus_action
        op: action.use
        action: hide
        when:
          lighting:
          - dim_light
          - darkness
    damage_resolution:
      choice:
      - printed_average
      - roll_expression
      chooser: controller
      exclusive: true
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
- id: dnd2024:creature_shadow
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
  capabilities:
  - action_economy
  - attack_resolution
  - creature_lifecycle
  - creature_rules
  - damage
  - effects
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: f3cb1a069086134249a225cf201132270d07193fe1714b67f30c6c26b4961615
  source_path: builtin/srd-5.2.1/creature/shadow.md
  pages:
  - 322
  - 323
  engine_gaps: &id001
  - dnd2024:creature_shadow
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 844b64ee65e2ffb3c195711e1308d838113fb3fe011627940eca0cbdc2855878
splat_clauses:
- key: source_rules
  text: '# Shadow Medium Undead, Chaotic Evil AC 12 Initiative +2 (12) HP 27 (5d8 + 5) Speed 40 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 6 -2 -2 Dex 14 +2 +2 Con 13 +1 +1 Int 6 -2 -2 Wis 10 +0 +0 Cha 8 -1 -1
    Skills Stealth +6 Vulnerabilities Radiant Resistances Acid, Cold, Fire, Lightning, Thunder Immunities
    Necrotic, Poison; Exhaustion, Frightened, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained,
    Unconscious Senses Darkvision 60 ft.; Passive Perception 10 Languages None CR 1/2 (XP 100; PB +2)
    ### Traits Amorphous. The shadow can move through a space as narrow as 1 inch without expending extra
    movement to do so. Sunlight Weakness. While in sunlight, the shadow has Disadvantage on D20 Tests.
    ### Actions Draining Swipe. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Necrotic damage, and
    the target’s Strength score decreases by 1d4. The target dies if this reduces that score to 0. If
    a Humanoid is slain by this attack, a Shadow rises from the corpse 1d4 hours later. ### Bonus Actions
    Shadow Stealth. While in Dim Light or Darkness, the shadow takes the Hide action.'
  sha256: 86f893afc0da3871b9af062ea424b932daeaf5b301e9159114c06eb26f64154b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Shadow

<!-- SRD page 322 -->

Medium Undead, Chaotic Evil\
AC 12  Initiative +2 (12)\
HP 27 (5d8 + 5)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 6 −2 −2 Dex 14 +2 +2 Con 13 +1 +1\
Int 6 −2 −2 Wis 10 +0 +0 Cha 8 −1 −1\
Skills Stealth +6\
Vulnerabilities Radiant\
Resistances Acid, Cold, Fire, Lightning, Thunder\
Immunities Necrotic, Poison; Exhaustion, Frightened,\
Grappled, Paralyzed, Petrified, Poisoned, Prone,\
Restrained, Unconscious\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Traits

Amorphous. The shadow can move through a space\
as narrow as 1 inch without expending extra move-\
ment to do so.\
Sunlight Weakness. While in sunlight, the shadow has\
Disadvantage on D20 Tests.\

<!-- SRD page 323 -->


### Actions

Draining Swipe. Melee Attack Roll: +4, reach 5 ft. Hit:\
5 (1d6 + 2) Necrotic damage, and the target’s Strength\
score decreases by 1d4. The target dies if this reduces\
that score to 0. If a Humanoid is slain by this attack, a\
Shadow rises from the corpse 1d4 hours later.\

### Bonus Actions

Shadow Stealth. While in Dim Light or Darkness, the\
shadow takes the Hide action.\

Original layout: [[../../srd-5.2.1/pages/322|page 322]], [[../../srd-5.2.1/pages/323|page 323]].
