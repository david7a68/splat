---
splat_schema: 1
splat_id: b9d9ae32-e7fc-5ddc-a4c2-14c1d4f47ee0
splat_kind: creature
splat_key: dnd2024:creatures/pirate_captain
splat_name: Pirate Captain
splat_origin:
  key: dnd2024:reference/creature/pirate_captain
splat_sources:
- title: SRD 5.2.1
  locator: pp. 314
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  skills_text: Acrobatics +7, Perception +5
  gear_text: Pistol, Rapier
  senses_text: Passive Perception 15
  languages_text: Common plus one other language
  cr_text: 6 (XP 2,300; PB +3)
  hit_points_text: 84 (13d8 + 26)
splat_stats:
  strength:
    value: 10
  save_strength:
    value: 3
  dexterity:
    value: 18
  save_dexterity:
    value: 7
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 14
  save_wisdom:
    value: 5
  charisma:
    value: 17
  save_charisma:
    value: 6
  armor_class:
    value: 17
  max_hp:
    value: 84
  initiative:
    value: 7
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/pirate_captain
  implementation: dnd2024:creature_pirate_captain
  parameters:
    features:
      rapier:
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
          damage_type: piercing
        - op: advantage
          participant: self
          roll: attack
          count: 1
          ends:
            event: turn.end
            participant: self
            occurrence: current
        reach_feet: 5
      pistol:
        economy: action
        op: attack
        attack_modes:
        - ranged
        attack_bonus: 7
        targets:
          count: 1
        hit:
        - op: damage
          average: 15
          amount:
            dice:
              count: 2
              sides: 10
            add: 4
          damage_type: piercing
        range_feet: 30
        long_range_feet: 90
      multiattack:
        economy: action
        op: repeat_choice
        count: 3
        choices:
        - feature: rapier
        - feature: pistol
      captain_s_charm:
        economy: bonus_action
        op: save
        ability: wisdom
        dc: 14
        targets:
          kind: creature
          count: 1
          range_feet: 30
          visible_to: self
        failure:
        - op: condition
          condition: charmed
          ends:
            event: turn.start
            participant: self
            occurrence: next
          source: self
        success: []
      riposte:
        economy: reaction
        trigger: melee_attack.hits_self
        requires:
          holding_weapon: true
        effects:
        - op: modify_ac
          add: 3
          scope: triggering_attack
          reevaluate_hit: true
        - op: if
          test:
            triggering_attack_misses_after_ac_bonus: true
            attacker_within_rapier_range: true
          then:
          - op: use_feature
            feature: rapier
            count: 1
            target: triggering_attacker
    skill_totals:
      acrobatics: 7
      perception: 5
    defenses: {}
    senses:
      passive_perception: 15
      special: []
    gear:
    - item_name: Pistol
      quantity: 1
    - item_name: Rapier
      quantity: 1
    communication:
      speaks:
      - common
      understands:
      - common
      additional_languages:
        count: 1
        chooser: gm
        distinct_from_known: true
        can_speak: true
        can_understand: true
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
- id: dnd2024:creature_pirate_captain
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
  - melee_attack.hits_self
  - turn.end
  - turn.start
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 546ca0e7c4cd0f55b3b1fea8279a38f53d78d6933271c0d03b544a50783fc070
  source_path: builtin/srd-5.2.1/creature/pirate_captain.md
  pages:
  - 314
  engine_gaps: &id001
  - dnd2024:creature_pirate_captain
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 41884a23ed0d94a73bebf46c8ea9899c6f12d891983b575a991c665df9bc4ad6
splat_clauses:
- key: source_rules
  text: '# Pirate Captain Medium or Small Humanoid, Neutral AC 17 Initiative +7 (17) HP 84 (13d8 + 26)
    Speed 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 10 +0 +3 Dex 18 +4 +7 Con 14 +2 +2 Int 10 +0 +0 Wis 14
    +2 +5 Cha 17 +3 +6 Skills Acrobatics +7, Perception +5 Gear Pistol, Rapier Senses Passive Perception
    15 Languages Common plus one other language CR 6 (XP 2,300; PB +3) ### Actions Multiattack. The pirate
    makes three attacks, using Rapier or Pistol in any combination. Rapier. Melee Attack Roll: +7, reach
    5 ft. Hit: 13 (2d8 + 4) Piercing damage, and the pirate has Advantage on the next attack roll it makes
    before the end of this turn. Pistol. Ranged Attack Roll: +7, range 30/90 ft. Hit: 15 (2d10 + 4) Piercing
    damage. ### Bonus Actions Captain’s Charm. Wisdom Saving Throw: DC 14, one creature the pirate can
    see within 30 feet. Failure: The target has the Charmed condition until the start of the pirate’s
    next turn. ### Reactions Riposte. Trigger: The pirate is hit by a melee attack roll while holding
    a weapon. Response: The pirate adds 3 to its AC against that attack, possibly causing it to miss.
    On a miss, the pirate makes one Rapier attack against the triggering creature if within range.'
  sha256: 67cba43be7688fb3e6fbcac78651ad92848d1dd7309f374b092ece687ee05a59
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pirate Captain

<!-- SRD page 314 -->

Medium or Small Humanoid, Neutral\
AC 17  Initiative +7 (17)\
HP 84 (13d8 + 26)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +3 Dex 18 +4 +7 Con 14 +2 +2\
Int 10 +0 +0 Wis 14 +2 +5 Cha 17 +3 +6\
Skills Acrobatics +7, Perception +5\
Gear Pistol, Rapier\
Senses Passive Perception 15\
Languages Common plus one other language\
CR 6 (XP 2,300; PB +3)\

### Actions

Multiattack. The pirate makes three attacks, using\
Rapier or Pistol in any combination.\
Rapier. Melee Attack Roll: +7, reach 5 ft. Hit: 13 (2d8\
+ 4) Piercing damage, and the pirate has Advantage on\
the next attack roll it makes before the end of this turn.\
Pistol. Ranged Attack Roll: +7, range 30/90 ft. Hit: 15\
(2d10 + 4) Piercing damage.\

### Bonus Actions

Captain’s Charm. Wisdom Saving Throw: DC 14, one\
creature the pirate can see within 30 feet. Failure: The\
target has the Charmed condition until the start of the\
pirate’s next turn.\

### Reactions

Riposte. Trigger: The pirate is hit by a melee attack roll\
while holding a weapon. Response: The pirate adds 3\
to its AC against that attack, possibly causing it to miss.\
On a miss, the pirate makes one Rapier attack against\
the triggering creature if within range.\

Original layout: [[../../srd-5.2.1/pages/314|page 314]].
