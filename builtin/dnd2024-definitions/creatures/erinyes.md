---
splat_schema: 1
splat_id: 9d266575-5494-5489-8e63-5a74a106373f
splat_kind: creature
splat_key: dnd2024:creatures/erinyes
splat_name: Erinyes
splat_origin:
  key: dnd2024:reference/creature/erinyes
splat_sources:
- title: SRD 5.2.1
  locator: pp. 283
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: fiend
  alignment: Lawful Evil
  tags_text: (Devil)
  movement_text: 30 ft., Fly 60 ft.
  skills_text: Perception +6, Persuasion +8
  resistances_text: Cold
  immunities_text: Fire, Poison; Poisoned
  senses_text: Truesight 120 ft.; Passive Perception 16
  languages_text: Infernal; telepathy 120 ft.
  cr_text: 12 (XP 8,400; PB +4)
  hit_points_text: 178 (21d8 + 84)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 16
  save_dexterity:
    value: 7
  constitution:
    value: 18
  save_constitution:
    value: 8
  intelligence:
    value: 14
  save_intelligence:
    value: 2
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 18
  save_charisma:
    value: 8
  armor_class:
    value: 18
  max_hp:
    value: 178
  initiative:
    value: 7
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/erinyes
  implementation: dnd2024:creature_erinyes
  parameters:
    skill_totals:
      perception: 6
      persuasion: 8
    damage_resistances:
    - cold
    damage_immunities:
    - fire
    - poison
    condition_immunities:
    - poisoned
    senses:
    - sense: truesight
      range_ft: 120
    passive_perception: 16
    languages:
      speak: true
      known:
      - infernal
    telepathy:
      range_ft: 120
      two_way: true
    features:
      diabolical_restoration:
        economy: passive
        trigger: actor.died
        when:
          plane_not: nine_hells
        body: disappears
        revive:
          new_body: true
          plane: nine_hells
          location_choice: gm
          hp: maximum
          delay:
            amount: 0
            unit: second
      magic_resistance:
        economy: passive
        op: roll.advantage
        rolls:
        - saving_throw
        against:
        - spell
        - magical_effect
      withering_sword:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 8
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: slashing
            amount:
              dice:
                count: 2
                sides: 8
              add: 4
            average: 13
          - type: necrotic
            amount:
              dice:
                count: 2
                sides: 10
              add: 0
            average: 11
      parry:
        economy: reaction
        trigger: melee_attack.hit_actor
        requires:
          holding_weapon: true
        op: ac.add
        amount: 4
        scope: triggering_attack
        recheck_hit: true
      magic_rope:
        economy: passive
        op: equipment.special
        item: magic_rope
        ac: 20
        hp: 90
        damage_immunities:
        - poison
        - psychic
        grants_feature: entangling_rope
        requires_bearing: true
        destroy_when:
        - hp_at_most: 0
        - distance_from_actor_ft_at_least: 5
          duration_at_least:
            amount: 1
            unit: hour
        - actor_dead: true
        restore_fully_after:
        - short_rest
        - long_rest
      entangling_rope:
        economy: action
        op: saving_throw
        ability: strength
        dc: 16
        targets:
          kind: creature
          count: 1
          range_ft: 120
          visible_to_actor: true
        failure:
        - op: damage
          components:
          - type: force
            amount:
              dice:
                count: 4
                sides: 6
              add: 0
            average: 14
        - op: condition.apply
          condition: restrained
          duration:
            until_any:
            - rope.destroyed
            - actor.bonus_action_release
            - actor.uses_entangling_rope_again
        success: []
        requires_item: magic_rope
        release_economy: bonus_action
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - feature: withering_sword
          count: 3
        - feature: entangling_rope
          count: 1
          optional: true
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
- id: dnd2024:creature_erinyes
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
  - actor.died
  - creature.feature_requested
  - melee_attack.hit_actor
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - item_state
  - saving_throws
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 067a60ecb233e1e9321f22ece23bca9e4e69f6dc1cab36f9769c18e467b87462
  source_path: builtin/srd-5.2.1/creature/erinyes.md
  pages:
  - 283
  engine_gaps: &id001
  - dnd2024:creature_erinyes
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 1613b0cd60ca2569f48945a18e82ae4cec3642b897acb2ae6330cae46f20e659
splat_clauses:
- key: source_rules
  text: '# Erinyes Medium Fiend (Devil), Lawful Evil AC 18 Initiative +7 (17) HP 178 (21d8 + 84) Speed
    30 ft., Fly 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 16 +3 +7 Con 18 +4 +8 Int 14 +2 +2
    Wis 14 +2 +2 Cha 18 +4 +8 Skills Perception +6, Persuasion +8 Resistances Cold Immunities Fire, Poison;
    Poisoned Senses Truesight 120 ft.; Passive Perception 16 Languages Infernal; telepathy 120 ft. CR
    12 (XP 8,400; PB +4) ### Traits Diabolical Restoration. If the erinyes dies outside the Nine Hells,
    its body disappears in sulfurous smoke, and it gains a new body instantly, reviving with all its Hit
    Points somewhere in the Nine Hells. Magic Resistance. The erinyes has Advantage on saving throws against
    spells and other magical effects. Magic Rope. The erinyes has a magic rope. While bearing it, the
    erinyes can use the Entangling Rope action. The rope has AC 20, HP 90, and Immunity to Poison and
    Psychic damage. The rope turns to dust if reduced to 0 Hit Points, if it is 5+ feet away from the
    erinyes for 1 hour or more, or if the erinyes dies. If the rope is damaged or destroyed, the erinyes
    can fully restore it when finishing a Short or Long Rest. ### Actions Multiattack. The erinyes makes
    three Withering Sword attacks and can use Entangling Rope. Withering Sword. Melee Attack Roll: +8,
    reach 5 ft. Hit: 13 (2d8 + 4) Slashing damage plus 11 (2d10) Necrotic damage. Entangling Rope (Requires
    Magic Rope). Strength Saving Throw: DC 16, one creature the erinyes can see within 120 feet. Failure:
    14 (4d6) Force damage, and the target has the Restrained condition until the rope is destroyed, the
    erinyes uses a Bonus Action to release the target, or the erinyes uses Entangling Rope again. ###
    Reactions Parry. Trigger: The erinyes is hit by a melee attack roll while holding a weapon. Response:
    The erinyes adds 4 to its AC against that attack, possibly causing it to miss.'
  sha256: e2913a78bfcabd917160d4802e761143da0356d87bf0ad9474e176dd13354756
  status: contract_specified
