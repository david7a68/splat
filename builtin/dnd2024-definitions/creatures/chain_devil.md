---
splat_schema: 1
splat_id: 4dc854a6-6d82-5089-b95f-ccf2527a0dae
splat_kind: creature
splat_key: dnd2024:creatures/chain_devil
splat_name: Chain Devil
splat_origin:
  key: dnd2024:reference/creature/chain_devil
splat_sources:
- title: SRD 5.2.1
  locator: pp. 272, 273
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: fiend
  alignment: Lawful Evil
  tags_text: (Devil)
  movement_text: 30 ft.
  resistances_text: Bludgeoning, Cold, Piercing, Slashing
  immunities_text: Fire, Poison; Poisoned
  senses_text: Darkvision 120 ft. (unimpeded by magical Darkness); Passive Perception 11
  languages_text: Infernal; telepathy 120 ft.
  cr_text: 8 (XP 3,900; PB +3)
  hit_points_text: 85 (10d8 + 40)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 18
  save_constitution:
    value: 7
  intelligence:
    value: 11
  save_intelligence:
    value: 0
  wisdom:
    value: 12
  save_wisdom:
    value: 4
  charisma:
    value: 14
  save_charisma:
    value: 2
  armor_class:
    value: 15
  max_hp:
    value: 85
  initiative:
    value: 5
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/chain_devil
  implementation: dnd2024:creature_chain_devil
  parameters:
    features:
      diabolical_restoration:
        economy: trait
        operation: passive_rule
        trigger: self.dies
        unless:
          plane: nine_hells
        remove_old_body: true
        new_body:
          delay_seconds: 0
          plane: nine_hells
          hp: maximum
      magic_resistance:
        economy: trait
        operation: passive_rule
        saving_throw_advantage_against:
        - spells
        - magical_effects
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: chain
          count: 2
        - operation: use_feature
          feature: conjure_infernal_chain
      chain:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 7
          reach_ft: 10
        hit:
          damage:
          - average: 11
            roll:
              count: 2
              sides: 6
              modifier: 4
            type: slashing
          grapple:
            condition: grappled
            source: self
            escape_dc: 14
            maximum_size: large
            appendage_pool:
              count: 2
              type: chains
              cost: 1
            while_grappled:
              conditions:
              - restrained
      conjure_infernal_chain:
        economy: action
        operation: saving_throw
        ability: dexterity
        dc: 15
        targets:
          kind: creature
          count: 1
          range_ft: 60
          visible: true
        failure:
          damage:
          - average: 9
            roll:
              count: 2
              sides: 4
              modifier: 4
            type: fire
          conditions:
          - condition: restrained
            until: self.next_turn_end
          pull:
            maximum_size: large
            maximum_ft: 30
            direction: straight_toward_self
          conjured_chain:
            disappears: self.next_turn_end
        success:
          conjured_chain_disappears: true
      unnerving_gaze:
        economy: reaction
        operation: saving_throw
        ability: wisdom
        dc: 15
        failure:
          conditions:
          - condition: frightened
            until: target.turn_end
        trigger: target.turn_start
        targets:
          kind: creature
          count: 1
          range_ft: 30
          visible: true
          target_can_see_self: true
        success:
          immunity:
            effect: this_unnerving_gaze
            duration_seconds: 86400
    damage_resistances:
    - bludgeoning
    - cold
    - piercing
    - slashing
    damage_immunities:
    - fire
    - poison
    condition_immunities:
    - poisoned
    senses:
      darkvision: 120
      passive_perception: 11
      darkvision_ignores_magical_darkness: true
    languages:
      speaks:
      - infernal
      understands: []
      telepathy_range_ft: 120
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
- id: dnd2024:creature_chain_devil
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
  - creature.died
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
  - hit_points
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 0a1837358caf4de285f7b8a14f9cf046926b1a5cfba3e14362f47272d159b488
  source_path: builtin/srd-5.2.1/creature/chain_devil.md
  pages:
  - 272
  - 273
  engine_gaps: &id001
  - dnd2024:creature_chain_devil
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 6cd11da005bb49e9ac8a955e7e8dc2df9be75dd8b35a2299e1f6681877e2dc7c
splat_clauses:
- key: source_rules
  text: '# Chain Devil Medium Fiend (Devil), Lawful Evil AC 15 Initiative +5 (15) HP 85 (10d8 + 40) Speed
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 15 +2 +2 Con 18 +4 +7 Int 11 +0 +0 Wis 12 +1 +4
    Cha 14 +2 +2 Resistances Bludgeoning, Cold, Piercing, Slashing Immunities Fire, Poison; Poisoned Senses
    Darkvision 120 ft. (unimpeded by magical Darkness); Passive Perception 11 Languages Infernal; telepathy
    120 ft. CR 8 (XP 3,900; PB +3) ### Traits Diabolical Restoration. If the devil dies outside the Nine
    Hells, its body disappears in sulfurous smoke, and it gains a new body instantly, reviving with all
    its Hit Points somewhere in the Nine Hells. Magic Resistance. The devil has Advantage on saving throws
    against spells and other magical effects. ### Actions Multiattack. The devil makes two Chain attacks
    and uses Conjure Infernal Chain. Chain. Melee Attack Roll: +7, reach 10 ft. Hit: 11 (2d6 + 4) Slashing
    damage. If the target is a Large or smaller creature, it has the Grappled condition (escape DC 14)
    from one of two chains, and it has the Restrained condition until the grapple ends. Conjure Infernal
    Chain. The devil conjures a fiery chain to bind a creature. Dexterity Saving Throw: DC 15, one creature
    the devil can see within 60 feet. Failure: 9 (2d4 + 4) Fire damage, and the target has the Restrained
    condition until the end of the devil’s next turn, at which point the chain disappears. If the target
    is Large or smaller, the devil moves the target up to 30 feet straight toward itself. Success: The
    chain disappears. ### Reactions Unnerving Gaze. Trigger: A creature the devil can see starts its turn
    within 30 feet of the devil and can see the devil. Response—Wisdom Saving Throw: DC 15, the triggering
    creature. Failure: The target has the Frightened condition until the end of its turn. Success: The
    target is immune to this devil’s Unnerving Gaze for 24 hours.'
  sha256: db42dfa3169acbac9334c251d91c0c44595b8f8bcfae3570744778a9a3f2e49f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Chain Devil

