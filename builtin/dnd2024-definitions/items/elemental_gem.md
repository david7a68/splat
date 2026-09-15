---
splat_schema: 1
splat_id: 401d65af-0bd4-543e-8af7-905173350417
splat_kind: item
splat_key: dnd2024:items/elemental_gem
splat_name: Elemental Gem
splat_origin:
  key: dnd2024:reference/magic_item/elemental_gem
splat_sources:
- title: SRD 5.2.1
  locator: pp. 220
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/elemental_gem
  implementation: dnd2024:item_elemental_gem
  parameters:
    activation:
      action: utilize
      operation: break_gem
      item_magic_ends: true
      summon:
        creature:
          value_from: variant.creature
        placement:
          unoccupied: true
          selection: nearest_item
        understands: summoner_languages
        commands: obey
        initiative: immediately_after_summoner_same_count
        duration:
          value: 1
          unit: hour
        end_on:
        - death
        - summoner_bonus_action_dismissal
    variants:
      blue_sapphire:
        creature:
          key: dnd2024:creatures/air_elemental
      emerald:
        creature:
          key: dnd2024:creatures/water_elemental
      red_corundum:
        creature:
          key: dnd2024:creatures/fire_elemental
      yellow_diamond:
        creature:
          key: dnd2024:creatures/earth_elemental
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_elemental_gem
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
    locations:
      required: false
      type: location_list
  outputs:
  - summoned_creatures
  - item_state
  events:
  - item.used
  - turn.started
  - creature.died
  capabilities:
  - summoning
  - initiative
  - item_resources
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 9990fb110720cfa26fc5e80e6e6ad5d96c5e0d6901b0dfc45988bfed4ec39b59
  source_path: builtin/srd-5.2.1/magic_item/elemental_gem.md
  pages:
  - 220
  engine_gaps: &id001
  - dnd2024:item_elemental_gem
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 63ff0de5bf7fe0b0f581a39ca8591ef1b2fc2137804834ba60f9086fd76004c6
splat_clauses:
- key: source_rules
  text: '# Elemental Gem Wondrous Item, Uncommon This gem contains a mote of elemental energy. When you
    take a Utilize action to break the gem, an elemental is summoned (see “Monsters” for its stat block),
    and the gem ceases to be magical. The elemental appears in an unoccupied space as close to the broken
    gem as possible, understands your languages, obeys your commands, and takes its turn immediately after
    you on your Initiative count. The elemental disappears after 1 hour, when it dies, or when you dismiss
    it as a Bonus Action. The type of gem determines the elemental, as shown in the following table. Gem
    Summoned Elemental Blue sapphire Air Elemental Emerald Water Elemental Red corundum Fire Elemental
    Yellow diamond Earth Elemental'
  sha256: b1f7857445833cbc756b707b03ce24949dc2e3b981db5afb0f42db55df4c912e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Elemental Gem

<!-- SRD page 220 -->

Wondrous Item, Uncommon\
This gem contains a mote of elemental energy.\
When you take a Utilize action to break the gem,\
an elemental is summoned (see “Monsters” for its\
stat block), and the gem ceases to be magical. The\
elemental appears in an unoccupied space as close\
to the broken gem as possible, understands your\
languages, obeys your commands, and takes its\
turn immediately after you on your Initiative count.\
The elemental disappears after 1 hour, when it dies,\
or when you dismiss it as a Bonus Action. The type\
of gem determines the elemental, as shown in the\
following table.\
Gem Summoned Elemental\
Blue sapphire Air Elemental\
Emerald Water Elemental\
Red corundum Fire Elemental\
Yellow diamond Earth Elemental\

Original layout: [[../../srd-5.2.1/pages/220|page 220]].
