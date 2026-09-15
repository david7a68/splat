---
splat_schema: 1
splat_id: d6cc876e-c146-5ef9-a1f6-ad5d4ec5e569
splat_kind: item
splat_key: dnd2024:items/ring_of_spell_storing
splat_name: Ring of Spell Storing
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_spell_storing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 239
  url: https://www.dndbeyond.com/srd
splat_item:
  category: ring
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/ring_of_spell_storing
  implementation: dnd2024:item_ring_of_spell_storing
  parameters:
    equipped: worn
    spell_storage:
      capacity_spell_levels: 5
      initial_spell_levels:
        roll:
          count: 1
          sides: 6
          add: -1
      initial_spells_choice: gm
      store:
        caster: any_creature
        touch_required_during_cast: true
        spell_level_min: 1
        spell_level_max: 5
        occupancy: cast_slot_level
        normal_spell_effect: false
        insufficient_capacity: expend_without_effect
      release:
        cast_as: user
        copy_original_caster:
        - slot_level
        - save_dc
        - spell_attack_bonus
        - spellcasting_ability
        remove_spell_from_storage: true
        free_capacity: true
    attunement_required: true
splat_contracts:
- id: dnd2024:item_ring_of_spell_storing
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
  - spell.cast_pending
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - item_spellcasting
  - inventory_containers
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 1e3f94479b38a68a973fdd99fda6a02a7e29a7826f7aa5d4495a9e57481b2233
  source_path: builtin/srd-5.2.1/magic_item/ring_of_spell_storing.md
  pages:
  - 239
  engine_gaps: &id001
  - dnd2024:item_ring_of_spell_storing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: d487cad34552fbf31cb0001103bd51c4bf928487d5828c8714e496e88dfe79bc
splat_clauses:
- key: source_rules
  text: '# Ring of Spell Storing Ring, Rare (Requires Attunement) This ring stores spells cast into it,
    holding them until the attuned wearer uses them. The ring can store up to 5 levels worth of spells
    at a time. When found, it contains 1d6 - 1 levels of stored spells chosen by the GM. Any creature
    can cast a spell of level 1 through 5 into the ring by touching the ring as the spell is cast. The
    spell has no effect other than to be stored in the ring. If the ring can’t hold the spell, the spell
    is expended without effect. The level of the slot used to cast the spell determines how much space
    it uses. While wearing this ring, you can cast any spell stored in it. The spell uses the slot level,
    spell save DC, spell attack bonus, and spellcasting ability of the original caster but is otherwise
    treated as if you cast the spell. The spell cast from the ring is no longer stored in it, freeing
    up space.'
  sha256: c36f514a7a4c4422586584351ad06575bef94f799566b080f3eb796d3123dcf3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Spell Storing

<!-- SRD page 239 -->

Ring, Rare (Requires Attunement)\
This ring stores spells cast into it, holding them un-\
til the attuned wearer uses them. The ring can store\
up to 5 levels worth of spells at a time. When found,\
it contains 1d6 − 1 levels of stored spells chosen by\
the GM.\
Any creature can cast a spell of level 1 through 5\
into the ring by touching the ring as the spell is cast.\
The spell has no effect other than to be stored in the\
ring. If the ring can’t hold the spell, the spell is ex-\
pended without effect. The level of the slot used to\
cast the spell determines how much space it uses.\
While wearing this ring, you can cast any spell\
stored in it. The spell uses the slot level, spell save\
DC, spell attack bonus, and spellcasting ability of\
the original caster but is otherwise treated as if you\
cast the spell. The spell cast from the ring is no lon-\
ger stored in it, freeing up space.\

Original layout: [[../../srd-5.2.1/pages/239|page 239]].
