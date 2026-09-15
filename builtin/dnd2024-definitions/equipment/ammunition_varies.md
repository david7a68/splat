---
splat_schema: 1
splat_id: 195d81fd-98a0-57e1-b2b3-134a814d0214
splat_kind: item
splat_key: dnd2024:equipment/ammunition_varies
splat_name: Ammunition (Varies)
splat_origin:
  key: dnd2024:reference/pages/094
splat_sources:
- title: SRD 5.2.1
  locator: pp. 94, 95, 96
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:equipment/ammunition_varies
  implementation: dnd2024:equipment_ammunition_varies
  parameters:
    required_by_property: ammunition
    weapon_selects_type: true
    storage_included: false
    variants:
    - id: arrows
      purchase_quantity: 20
      storage:
        key: dnd2024:equipment/quiver_1_gp
      mass:
        value: 1
        unit: lb
      price:
        value: 1
        unit: gp
    - id: bolts
      purchase_quantity: 20
      storage:
        key: dnd2024:equipment/case_crossbow_bolt_1_gp
      mass:
        value: 1.5
        unit: lb
      price:
        value: 1
        unit: gp
    - id: firearm_bullets
      purchase_quantity: 10
      storage:
        key: dnd2024:equipment/pouch_5_sp
      mass:
        value: 2
        unit: lb
      price:
        value: 3
        unit: gp
    - id: sling_bullets
      purchase_quantity: 20
      storage:
        key: dnd2024:equipment/pouch_5_sp
      mass:
        value: 1.5
        unit: lb
      price:
        value: 4
        unit: cp
    - id: needles
      purchase_quantity: 50
      storage:
        key: dnd2024:equipment/pouch_5_sp
      mass:
        value: 1
        unit: lb
      price:
        value: 1
        unit: gp
splat_contracts:
- id: dnd2024:equipment_ammunition_varies
  inputs:
    item:
      type: item_instance
      required: true
    variant:
      type: choice
      required: true
    weapon:
      required: false
      type: item_instance
  outputs:
  - variant_properties
  - ammunition_compatibility
  - purchase_quantities
  events: []
  capabilities:
  - ammunition
  - item_variants
  - inventory
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 94
  - 95
  - 96
  engine_gaps: &id001
  - dnd2024:equipment_ammunition_varies
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: c166bfc3f66d80a348ca9de10c6a7f5c301ec19c329de86a0c96f1e31a056e6d
  line_span:
    first: 8161
    count: 100
splat_clauses:
- key: source_rules
  text: '# Ammunition (Varies) Ammunition is required by a weapon that has the Ammunition property. A
    weapon’s description specifies the type of ammunition used by the weapon. The Ammunition table lists
    the different types and the amount you get when you buy them. The table also lists the item that is
    typically used to store each type; storage must be bought separately. Adventuring Gear Item Weight
    Cost Acid 1 lb. 25 GP Alchemist’s Fire 1 lb. 50 GP Ammunition Varies Varies Antitoxin — 50 GP Arcane
    Focus Varies Varies Backpack 5 lb. 2 GP Ball Bearings 2 lb. 1 GP Barrel 70 lb. 2 GP Basket 2 lb. 4
    SP Bedroll 7 lb. 1 GP Bell — 1 GP Blanket 3 lb. 5 SP Block and Tackle 5 lb. 1 GP Book 5 lb. 25 GP
    Bottle, Glass 2 lb. 2 GP Bucket 2 lb. 5 CP Burglar’s Pack 42 lb. 16 GP Caltrops 2 lb. 1 GP Candle
    — 1 CP Case, Crossbow Bolt 1 lb. 1 GP Case, Map or Scroll 1 lb. 1 GP Chain 10 lb. 5 GP Chest 25 lb.
    5 GP Climber’s Kit 12 lb. 25 GP Clothes, Fine 6 lb. 15 GP Clothes, Traveler’s 4 lb. 2 GP Component
    Pouch 2 lb. 25 GP Costume 4 lb. 5 GP Crowbar 5 lb. 2 GP Diplomat’s Pack 39 lb. 39 GP Druidic Focus
    Varies Varies Dungeoneer’s Pack 55 lb. 12 GP Entertainer’s Pack 58½ lb. 40 GP Explorer’s Pack 55 lb.
    10 GP Flask 1 lb. 2 CP Grappling Hook 4 lb. 2 GP Healer’s Kit 3 lb. 5 GP Holy Symbol Varies Varies
    Holy Water 1 lb. 25 GP Hunting Trap 25 lb. 5 GP Ink — 10 GP Item Weight Cost Ink Pen — 2 CP Jug 4
    lb. 2 CP Ladder 25 lb. 1 SP Lamp 1 lb. 5 SP Lantern, Bullseye 2 lb. 10 GP Lantern, Hooded 2 lb. 5
    GP Lock 1 lb. 10 GP Magnifying Glass — 100 GP Manacles 6 lb. 2 GP Map — 1 GP Mirror 1/2 lb. 5 GP Net
    3 lb. 1 GP Oil 1 lb. 1 SP Paper — 2 SP Parchment — 1 SP Perfume — 5 GP Poison, Basic — 100 GP Pole
    7 lb. 5 CP Pot, Iron 10 lb. 2 GP Potion of Healing 1/2 lb. 50 GP Pouch 1 lb. 5 SP Priest’s Pack 29
    lb. 33 GP Quiver 1 lb. 1 GP Ram, Portable 35 lb. 4 GP Rations 2 lb. 5 SP Robe 4 lb. 1 GP Rope 5 lb.
    1 GP Sack 1/2 lb. 1 CP Scholar’s Pack 22 lb. 40 GP Shovel 5 lb. 2 GP Signal Whistle — 5 CP Spell Scroll
    (Cantrip) — 30 GP Spell Scroll (Level 1) — 50 GP Spikes, Iron 5 lb. 1 GP Spyglass 1 lb. 1,000 GP String
    — 1 SP Tent 20 lb. 2 GP Tinderbox 1 lb. 5 SP Torch 1 lb. 1 CP Vial — 1 GP Waterskin 5 lb. (full) 2
    SP Ammunition Type Amount Storage Weight Cost Arrows 20 Quiver 1 lb. 1 GP Bolts 20 Case 1½ lb. 1 GP
    Bullets, Firearm 10 Pouch 2 lb. 3 GP Bullets, Sling 20 Pouch 1½ lb. 4 CP Needles 50 Pouch 1 lb. 1
    GP'
  sha256: a64bcb3a89d72b484fdfb62f63621c7b4ed32539e42a89d793db0c1a79e23a68
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ammunition (Varies)

