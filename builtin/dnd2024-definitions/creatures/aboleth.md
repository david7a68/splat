---
splat_schema: 1
splat_id: 8dc91c9b-4e6d-51e8-97a0-6f23bafeb1ef
splat_kind: creature
splat_key: dnd2024:creatures/aboleth
splat_name: Aboleth
splat_origin:
  key: dnd2024:reference/creature/aboleth
splat_sources:
- title: SRD 5.2.1
  locator: pp. 258
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: aberration
  alignment: Lawful Evil
  movement_text: 10 ft., Swim 40 ft.
  skills_text: History +12, Perception +10
  senses_text: Darkvision 120 ft.; Passive Perception 20
  languages_text: Deep Speech; telepathy 120 ft.
  cr_text: 10 (XP 5,900, or 7,200 in lair; PB +4)
  hit_points_text: 150 (20d10 + 40)
splat_stats:
  strength:
    value: 21
  save_strength:
    value: 5
  dexterity:
    value: 9
  save_dexterity:
    value: 3
  constitution:
    value: 15
  save_constitution:
    value: 6
  intelligence:
    value: 18
  save_intelligence:
    value: 8
  wisdom:
    value: 15
  save_wisdom:
    value: 6
  charisma:
    value: 18
  save_charisma:
    value: 4
  armor_class:
    value: 17
  max_hp:
    value: 150
  initiative:
    value: 7
  speed:
    value: 10
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/aboleth
  implementation: dnd2024:creature_aboleth
  parameters:
    features:
      amphibious:
        economy: trait
        operation: passive_rule
        breathing:
        - air
        - water
      eldritch_restoration:
        economy: trait
        operation: passive_rule
        trigger: self.destroyed
        new_body:
          delay:
            roll:
              count: 5
              sides: 10
              modifier: 0
            unit: day
          hp: maximum
          location:
            choice:
            - far_realm
            - gm_selected_location
            chooser: gm
      legendary_resistance:
        economy: trait
        resource:
          kind: daily
          uses: 3
          recovery: long_rest
          lair_uses: 4
        operation: replace_save_result
        trigger: self.save_failed
        optional: true
        result: success
      mucus_cloud:
        economy: trait
        operation: saving_throw
        ability: constitution
        dc: 14
        targets:
          kind: creature
          selection: each
          area:
            shape: emanation
            size_ft: 5
            origin: self
        failure:
          curse:
            skin: slimy
            breathing:
            - air
            - water
            can_regain_hp_if:
              underwater: true
            dry_damage:
              interval_seconds: 600
              while:
                outside_body_of_water: true
              unless:
                skin_moistened_during_interval: true
              damage:
              - roll:
                  count: 1
                  sides: 12
                  modifier: 0
                type: acid
        trigger: self.turn_end
        requires:
          self_underwater: true
        success:
          effects: []
      probing_telepathy:
        economy: trait
        operation: passive_rule
        trigger: telepathic_message_received
        requires:
          sender_visible: true
        learn:
        - sender.greatest_desires
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: tentacle
          count: 2
        - operation: choice
          options:
          - operation: use_feature
            feature: consume_memories
          - operation: use_feature
            feature: dominate_mind
          require_available: true
      tentacle:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 9
          reach_ft: 15
        hit:
          damage:
          - average: 12
            roll:
              count: 2
              sides: 6
              modifier: 5
            type: bludgeoning
          grapple:
            condition: grappled
            source: self
            escape_dc: 14
            maximum_size: large
            appendage_pool:
              count: 4
              type: tentacles
              cost: 1
      consume_memories:
        economy: action
        operation: saving_throw
        ability: intelligence
        dc: 16
        targets:
          kind: creature
          count: 1
          range_ft: 30
          any_of:
          - charmed_by: self
          - grappled_by: self
        failure:
          damage:
          - average: 10
            roll:
              count: 3
              sides: 6
              modifier: 0
            type: psychic
        success:
          damage_multiplier: 0.5
          other_effects: false
        after_resolution:
          if:
            target_type: humanoid
            target_reduced_to_zero_hp_by_this_action: true
          learn: target.memories
      dominate_mind:
        economy: action
        resource:
          kind: daily
          uses: 2
          recovery: long_rest
        operation: saving_throw
        ability: wisdom
        dc: 16
        targets:
          kind: creature
          count: 1
          range_ft: 30
          visible: true
        failure:
          conditions:
          - condition: charmed
          ends_on:
          - self.dies
          - self_and_target_on_different_planes
          attitude: ally_of_self
          control:
            controller: self
            within_ft: 60
          telepathy:
            with: self
            range: unlimited
          repeat_save:
            events:
            - target.damage_taken
            - event: elapsed_time
              seconds: 86400
              while_distance_at_least_miles: 1
            success: end_effect
        success:
          effects: []
      lash:
        economy: legendary_action
        legendary_cost: 1
        operation: attack_sequence
        feature: tentacle
        count: 1
      psychic_drain:
        economy: legendary_action
        legendary_cost: 1
        operation: sequence
        requires:
          any_creature:
            any_of:
            - charmed_by: self
            - grappled_by: self
        effects:
        - operation: use_feature
          feature: consume_memories
        - operation: heal
          target: self
          roll:
            count: 1
            sides: 10
            modifier: 0
    skill_totals:
      history: 12
      perception: 10
    senses:
      darkvision: 120
      passive_perception: 20
    languages:
      speaks:
      - deep_speech
      understands: []
      telepathy_range_ft: 120
    legendary_actions:
      uses: 3
      recovery: self.turn_start
      trigger: other_creature.turn_end
      cost_per_action: 1
      lair_uses: 4
      requires:
        not_incapacitated: true
      maximum_actions_per_trigger: 1
    resolution_conventions:
      distance_unit: foot
      duration_unit: second
      damage_roll_mode:
        choice:
        - listed_average
        - roll_dice
        chooser: gm
      supplied_totals: before_additional_effect_modifiers
      multiattack_nested_economy_cost: 0
      nested_feature_resource_cost: normal
      spell_level_unless_specified: lowest_possible
      spell_upcast_without_permission: false
      spell_casting_time_at_least_one_minute: normal_casting_procedure
      geometry:
        input: adjudication
        assumptions:
        - range_and_visibility_verified
        - area_membership_verified
        - path_and_destination_legal
      unlisted_success_branch: no_effect
      unlisted_duration: until_removed_by_applicable_rule
