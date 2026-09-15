---
splat_schema: 1
splat_id: d70a4d55-a86b-50ec-822f-6a1be159df0b
splat_kind: item
splat_key: dnd2024:items/rod_of_absorption
splat_name: Rod of Absorption
splat_origin:
  key: dnd2024:reference/magic_item/rod_of_absorption
splat_sources:
- title: SRD 5.2.1
  locator: pp. 241
  url: https://www.dndbeyond.com/srd
splat_item:
  category: rod
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/rod_of_absorption
  implementation: dnd2024:item_rod_of_absorption
  parameters:
    equipment_state: held
    absorb:
      action: reaction
      targeting: only_user
      area_of_effect_allowed: false
      cancel_spell_effect: true
      casting_resources_wasted: true
      energy_gained: cast_spell_level
      lifetime_energy_limit: 50
      overflow: spell_unaffected
      store_spell_identity: false
    attunement_reveals:
    - lifetime_absorbed_levels
    - current_stored_levels
    spend_energy:
      requires_spellcaster: true
      spell_must_be: prepared_or_known
      slot_level_maximum: 5
      cannot_exceed_own_slot_levels: true
      energy_cost: slot_level
      purpose: cast_spell_in_place_of_own_slot
      other_casting_rules: normal
    initial_energy:
      expression: 1d10
      scope: use
    initial_energy_counts_toward_lifetime_absorption: true
    initial_lifetime_absorbed_levels:
      source: item_history
      minimum: initial_stored_energy
      maximum: 50
    becomes_nonmagical_when:
      lifetime_absorbed_levels: 50
      current_stored_levels: 0
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_rods
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_rod_of_absorption
  inputs:
    actor:
      type: participant
      required: true
    source_item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choice:
      required: false
      type: choice
    event:
      required: false
      type: event
    campaign:
      required: false
      type: campaign
  outputs:
  - item_state
  - effects
  - resolutions
  events:
  - spell.cast_pending
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 52a2627bfd9ab803f31051a497ed6ff1c09e9202ee841387f70296d480352987
  source_path: builtin/srd-5.2.1/magic_item/rod_of_absorption.md
  pages:
  - 241
  engine_gaps: &id001
  - dnd2024:item_rod_of_absorption
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 83dcc1490afb0092ae6669b90e9fa6af9a6ff381127f49f21da2705e56057b95
splat_clauses:
- key: source_rules
  text: '# Rod of Absorption Rod, Very Rare (Requires Attunement) While holding this rod, you can take
    a Reaction to absorb a spell that is targeting only you and doesn’t create an area of effect. The
    absorbed spell’s effect is canceled, and the spell’s energy—not the spell itself—is stored in the
    rod. The energy has the same level as the spell when it was cast. A canceled spell dissipates with
    no effect, and any resources used to cast it are wasted. The rod can absorb and store up to 50 levels
    of energy over the course of its existence. Once the rod absorbs 50 levels of energy, it can’t absorb
    more. If you are targeted by a spell that the rod can’t store, the rod has no effect on that spell.
    When you become attuned to the rod, you know how many levels of energy the rod has absorbed over the
    course of its existence and how many levels of spell energy it currently has stored. If you are a
    spellcaster holding the rod, you can convert energy stored in it into spell slots to cast spells you
    have prepared or know. You can create spell slots only of a level equal to or lower than your own
    spell slots, up to a maximum of level 5. You use the stored levels in place of your slots but otherwise
    cast the spell as normal. For example, you can use 3 levels stored in the rod as a level 3 spell slot.
    A newly found rod typically has 1d10 levels of spell energy stored in it. A rod that can no longer
    absorb spell energy and has no energy remaining becomes nonmagical.'
  sha256: 26959cfc778a8272ae55b43cc7e65f34acced19a466f61f117c5144213880ce5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rod of Absorption

<!-- SRD page 241 -->

Rod, Very Rare (Requires Attunement)\
While holding this rod, you can take a Reaction to\
absorb a spell that is targeting only you and doesn’t\
create an area of effect. The absorbed spell’s effect\
is canceled, and the spell’s energy—not the spell it-\
self—is stored in the rod. The energy has the same\
level as the spell when it was cast. A canceled spell\
dissipates with no effect, and any resources used to\
cast it are wasted. The rod can absorb and store up\
to 50 levels of energy over the course of its existence.\
Once the rod absorbs 50 levels of energy, it can’t ab-\
sorb more. If you are targeted by a spell that the rod\
can’t store, the rod has no effect on that spell.\
When you become attuned to the rod, you know\
how many levels of energy the rod has absorbed\
over the course of its existence and how many levels\
of spell energy it currently has stored.\
If you are a spellcaster holding the rod, you can\
convert energy stored in it into spell slots to cast\
spells you have prepared or know. You can create\
spell slots only of a level equal to or lower than your\
own spell slots, up to a maximum of level 5. You use\
the stored levels in place of your slots but otherwise\
cast the spell as normal. For example, you can use 3\
levels stored in the rod as a level 3 spell slot.\
A newly found rod typically has 1d10 levels of\
spell energy stored in it. A rod that can no longer\
absorb spell energy and has no energy remaining\
becomes nonmagical.\

Original layout: [[../../srd-5.2.1/pages/241|page 241]].
