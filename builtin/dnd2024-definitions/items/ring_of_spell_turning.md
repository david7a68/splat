---
splat_schema: 1
splat_id: cda544df-8444-5a1d-810a-e75823ae0f9f
splat_kind: item
splat_key: dnd2024:items/ring_of_spell_turning
splat_name: Ring of Spell Turning
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_spell_turning
splat_sources:
- title: SRD 5.2.1
  locator: pp. 239
  url: https://www.dndbeyond.com/srd
splat_item:
  category: ring
  rarity: legendary
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/ring_of_spell_turning
  implementation: dnd2024:item_ring_of_spell_turning
  parameters:
    equipment_state: worn
    saving_throws_against_spells: advantage
    successful_save:
      maximum_spell_level: 7
      spell_effect_on_user: none
      reflection:
        action: reaction
        requires:
          only_target: user
          area_of_effect: false
        target: original_caster
        save_dc: original_caster_spell_save_dc
        save_against: same_spell
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_ring_of_spell_turning
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    event:
      required: false
      type: event
  outputs:
  - modifiers
  - effects
  - damage
  - item_state
  events:
  - equipment.state_changed
  - attack.resolved
  - saving_throw.resolved
  capabilities:
  - equipment
  - combat_reactions
  - effects
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: dbfcf154d86c143768a37007bb03b1895a32aa176fabdb7c998e32d88921a5f6
  source_path: builtin/srd-5.2.1/magic_item/ring_of_spell_turning.md
  pages:
  - 239
  engine_gaps: &id001
  - dnd2024:item_ring_of_spell_turning
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: ee1cfc148ddf0409d726ae6ae56cf0c26951b4c40c4cb3f5f31463ae101fb6a2
splat_clauses:
- key: source_rules
  text: '# Ring of Spell Turning Ring, Legendary (Requires Attunement) While wearing this ring, you have
    Advantage on saving throws against spells. If you succeed on the save for a spell of level 7 or lower,
    the spell has no effect on you. If that spell targeted only you and didn’t create an area of effect,
    you can take a Reaction to deflect the spell back at the spell’s caster; the caster must make a saving
    throw against the spell using their own spell save DC.'
  sha256: 4e0b803c8e9d470a81f12c2afe65c9285569ddd5749085aa985f5dd88aaae437
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Spell Turning

<!-- SRD page 239 -->

Ring, Legendary (Requires Attunement)\
While wearing this ring, you have Advantage on\
saving throws against spells. If you succeed on the\
save for a spell of level 7 or lower, the spell has no\
effect on you. If that spell targeted only you and\
didn’t create an area of effect, you can take a Reac-\
tion to deflect the spell back at the spell’s caster; the\
caster must make a saving throw against the spell\
using their own spell save DC.\

Original layout: [[../../srd-5.2.1/pages/239|page 239]].