splat_requires_mechanics: *id001
---

# Erinyes

<!-- SRD page 283 -->

Medium Fiend (Devil), Lawful Evil\
AC 18 Initiative +7 (17)\
HP 178 (21d8 + 84)\
Speed 30 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 16 +3 +7 Con 18 +4 +8\
Int 14 +2 +2 Wis 14 +2 +2 Cha 18 +4 +8\
Skills Perception +6, Persuasion +8\
Resistances Cold\
Immunities Fire, Poison; Poisoned\
Senses Truesight 120 ft.; Passive Perception 16\
Languages Infernal; telepathy 120 ft.\
CR 12 (XP 8,400; PB +4)\

### Traits

Diabolical Restoration. If the erinyes dies outside the\
Nine Hells, its body disappears in sulfurous smoke, and\
it gains a new body instantly, reviving with all its Hit\
Points somewhere in the Nine Hells.\
Magic Resistance. The erinyes has Advantage on saving\
throws against spells and other magical effects.\
Magic Rope. The erinyes has a magic rope. While bear-\
ing it, the erinyes can use the Entangling Rope action.\
The rope has AC 20, HP 90, and Immunity to Poison\
and Psychic damage. The rope turns to dust if reduced\
to 0 Hit Points, if it is 5+ feet away from the erinyes\
for 1 hour or more, or if the erinyes dies. If the rope is\
damaged or destroyed, the erinyes can fully restore it\
when finishing a Short or Long Rest.\

### Actions

Multiattack. The erinyes makes three Withering Sword\
attacks and can use Entangling Rope.\
Withering Sword. Melee Attack Roll: +8, reach 5 ft.\
Hit: 13 (2d8 + 4) Slashing damage plus 11 (2d10) Ne-\
crotic damage.\
Entangling Rope (Requires Magic Rope). Strength\
Saving Throw: DC 16, one creature the erinyes can see\
within 120 feet. Failure: 14 (4d6) Force damage, and the\
target has the Restrained condition until the rope is de-\
stroyed, the erinyes uses a Bonus Action to release the\
target, or the erinyes uses Entangling Rope again.\

### Reactions

Parry. Trigger: The erinyes is hit by a melee attack roll\
while holding a weapon. Response: The erinyes adds 4\
to its AC against that attack, possibly causing it to miss.\

Original layout: [[../../srd-5.2.1/pages/283|page 283]].
