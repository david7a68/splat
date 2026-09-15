---
splat_schema: 1
splat_id: d84e740e-9522-58e4-b9d1-ecf40f3f5485
splat_kind: creature
splat_key: dnd2024:creatures/marilith
splat_name: Marilith
splat_origin:
  key: dnd2024:reference/creature/marilith
splat_sources:
- title: SRD 5.2.1
  locator: pp. 306
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: fiend
  alignment: Chaotic Evil
  tags_text: (Demon)
  movement_text: 40 ft., Climb 40 ft.
  skills_text: Perception +8
  resistances_text: Cold, Fire, Lightning
  immunities_text: Poison; Poisoned
  senses_text: Truesight 120 ft.; Passive Perception 18
  languages_text: Abyssal; telepathy 120 ft.
  cr_text: 16 (XP 15,000; PB +5)
  hit_points_text: 220 (21d10 + 105)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 9
  dexterity:
    value: 20
  save_dexterity:
    value: 5
  constitution:
    value: 20
  save_constitution:
    value: 10
  intelligence:
    value: 18
  save_intelligence:
    value: 4
  wisdom:
    value: 16
  save_wisdom:
    value: 8
  charisma:
    value: 20
  save_charisma:
    value: 10
  armor_class:
    value: 16
  max_hp:
    value: 220
  initiative:
    value: 10
  speed:
    value: 40
    unit: ft
  climb_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/marilith
  implementation: dnd2024:creature_marilith
  parameters:
    skill_totals:
      perception: 8
    damage_resistances:
    - cold
    - fire
    - lightning
    damage_immunities:
    - poison
    condition_immunities:
    - poisoned
    senses:
    - sense: truesight
      range_ft: 120
    passive_perception: 18
    languages:
      speak: true
      known:
      - abyssal
    telepathy:
      range_ft: 120
      two_way: true
    features:
      demonic_restoration:
        economy: passive
        trigger: actor.died
        when:
          plane_not: abyss
        body: disappears
        revive:
          new_body: true
          plane: abyss
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
      reactive:
        economy: passive
        op: economy.reaction_capacity
        count: 1
        refresh: each_combat_turn
      pact_blade:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 10
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
                count: 1
                sides: 10
              add: 5
            average: 10
          - type: necrotic
            amount:
              dice:
                count: 2
                sides: 6
              add: 0
            average: 7
      teleport:
        economy: bonus_action
        op: teleport
        maximum_ft: 120
        destination:
          unoccupied: true
          visible_to_actor: true
        recharge:
          die_sides: 6
          success_min: 5
          success_max: 6
          check_event: actor.turn_start
          maximum_uses: 1
          initial_uses: 1
          also_recovers_after:
          - short_rest
          - long_rest
      parry:
        economy: reaction
        trigger: melee_attack.hit_actor
        requires:
          holding_weapon: true
        op: ac.add
        amount: 5
        scope: triggering_attack
        recheck_hit: true
      constrict:
        economy: action
        op: saving_throw
        ability: strength
        dc: 17
        targets:
          kind: creature
          count: 1
          range_ft: 5
          maximum_size: medium
          visible_to_actor: true
        failure:
        - op: damage
          components:
          - type: bludgeoning
            amount:
              dice:
                count: 2
                sides: 10
              add: 4
            average: 15
        - op: grapple
          escape_dc: 14
          while_grappled:
          - op: condition.apply
            condition: restrained
        success: []
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - feature: pact_blade
          count: 6
        - feature: constrict
          count: 1
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
- id: dnd2024:creature_marilith
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
  - actor.turn_start
  - creature.feature_requested
  - melee_attack.hit_actor
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - resources
  - rest_recovery
  - saving_throws
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 22961ea4a305546eb023f5e5930df71bd3a71a5afbc6ccbc13dfdea040996313
  source_path: builtin/srd-5.2.1/creature/marilith.md
  pages:
  - 306
  engine_gaps: &id001
  - dnd2024:creature_marilith
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 566fa46112eb178b7c5a8e4021d61775aa2f58ea8742e307b00cc8c4dfc335fe
splat_clauses:
- key: source_rules
  text: '# Marilith Large Fiend (Demon), Chaotic Evil AC 16 Initiative +10 (20) HP 220 (21d10 + 105) Speed
    40 ft., Climb 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +9 Dex 20 +5 +5 Con 20 +5 +10 Int 18 +4
    +4 Wis 16 +3 +8 Cha 20 +5 +10 Skills Perception +8 Resistances Cold, Fire, Lightning Immunities Poison;
    Poisoned Senses Truesight 120 ft.; Passive Perception 18 Languages Abyssal; telepathy 120 ft. CR 16
    (XP 15,000; PB +5) ### Traits Demonic Restoration. If the marilith dies outside the Abyss, its body
    dissolves into ichor, and it gains a new body instantly, reviving with all its Hit Points somewhere
    in the Abyss. Magic Resistance. The marilith has Advantage on saving throws against spells and other
    magical effects. Reactive. The marilith can take one Reaction on every turn of combat. ### Actions
    Multiattack. The marilith makes six Pact Blade attacks and uses Constrict. Pact Blade. Melee Attack
    Roll: +10, reach 5 ft. Hit: 10 (1d10 + 5) Slashing damage plus 7 (2d6) Necrotic damage. Constrict.
    Strength Saving Throw: DC 17, one Medium or smaller creature the marilith can see within 5 feet. Failure:
    15 (2d10 + 4) Bludgeoning damage. The target has the Grappled condition (escape DC 14), and it has
    the Restrained condition until the grapple ends. ### Bonus Actions Teleport (Recharge 5–6). The marilith
    teleports up to 120 feet to an unoccupied space it can see. ### Reactions Parry. Trigger: The marilith
    is hit by a melee attack roll while holding a weapon. Response: The marilith adds 5 to its AC against
    that attack, possibly causing it to miss.'
  sha256: 0ab25495bf95aa0e266fb5f2b6c1d0d9b45d394705c0b7d18cac617158ff36bc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Marilith

