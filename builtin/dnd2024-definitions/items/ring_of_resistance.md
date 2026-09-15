---
splat_schema: 1
splat_id: e61dfba9-0409-5feb-b290-94310960df25
splat_kind: item
splat_key: dnd2024:items/ring_of_resistance
splat_name: Ring of Resistance
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_resistance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 238
  url: https://www.dndbeyond.com/srd
splat_item:
  category: ring
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/ring_of_resistance
  implementation: dnd2024:item_ring_of_resistance
  parameters:
    equipment_state: worn
    variant_selection:
    - gm_choice
    - d10
    variants:
    - roll: 1
      resistance: acid
      gemstone: pearl
    - roll: 2
      resistance: cold
      gemstone: tourmaline
    - roll: 3
      resistance: fire
      gemstone: garnet
    - roll: 4
      resistance: force
      gemstone: sapphire
    - roll: 5
      resistance: lightning
      gemstone: citrine
    - roll: 6
      resistance: necrotic
      gemstone: jet
    - roll: 7
      resistance: poison
      gemstone: amethyst
    - roll: 8
      resistance: psychic
      gemstone: jade
    - roll: 9
      resistance: radiant
      gemstone: topaz
    - roll: 10
      resistance: thunder
      gemstone: spinel
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_ring_of_resistance
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
    campaign:
      required: false
      type: campaign
  outputs:
  - modifiers
  events:
  - equipment.state_changed
  capabilities:
  - equipment
  - derived_statistics
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 645e23d19a36960ca727fb9f59527a786350e306a49adf542830c98b5c938f26
  source_path: builtin/srd-5.2.1/magic_item/ring_of_resistance.md
  pages:
  - 238
  engine_gaps: &id001
  - dnd2024:item_ring_of_resistance
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: ea5475a48d6de0940b96deacf77d884af07b6d759f67438af11bf452b1b23315
splat_clauses:
- key: source_rules
  text: '# Ring of Resistance Ring, Rare You have Resistance to one damage type while wearing this ring.
    The gemstone in the ring indicates the type, which the GM chooses or determines randomly by rolling
    on the following table. 1d10 Damage Type Gemstone 1 Acid Pearl 2 Cold Tourmaline 3 Fire Garnet 4 Force
    Sapphire 5 Lightning Citrine 6 Necrotic Jet 7 Poison Amethyst 8 Psychic Jade 9 Radiant Topaz 10 Thunder
    Spinel'
  sha256: eecf9f439ac43c39780304aeddc5b0a44cce8bcdb89b5072e626c764265cd4a3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Resistance

<!-- SRD page 238 -->

Ring, Rare\
You have Resistance to one damage type while\
wearing this ring. The gemstone in the ring indi-\
cates the type, which the GM chooses or determines\
randomly by rolling on the following table.\
1d10 Damage Type Gemstone\
1 Acid Pearl\
2 Cold Tourmaline\
3 Fire Garnet\
4 Force Sapphire\
5 Lightning Citrine\
6 Necrotic Jet\
7 Poison Amethyst\
8 Psychic Jade\
9 Radiant Topaz\
10 Thunder Spinel\

Original layout: [[../../srd-5.2.1/pages/238|page 238]].
