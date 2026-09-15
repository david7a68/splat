---
splat_schema: 1
splat_id: 6e1673a3-b02f-5545-a61d-ace62930101c
splat_kind: item
splat_key: dnd2024:items/wand_of_polymorph
splat_name: Wand of Polymorph
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_polymorph
splat_sources:
- title: SRD 5.2.1
  locator: pp. 251
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wand
  rarity: very_rare
  equipment:
    attunement: required
    attunement_requirement_text: a Spellcaster
splat_rule:
  mechanic: dnd2024:items/wand_of_polymorph
  implementation: dnd2024:item_wand_of_polymorph
  parameters:
    equipment_state: held
    charges:
      maximum: 7
      initial: 7
      recovery:
        event: dawn
        amount:
          expression: 1d6+1
          scope: use
        cap: maximum
    spell_grants:
    - spell:
        key: dnd2024:spells/polymorph
      charge_cost: 1
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
      save_dc: 15
    last_charge:
      roll:
        expression: 1d20
        scope: use
      'on': 1
      effect: destroyed
    attunement:
      required: true
      requires_spellcaster: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wands
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_wand_of_polymorph
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
  - time.dawn
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - spellcasting
  - resources_and_time
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 9a0903691ad69e65bda8c766bc6a1cef4fb4a9cdf4703f946fa4c266e8174ede
  source_path: builtin/srd-5.2.1/magic_item/wand_of_polymorph.md
  pages:
  - 251
  engine_gaps: &id001
  - dnd2024:item_wand_of_polymorph
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 29aed6132213cc5a91f5bb27ab48f22175c6fb4b16a6bd37d47991360f35dffe
splat_clauses:
- key: source_rules
  text: '# Wand of Polymorph Wand, Very Rare (Requires Attunement by a Spellcaster) This wand has 7 charges.
    While holding it, you can expend 1 charge to cast Polymorph (save DC 15) from it. Regaining Charges.
    The wand regains 1d6 + 1 expended charges daily at dawn. If you expend the wand’s last charge, roll
    1d20. On a 1, the wand crumbles into ashes and is destroyed.'
  sha256: 48040bb04e0394b59b7d3ac1f4a433e349e882ecca0dfd9e6e70bd2da381eaa0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wand of Polymorph

<!-- SRD page 251 -->

Wand, Very Rare (Requires Attunement by a\
Spellcaster)\
This wand has 7 charges. While holding it, you can\
expend 1 charge to cast Polymorph (save DC 15)\
from it.\
Regaining Charges. The wand regains 1d6 +\
1 expended charges daily at dawn. If you expend\
the wand’s last charge, roll 1d20. On a 1, the wand\
crumbles into ashes and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/251|page 251]].