<!-- SRD page 306 -->

Large Fiend (Demon), Chaotic Evil\
AC 16 Initiative +10 (20)\
HP 220 (21d10 + 105)\
Speed 40 ft., Climb 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +9 Dex 20 +5 +5 Con 20 +5 +10\
Int 18 +4 +4 Wis 16 +3 +8 Cha 20 +5 +10\
Skills Perception +8\
Resistances Cold, Fire, Lightning\
Immunities Poison; Poisoned\
Senses Truesight 120 ft.; Passive Perception 18\
Languages Abyssal; telepathy 120 ft.\
CR 16 (XP 15,000; PB +5)\

### Traits

Demonic Restoration. If the marilith dies outside the\
Abyss, its body dissolves into ichor, and it gains a new\
body instantly, reviving with all its Hit Points some-\
where in the Abyss.\
Magic Resistance. The marilith has Advantage on sav-\
ing throws against spells and other magical effects.\
Reactive. The marilith can take one Reaction on every\
turn of combat.\

### Actions

Multiattack. The marilith makes six Pact Blade attacks\
and uses Constrict.\
Pact Blade. Melee Attack Roll: +10, reach 5 ft. Hit:\
10 (1d10 + 5) Slashing damage plus 7 (2d6) Ne-\
crotic damage.\
Constrict. Strength Saving Throw: DC 17, one Medium\
or smaller creature the marilith can see within 5 feet.\
Failure: 15 (2d10 + 4) Bludgeoning damage. The target\
has the Grappled condition (escape DC 14), and it has\
the Restrained condition until the grapple ends.\

### Bonus Actions

Teleport (Recharge 5–6). The marilith teleports up to\
120 feet to an unoccupied space it can see.\

### Reactions

Parry. Trigger: The marilith is hit by a melee attack roll\
while holding a weapon. Response: The marilith adds 5\
to its AC against that attack, possibly causing it to miss.\

Original layout: [[../../srd-5.2.1/pages/306|page 306]].