splat_contracts:
- id: dnd2024:creature_aboleth
  inputs:
    actor:
      type: participant
      required: true
    feature:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    adjudication:
      required: false
      type: resolution
    trigger:
      required: false
      type: event
  outputs:
  - effects
  events:
  - creature.feature_requested
  - turn.started
  - turn.ended
  - damage.resolved
  - creature.died
  - creature.destroyed
  - rest.completed
  - saving_throw.resolved
  - time.elapsed
  - telepathy.received
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - grappling
  - spatial_queries
  - limited_resources
  - curses
  - legendary_actions
  - hit_points
splat_conversion:
  status: specified
  category: creature
  source_sha256: fa19131d13c3244a24fcbf9fab95955a12b1f66646142838fbd7f353e8875268
  source_path: builtin/srd-5.2.1/creature/aboleth.md
  pages:
  - 258
  engine_gaps: &id001
  - dnd2024:creature_aboleth
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: fba03f177d7e2064ddc3a843c047ffeeee17158e56bc24871ba6f66719568810
splat_clauses:
- key: source_rules
  text: '# Aboleth Large Aberration, Lawful Evil AC 17 Initiative +7 (17) HP 150 (20d10 + 40) Speed 10
    ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 21 +5 +5 Dex 9 -1 +3 Con 15 +2 +6 Int 18 +4 +8 Wis
    15 +2 +6 Cha 18 +4 +4 Skills History +12, Perception +10 Senses Darkvision 120 ft.; Passive Perception
    20 Languages Deep Speech; telepathy 120 ft. CR 10 (XP 5,900, or 7,200 in lair; PB +4) ### Traits Amphibious.
    The aboleth can breathe air and water. Eldritch Restoration. If destroyed, the aboleth gains a new
    body in 5d10 days, reviving with all its Hit Points in the Far Realm or another location chosen by
    the GM. Legendary Resistance (3/Day, or 4/Day in Lair). If the aboleth fails a saving throw, it can
    choose to succeed instead. Mucus Cloud. While underwater, the aboleth is surrounded by mucus. Constitution
    Saving Throw: DC 14, each creature in a 5-foot Emanation originating from the aboleth at the end of
    the aboleth’s turn. Failure: The target is cursed. Until the curse ends, the target’s skin becomes
    slimy, the target can breathe air and water, and it can’t regain Hit Points unless it is underwater.
    While the cursed creature is outside a body of water, the creature takes 6 (1d12) Acid damage at the
    end of every 10 minutes unless moisture is applied to its skin before those minutes have passed. Probing
    Telepathy. If a creature the aboleth can see communicates telepathically with the aboleth, the aboleth
    learns the creature’s greatest desires. ### Actions Multiattack. The aboleth makes two Tentacle attacks
    and uses either Consume Memories or Dominate Mind if available. Tentacle. Melee Attack Roll: +9, reach
    15 ft. Hit: 12 (2d6 + 5) Bludgeoning damage. If the target is a Large or smaller creature, it has
    the Grappled condition (escape DC 14) from one of four tentacles. Consume Memories. Intelligence Saving
    Throw: DC 16, one creature within 30 feet that is Charmed or Grappled by the aboleth. Failure: 10
    (3d6) Psychic damage. Success: Half damage. Failure or Success: The aboleth gains the target’s memories
    if the target is a Humanoid and is reduced to 0 Hit Points by this action. Dominate Mind (2/Day).
    Wisdom Saving Throw: DC 16, one creature the aboleth can see within 30 feet. Failure: The target has
    the Charmed condition until the aboleth dies or is on a different plane of existence from the target.
    While Charmed, the target acts as an ally to the aboleth and is under its control while within 60
    feet of it. In addition, the aboleth and the target can communicate telepathically with each other
    over any distance. The target repeats the save whenever it takes damage as well as after every 24
    hours it spends at least 1 mile away from the aboleth, ending the effect on itself on a success. ###
    Legendary Actions Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s turn,
    the aboleth can expend a use to take one of the following actions. The aboleth regains all expended
    uses at the start of each of its turns. Lash. The aboleth makes one Tentacle attack. Psychic Drain.
    If the aboleth has at least one creature Charmed or Grappled, it uses Consume Memories and regains
    5 (1d10) Hit Points.'
  sha256: 81902e7fe9ecefc7f883efa5964bea2a3c2c9b44a8d57c0d5be57f23d28f8365
  status: contract_specified
