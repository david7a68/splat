---
splat_schema: 1
splat_id: 8b52c7cd-6dc1-5f02-949a-6df4b15a852c
splat_kind: creature
splat_key: dnd2024:creatures/flesh_golem
splat_name: Flesh Golem
splat_origin:
  key: dnd2024:reference/creature/flesh_golem
splat_sources:
- title: SRD 5.2.1
  locator: pp. 285
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: construct
  alignment: Neutral
  movement_text: 30 ft.
  immunities_text: Lightning, Poison; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Understands Common plus one other language but can’t speak
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 127 (15d8 + 60)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 9
  save_dexterity:
    value: -1
  constitution:
    value: 18
  save_constitution:
    value: 4
  intelligence:
    value: 6
  save_intelligence:
    value: -2
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 9
  max_hp:
    value: 127
  initiative:
    value: -1
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/flesh_golem
  implementation: dnd2024:creature_flesh_golem
  parameters:
    features:
      immutable_form:
        economy: passive
        shape_shift_allowed: false
      magic_resistance:
        economy: passive
        advantage:
          roll: saving_throw
          against:
          - spell
          - magical_effect
      slam:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 7
        targets:
          count: 1
        hit:
        - op: damage
          average: 13
          amount:
            dice:
              count: 2
              sides: 8
            add: 4
          damage_type: bludgeoning
        - op: damage
          average: 4
          amount:
            dice:
              count: 1
              sides: 8
            add: 0
          damage_type: lightning
        reach_feet: 5
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: slam
          count: 2
      lightning_absorption:
        economy: passive
        trigger: damage.subjected
        damage_type: lightning
        op: heal
        amount:
          event_value: incoming_damage_of_matching_type
      aversion_to_fire:
        economy: passive
        trigger: self.damage_taken
        requires:
          damage_type: fire
        disadvantage:
          rolls:
          - attack
          - ability_check
          ends:
            event: turn.end
            participant: self
            occurrence: next
      berserk:
        economy: passive
        trigger: self.turn.start
        requires:
          self_bloodied: true
        activation_roll:
          roll:
            dice:
              count: 1
              sides: 6
            add: 0
          success:
          - 6
        state: berserk
        while_active:
          turn_behavior:
            attack_target: nearest_visible_creature
            requires_reachable_with_move_and_attack: true
            fallback_target_kind: object
        ends_any:
        - self_destroyed: true
        - self_bloodied: false
        suppress:
          actor: creator
          economy: action
          range_feet: 60
          self_can_hear_creator: true
          check:
            ability: charisma
            skill: persuasion
            dc: 15
          on_success:
            suppress_until:
              event: turn.start
              participant: self
              occurrence: next
            resume_activation_rolls_if_bloodied: true
    defenses:
      immunities:
        damage_types:
        - lightning
        - poison
        conditions:
        - charmed
        - exhaustion
        - frightened
        - paralyzed
        - petrified
        - poisoned
    senses:
      passive_perception: 10
      special:
      - sense: darkvision
        range_feet: 60
    communication:
      speaks: []
      understands:
      - common
      additional_languages:
        count: 1
        chooser: gm
        distinct_from_known: true
        can_speak: false
        can_understand: true
      can_speak: false
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
- id: dnd2024:creature_flesh_golem
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
  - damage.subjected
  - self.damage_taken
  - self.turn.start
  - turn.end
  - turn.start
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - hit_points
  - resources
  - shapechanging
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 5ef0e407664162639662d26defc923487684fe0371d72787ae0cf36eddea1b65
  source_path: builtin/srd-5.2.1/creature/flesh_golem.md
  pages:
  - 285
  engine_gaps: &id001
  - dnd2024:creature_flesh_golem
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: edd84d5ac41cab17fb31186afe33346dfcb313da9cd90a4c044c5fba20c8b5c1
splat_clauses:
- key: source_rules
  text: '# Flesh Golem Medium Construct, Neutral AC 9 Initiative -1 (9) HP 127 (15d8 + 60) Speed 30 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 9 -1 -1 Con 18 +4 +4 Int 6 -2 -2 Wis 10 +0 +0 Cha 5 -3
    -3 Immunities Lightning, Poison; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned Senses
    Darkvision 60 ft.; Passive Perception 10 Languages Understands Common plus one other language but
    can’t speak CR 5 (XP 1,800; PB +3) ### Traits Aversion to Fire. If the golem takes Fire damage, it
    has Disadvantage on attack rolls and ability checks until the end of its next turn. Berserk. Whenever
    the golem starts its turn Bloodied, roll 1d6. On a 6, the golem goes berserk. On each of its turns
    while berserk, the golem attacks the nearest creature it can see. If no creature is near enough to
    move to and attack, the golem attacks an object. Once the golem goes berserk, it remains so until
    it is destroyed or it is no longer Bloodied. The golem’s creator, if within 60 feet of the berserk
    golem, can try to calm it by taking an action to make a DC 15 Charisma (Persuasion) check; the golem
    must be able to hear its creator. If this check succeeds, the golem ceases being berserk until the
    start of its next turn, at which point it resumes rolling for the Berserk trait again if it is still
    Bloodied. Immutable Form. The golem can’t shape-shift. Lightning Absorption. Whenever the golem is
    subjected to Lightning damage, it regains a number of Hit Points equal to the Lightning damage dealt.
    Magic Resistance. The golem has Advantage on saving throws against spells and other magical effects.
    ### Actions Multiattack. The golem makes two Slam attacks. Slam. Melee Attack Roll: +7, reach 5 ft.
    Hit: 13 (2d8 + 4) Bludgeoning damage plus 4 (1d8) Lightning damage.'
  sha256: 03b6be482d0cbfd0a3e0586bfb8d932bd603cef3b0b33c25effafaacca9685bf
  status: contract_specified