<!-- SRD page 94 -->

Ammunition is required by a weapon that has the\
Ammunition property. A weapon’s description spec-\
ifies the type of ammunition used by the weapon.\
The Ammunition table lists the different types and\
the amount you get when you buy them. The table\
also lists the item that is typically used to store\
each type; storage must be bought separately.\

<!-- SRD page 95 -->

Adventuring Gear\
Item Weight Cost\
Acid 1 lb. 25 GP\
Alchemist’s Fire 1 lb. 50 GP\
Ammunition Varies Varies\
Antitoxin — 50 GP\
Arcane Focus Varies Varies\
Backpack 5 lb. 2 GP\
Ball Bearings 2 lb. 1 GP\
Barrel 70 lb. 2 GP\
Basket 2 lb. 4 SP\
Bedroll 7 lb. 1 GP\
Bell — 1 GP\
Blanket 3 lb. 5 SP\
Block and Tackle 5 lb. 1 GP\
Book 5 lb. 25 GP\
Bottle, Glass 2 lb. 2 GP\
Bucket 2 lb. 5 CP\
Burglar’s Pack 42 lb. 16 GP\
Caltrops 2 lb. 1 GP\
Candle — 1 CP\
Case, Crossbow Bolt 1 lb. 1 GP\
Case, Map or Scroll 1 lb. 1 GP\
Chain 10 lb. 5 GP\
Chest 25 lb. 5 GP\
Climber’s Kit 12 lb. 25 GP\
Clothes, Fine 6 lb. 15 GP\
Clothes, Traveler’s 4 lb. 2 GP\
Component Pouch 2 lb. 25 GP\
Costume 4 lb. 5 GP\
Crowbar 5 lb. 2 GP\
Diplomat’s Pack 39 lb. 39 GP\
Druidic Focus Varies Varies\
Dungeoneer’s Pack 55 lb. 12 GP\
Entertainer’s Pack 58½ lb. 40 GP\
Explorer’s Pack 55 lb. 10 GP\
Flask 1 lb. 2 CP\
Grappling Hook 4 lb. 2 GP\
Healer’s Kit 3 lb. 5 GP\
Holy Symbol Varies Varies\
Holy Water 1 lb. 25 GP\
Hunting Trap 25 lb. 5 GP\
Ink — 10 GP\
Item Weight Cost\
Ink Pen — 2 CP\
Jug 4 lb. 2 CP\
Ladder 25 lb. 1 SP\
Lamp 1 lb. 5 SP\
Lantern, Bullseye 2 lb. 10 GP\
Lantern, Hooded 2 lb. 5 GP\
Lock 1 lb. 10 GP\
Magnifying Glass — 100 GP\
Manacles 6 lb. 2 GP\
Map — 1 GP\
Mirror 1/2 lb. 5 GP\
Net 3 lb. 1 GP\
Oil 1 lb. 1 SP\
Paper — 2 SP\
Parchment — 1 SP\
Perfume — 5 GP\
Poison, Basic — 100 GP\
Pole 7 lb. 5 CP\
Pot, Iron 10 lb. 2 GP\
Potion of Healing 1/2 lb. 50 GP\
Pouch 1 lb. 5 SP\
Priest’s Pack 29 lb. 33 GP\
Quiver 1 lb. 1 GP\
Ram, Portable 35 lb. 4 GP\
Rations 2 lb. 5 SP\
Robe 4 lb. 1 GP\
Rope 5 lb. 1 GP\
Sack 1/2 lb. 1 CP\
Scholar’s Pack 22 lb. 40 GP\
Shovel 5 lb. 2 GP\
Signal Whistle — 5 CP\
Spell Scroll (Cantrip) — 30 GP\
Spell Scroll (Level 1) — 50 GP\
Spikes, Iron 5 lb. 1 GP\
Spyglass 1 lb. 1,000 GP\
String — 1 SP\
Tent 20 lb. 2 GP\
Tinderbox 1 lb. 5 SP\
Torch 1 lb. 1 CP\
Vial — 1 GP\
Waterskin 5 lb. (full) 2 SP\

<!-- SRD page 96 -->

Ammunition\
Type Amount Storage Weight Cost\
Arrows 20 Quiver 1 lb. 1 GP\
Bolts 20 Case 1½ lb. 1 GP\
Bullets, Firearm 10 Pouch 2 lb. 3 GP\
Bullets, Sling 20 Pouch 1½ lb. 4 CP\
Needles 50 Pouch 1 lb. 1 GP\
