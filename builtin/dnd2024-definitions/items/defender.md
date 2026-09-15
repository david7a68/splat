---
splat_schema: 1
splat_id: 584405e4-63b8-5ad4-995a-61f0e6091339
splat_kind: item
splat_key: dnd2024:items/defender
splat_name: Defender
splat_origin:
  key: dnd2024:reference/magic_item/defender
splat_sources:
- title: SRD 5.2.1
  locator: pp. 218
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: legendary
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/defender
  implementation: dnd2024:item_defender
  parameters:
    attack_bonus: 3
    damage_bonus: 3
    allocation:
      event: first_attack_with_item_on_own_turn
      choice:
        integer_min: 0
        integer_max: 3
      ac_bonus: chosen_amount
      attack_and_damage_bonus:
        subtract:
        - 3
        - chosen_amount
      duration_end: actor.next_turn_start
      ac_requires_holding: true
    attunement_required: true
splat_contracts:
- id: dnd2024:item_defender
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    selection:
      required: false
      type: choice
    context:
      required: false
      type: resolution
  outputs:
  - effects
  - item_state_changes
  events:
  - attack.pending
  - turn.started
  - item.equipment_changed
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - effect_duration
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 1a366d1bf7dbdf4d9346491a5c95247a70e06e26206310f0bf24983bc3b649fb
  source_path: builtin/srd-5.2.1/magic_item/defender.md
  pages:
  - 218
  engine_gaps: &id001
  - dnd2024:item_defender
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: eed2afc35005dd8aaf2e29d0bf3eb5d8b1314041e6b74ad502357a118ab785be
splat_clauses:
- key: source_rules
  text: '# Defender Weapon (Any Melee Weapon), Legendary (Requires Attunement) You gain a +3 bonus to
    attack rolls and damage rolls made with this magic weapon. The first time you attack with the weapon
    on each of your turns, you can transfer some or all of the weapon’s bonus to your Armor Class. For
    example, you could reduce the bonus to your attack rolls and damage rolls to +1 and gain a +2 bonus
    to Armor Class. The adjusted bonuses remain in effect until the start of your next turn, although
    you must hold the weapon to gain a bonus to AC from it.'
  sha256: 4d2136599d99e14d069139772862309a581b0471aece6aff77b8169dc4802fb3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Defender

<!-- SRD page 218 -->

Weapon (Any Melee Weapon), Legendary (Requires\
Attunement)\
You gain a +3 bonus to attack rolls and damage rolls\
made with this magic weapon.\
The first time you attack with the weapon on each\
of your turns, you can transfer some or all of the\
weapon’s bonus to your Armor Class. For example,\
you could reduce the bonus to your attack rolls and\
damage rolls to +1 and gain a +2 bonus to Armor\
Class. The adjusted bonuses remain in effect until\
the start of your next turn, although you must hold\
the weapon to gain a bonus to AC from it.\

Original layout: [[../../srd-5.2.1/pages/218|page 218]].