splat_requires_mechanics: *id001
---

# Flesh Golem

<!-- SRD page 285 -->

Medium Construct, Neutral\
AC 9 Initiative −1 (9)\
HP 127 (15d8 + 60)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 9 −1 −1 Con 18 +4 +4\
Int 6 −2 −2 Wis 10 +0 +0 Cha 5 −3 −3\
Immunities Lightning, Poison; Charmed, Exhaustion,\
Frightened, Paralyzed, Petrified, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Understands Common plus one other language\
but can’t speak\
CR 5 (XP 1,800; PB +3)\

### Traits

Aversion to Fire. If the golem takes Fire damage, it has\
Disadvantage on attack rolls and ability checks until the\
end of its next turn.\
Berserk. Whenever the golem starts its turn Bloodied,\
roll 1d6. On a 6, the golem goes berserk. On each of\
its turns while berserk, the golem attacks the nearest\
creature it can see. If no creature is near enough to\
move to and attack, the golem attacks an object. Once\
the golem goes berserk, it remains so until it is de-\
stroyed or it is no longer Bloodied.\
The golem’s creator, if within 60 feet of the berserk\
golem, can try to calm it by taking an action to make a\
DC 15 Charisma (Persuasion) check; the golem must be\
able to hear its creator. If this check succeeds, the go-\
lem ceases being berserk until the start of its next turn,\
at which point it resumes rolling for the Berserk trait\
again if it is still Bloodied.\
Immutable Form. The golem can’t shape-shift.\
Lightning Absorption. Whenever the golem is sub-\
jected to Lightning damage, it regains a number of Hit\
Points equal to the Lightning damage dealt.\
Magic Resistance. The golem has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Multiattack. The golem makes two Slam attacks.\
Slam. Melee Attack Roll: +7, reach 5 ft. Hit: 13 (2d8 +\
4) Bludgeoning damage plus 4 (1d8) Lightning damage.\

Original layout: [[../../srd-5.2.1/pages/285|page 285]].
