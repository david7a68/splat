---
splat_schema: 1
splat_id: 578f7a44-f954-5ae1-8faa-a95da7eb24c9
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_3_deflect_attacks
splat_name: 'Level 3: Deflect Attacks'
splat_origin:
  key: dnd2024:reference/pages/051
splat_sources:
- title: SRD 5.2.1
  locator: pp. 51
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/monk_level_3_deflect_attacks
  parameters:
    action: reaction
    trigger: attack.hit_actor
    requires:
      damage_types_include_any:
      - bludgeoning
      - piercing
      - slashing
    reduce_total_damage:
      op: add
      args:
      - dice:
          count: 1
          sides: 10
      - var: actor.abilities.dexterity.modifier
      - var: actor.class_levels.monk
    if_damage_reduced_to_zero:
      optional: true
      cost:
        resource: monk.focus_points
        amount: 1
      target:
        visible: true
        type: creature
        if_melee_attack:
          range_feet: 5
        if_ranged_attack:
          range_feet: 60
          total_cover: false
      save:
        ability: dexterity
        dc:
          op: add
          args:
          - 8
          - var: actor.abilities.wisdom.modifier
          - var: actor.proficiency_bonus
        failure_damage:
          amount:
            op: add
            args:
            - dice:
                count: 2
                sides:
                  martial_arts_die_sides:
                    table:
                      key: dnd2024:tables/monk_progression
                    row:
                      var: actor.class_levels.monk
                    column: martial_arts
            - var: actor.abilities.dexterity.modifier
          type: trigger_attack.damage_type
        success_damage: 0
    class_scope:
      class: monk
      minimum_level: 3
  implementation: dnd2024:class_features_monk_level_3_deflect_attacks
splat_contracts:
- id: dnd2024:class_features_monk_level_3_deflect_attacks
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - attack.hit
  - damage.pending
  capabilities:
  - character_features
  - resources
  - damage
  - proficiencies
  - d20_tests
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 51
  engine_gaps: &id001
  - dnd2024:class_features_monk_level_3_deflect_attacks
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 0beac5e33d2daece4a64692f177bbc482b57e744a1652418b4af2af343ed6647
  parent_name: Monk
  line_span:
    first: 4238
    count: 16
splat_clauses:
- key: source_rules
  text: '# Level 3: Deflect Attacks When an attack roll hits you and its damage includes Bludgeoning,
    Piercing, or Slashing damage, you can take a Reaction to reduce the attack’s total damage against
    you. The reduction equals 1d10 plus your Dexterity modifier and Monk level. If you reduce the damage
    to 0, you can expend 1 Focus Point to redirect some of the attack’s force. If you do so, choose a
    creature you can see within 5 feet of yourself if the attack was a melee attack or a creature you
    can see within 60 feet of yourself that isn’t behind Total Cover if the attack was a ranged attack.
    That creature must succeed on a Dexterity saving throw or take damage equal to two rolls of your Martial
    Arts die plus your Dexterity modifier. The damage is the same type dealt by the attack.'
  sha256: bdee3257f11c821bab9cf7a6e818f98c585f72090516a00c6eace23f2e400818
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Deflect Attacks

<!-- SRD page 51 -->

When an attack roll hits you and its damage in-\
cludes Bludgeoning, Piercing, or Slashing damage,\
you can take a Reaction to reduce the attack’s total\
damage against you. The reduction equals 1d10 plus\
your Dexterity modifier and Monk level.\
If you reduce the damage to 0, you can expend 1\
Focus Point to redirect some of the attack’s force.\
If you do so, choose a creature you can see within 5\
feet of yourself if the attack was a melee attack or a\
creature you can see within 60 feet of yourself that\
isn’t behind Total Cover if the attack was a ranged\
attack. That creature must succeed on a Dexterity\
saving throw or take damage equal to two rolls of\
your Martial Arts die plus your Dexterity modifier.\
The damage is the same type dealt by the attack.\
