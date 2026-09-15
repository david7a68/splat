---
splat_schema: 1
splat_id: 3a0a3b78-4a57-5d28-ac37-c48f7c00ee79
splat_kind: creature
splat_key: dnd2024:creatures/giant_frog
splat_name: Giant Frog
splat_origin:
  key: dnd2024:reference/creature/giant_frog
splat_sources:
- title: SRD 5.2.1
  locator: pp. 351
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Swim 30 ft.
  skills_text: Perception +2, Stealth +4
  senses_text: Darkvision 30 ft.; Passive Perception 12
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 18 (4d8)
splat_stats:
  strength:
    value: 12
  save_strength:
    value: 1
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 11
  max_hp:
    value: 18
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
  swim_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_frog
  implementation: dnd2024:creature_giant_frog
  parameters:
    features:
      amphibious:
        economy: passive
        breathing_media:
        - air
        - water
      standing_leap:
        economy: passive
        long_jump_max_feet: 20
        high_jump_max_feet: 10
        running_start_required: false
      bite:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 3
        targets:
          count: 1
        hit:
        - op: damage
          average: 5
          amount:
            dice:
              count: 1
              sides: 6
            add: 2
          damage_type: piercing
        - op: if
          test:
            target_kind: creature
            target_size_max: medium
          then:
          - op: condition
            condition: grappled
            escape_dc: 11
            source: self
        reach_feet: 5
      swallow:
        economy: action
        op: swallow
        targets:
          count: 1
          grappled_by: self
          size_max: small
        on_swallow:
        - op: remove_condition
          condition: grappled
        - op: condition
          condition: blinded
        - op: condition
          condition: restrained
        while_swallowed:
          total_cover_against: external_attacks_and_effects
          self_cannot_use:
          - bite
        self_death:
          remove_conditions:
          - restrained
          escape:
            movement_cost_feet: 5
            exit_condition: prone
        digest:
          event: self.turn.end
          occurrence: next
          effects:
          - op: damage
            average: 5
            amount:
              dice:
                count: 2
                sides: 4
              add: 0
            damage_type: acid
          after_damage:
          - op: if
            test:
              target_killed: false
            then:
            - op: disgorge
              exit_condition: prone
    skill_totals:
      perception: 2
      stealth: 4
    defenses: {}
    senses:
      passive_perception: 12
      special:
      - sense: darkvision
        range_feet: 30
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
- id: dnd2024:creature_giant_frog
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
  - self.turn.end
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - grappling
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 4d7b76c43cff92aa133d849e88f0175802c043d8770e0588e29621dac9290fcd
  source_path: builtin/srd-5.2.1/creature/giant_frog.md
  pages:
  - 351
  engine_gaps: &id001
  - dnd2024:creature_giant_frog
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: d5b93868e78d1c469d922951506dbd664ed1ccef152074c238e268cf6e90a898
splat_clauses:
- key: source_rules
  text: '# Giant Frog Medium Beast, Unaligned AC 11 Initiative +1 (11) HP 18 (4d8) Speed 30 ft., Swim
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 12 +1 +1 Dex 13 +1 +1 Con 11 +0 +0 Int 2 -4 -4 Wis 10 +0 +0
    Cha 3 -4 -4 Skills Perception +2, Stealth +4 Senses Darkvision 30 ft.; Passive Perception 12 Languages
    None CR 1/4 (XP 50; PB +2) ### Traits Amphibious. The frog can breathe air and water. Standing Leap.
    The frog’s Long Jump is up to 20 feet and its High Jump is up to 10 feet with or without a running
    start. ### Actions Bite. Melee Attack Roll: +3, reach 5 ft. Hit: 5 (1d6 + 2) Piercing damage. If the
    target is a Medium or smaller creature, it has the Grappled condition (escape DC 11). Swallow. The
    frog swallows a Small or smaller target it is grappling. While swallowed, the target isn’t Grappled
    but has the Blinded and Restrained conditions, and it has Total Cover against attacks and other effects
    outside the frog. While swallowing the target, the frog can’t use Bite, and if the frog dies, the
    swallowed target is no longer Restrained and can escape from the corpse using 5 feet of movement,
    exiting with the Prone condition. At the end of the frog’s next turn, the swallowed target takes 5
    (2d4) Acid damage. If that damage doesn’t kill it, the frog disgorges it, causing it to exit Prone.'
  sha256: 1400f83bf0d30adf554d8d696deb9d1f31fc0ab0f05fd43154ed8ab39aff569f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Frog

<!-- SRD page 351 -->

Medium Beast, Unaligned\
AC 11  Initiative +1 (11)\
HP 18 (4d8)\
Speed 30 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 12 +1 +1 Dex 13 +1 +1 Con 11 +0 +0\
Int 2 −4 −4 Wis 10 +0 +0 Cha 3 −4 −4\
Skills Perception +2, Stealth +4\
Senses Darkvision 30 ft.; Passive Perception 12\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Traits

Amphibious. The frog can breathe air and water.\
Standing Leap. The frog’s Long Jump is up to 20 feet\
and its High Jump is up to 10 feet with or without a\
running start.\

### Actions

Bite. Melee Attack Roll: +3, reach 5 ft. Hit: 5 (1d6 + 2)\
Piercing damage. If the target is a Medium or smaller\
creature, it has the Grappled condition (escape DC 11).\
Swallow. The frog swallows a Small or smaller target it\
is grappling. While swallowed, the target isn’t Grappled\
but has the Blinded and Restrained conditions, and it\
has Total Cover against attacks and other effects outside\
the frog. While swallowing the target, the frog can’t use\
Bite, and if the frog dies, the swallowed target is no lon-\
ger Restrained and can escape from the corpse using 5\
feet of movement, exiting with the Prone condition.\
At the end of the frog’s next turn, the swallowed target\
takes 5 (2d4) Acid damage. If that damage doesn’t kill it,\
the frog disgorges it, causing it to exit Prone.\

Original layout: [[../../srd-5.2.1/pages/351|page 351]].