splat_requires_mechanics: *id001
---

# Aboleth

<!-- SRD page 258 -->

Large Aberration, Lawful Evil\
AC 17  Initiative +7 (17)\
HP 150 (20d10 + 40)\
Speed 10 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 21 +5 +5 Dex 9 −1 +3 Con 15 +2 +6\
Int 18 +4 +8 Wis 15 +2 +6 Cha 18 +4 +4\
Skills History +12, Perception +10\
Senses Darkvision 120 ft.; Passive Perception 20\
Languages Deep Speech; telepathy 120 ft.\
CR 10 (XP 5,900, or 7,200 in lair; PB +4)\

### Traits

Amphibious. The aboleth can breathe air and water.\
Eldritch Restoration. If destroyed, the aboleth gains\
a new body in 5d10 days, reviving with all its Hit\
Points in the Far Realm or another location chosen\
by the GM.\
Legendary Resistance (3/Day, or 4/Day in Lair). If\
the aboleth fails a saving throw, it can choose to suc-\
ceed instead.\
Mucus Cloud. While underwater, the aboleth is sur-\
rounded by mucus. Constitution Saving Throw: DC 14,\
each creature in a 5-foot Emanation originating from\
the aboleth at the end of the aboleth’s turn. Failure: The\
target is cursed. Until the curse ends, the target’s skin\
becomes slimy, the target can breathe air and water,\
and it can’t regain Hit Points unless it is underwater.\
While the cursed creature is outside a body of water,\
the creature takes 6 (1d12) Acid damage at the end of\
every 10 minutes unless moisture is applied to its skin\
before those minutes have passed.\
Probing Telepathy. If a creature the aboleth can see\
communicates telepathically with the aboleth, the abo-\
leth learns the creature’s greatest desires.\

### Actions

Multiattack. The aboleth makes two Tentacle attacks\
and uses either Consume Memories or Dominate Mind\
if available.\
Tentacle. Melee Attack Roll: +9, reach 15 ft. Hit: 12\
(2d6 + 5) Bludgeoning damage. If the target is a Large\
or smaller creature, it has the Grappled condition (es-\
cape DC 14) from one of four tentacles.\
Consume Memories. Intelligence Saving Throw: DC 16,\
one creature within 30 feet that is Charmed or Grap-\
pled by the aboleth. Failure: 10 (3d6) Psychic damage.\
Success: Half damage. Failure or Success: The aboleth\
gains the target’s memories if the target is a Humanoid\
and is reduced to 0 Hit Points by this action.\
Dominate Mind (2/Day). Wisdom Saving Throw: DC\
16, one creature the aboleth can see within 30 feet.\
Failure: The target has the Charmed condition until\
the aboleth dies or is on a different plane of existence\
from the target. While Charmed, the target acts as an\
ally to the aboleth and is under its control while within\
60 feet of it. In addition, the aboleth and the target\
can communicate telepathically with each other over\
any distance.\
The target repeats the save whenever it takes damage\
as well as after every 24 hours it spends at least 1 mile\
away from the aboleth, ending the effect on itself on\
a success.\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the aboleth can expend a use to\
take one of the following actions. The aboleth regains all\
expended uses at the start of each of its turns.\
Lash. The aboleth makes one Tentacle attack.\
Psychic Drain. If the aboleth has at least one creature\
Charmed or Grappled, it uses Consume Memories and\
regains 5 (1d10) Hit Points.\

Original layout: [[../../srd-5.2.1/pages/258|page 258]].