<!-- SRD page 272 -->

Medium Fiend (Devil), Lawful Evil\
AC 15 Initiative +5 (15)\
HP 85 (10d8 + 40)\
Speed 30 ft.\

<!-- SRD page 273 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 15 +2 +2 Con 18 +4 +7\
Int 11 +0 +0 Wis 12 +1 +4 Cha 14 +2 +2\
Resistances Bludgeoning, Cold, Piercing, Slashing\
Immunities Fire, Poison; Poisoned\
Senses Darkvision 120 ft. (unimpeded by magical\
Darkness); Passive Perception 11\
Languages Infernal; telepathy 120 ft.\
CR 8 (XP 3,900; PB +3)\

### Traits

Diabolical Restoration. If the devil dies outside the\
Nine Hells, its body disappears in sulfurous smoke, and\
it gains a new body instantly, reviving with all its Hit\
Points somewhere in the Nine Hells.\
Magic Resistance. The devil has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Multiattack. The devil makes two Chain attacks and\
uses Conjure Infernal Chain.\
Chain. Melee Attack Roll: +7, reach 10 ft. Hit: 11 (2d6\
+ 4) Slashing damage. If the target is a Large or smaller\
creature, it has the Grappled condition (escape DC 14)\
from one of two chains, and it has the Restrained con-\
dition until the grapple ends.\
Conjure Infernal Chain. The devil conjures a fiery\
chain to bind a creature. Dexterity Saving Throw:\
DC 15, one creature the devil can see within 60 feet.\
Failure: 9 (2d4 + 4) Fire damage, and the target has\
the Restrained condition until the end of the devil’s\
next turn, at which point the chain disappears. If the\
target is Large or smaller, the devil moves the target\
up to 30 feet straight toward itself. Success: The chain\
disappears.\

### Reactions

Unnerving Gaze. Trigger: A creature the devil can\
see starts its turn within 30 feet of the devil and can\
see the devil. Response—Wisdom Saving Throw: DC\
15, the triggering creature. Failure: The target has the\
Frightened condition until the end of its turn. Success:\
The target is immune to this devil’s Unnerving Gaze\
for 24 hours.\

Original layout: [[../../srd-5.2.1/pages/272|page 272]], [[../../srd-5.2.1/pages/273|page 273]].
