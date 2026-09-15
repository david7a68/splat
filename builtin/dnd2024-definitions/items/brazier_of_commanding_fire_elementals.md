---
splat_schema: 1
splat_id: e3d615d2-9240-5f55-9ac9-75f993be59c0
splat_kind: item
splat_key: dnd2024:items/brazier_of_commanding_fire_elementals
splat_name: Brazier of Commanding Fire Elementals
splat_origin:
  key: dnd2024:reference/magic_item/brazier_of_commanding_fire_elementals
splat_sources:
- title: SRD 5.2.1
  locator: pp. 214
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/brazier_of_commanding_fire_elementals
  implementation: dnd2024:item_brazier_of_commanding_fire_elementals
  parameters:
    activation:
      action: magic
      distance_ft: 5
      resource:
        uses: 1
        recovery: dawn
      summon:
        creature:
          key: dnd2024:creatures/fire_elemental
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
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_brazier_of_commanding_fire_elementals
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
  source_sha256: b81df613112bbf53b80af1ac599d120b484773ccbbeef88fbccedf2a1ae434a4
  source_path: builtin/srd-5.2.1/magic_item/brazier_of_commanding_fire_elementals.md
  pages:
  - 214
  engine_gaps: &id001
  - dnd2024:item_brazier_of_commanding_fire_elementals
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 4f051becb19f2fe196767fa6f6a97a79e861f941842c0b3346246f1161c538b0
splat_clauses:
- key: source_rules
  text: '# Brazier of Commanding Fire Elementals Wondrous Item, Rare While you are within 5 feet of this
    brazier, you can take a Magic action to summon a Fire Elemental. The elemental appears in an unoccupied
    space as close to the brazier as possible, understands your languages, obeys your commands, and takes
    its turn immediately after you on your Initiative count. The elemental disappears after 1 hour, when
    it dies, or when you dismiss it as a Bonus Action. The brazier can’t be used this way again until
    the next dawn.'
  sha256: 5465cde923d62cfebc1ec476802da0016b49a2334b835fdd2130a3e3dd07f4d2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Brazier of Commanding Fire Elementals

<!-- SRD page 214 -->

Wondrous Item, Rare\
While you are within 5 feet of this brazier, you can\
take a Magic action to summon a Fire Elemental.\
The elemental appears in an unoccupied space as\
close to the brazier as possible, understands your\
languages, obeys your commands, and takes its turn\
immediately after you on your Initiative count. The\
elemental disappears after 1 hour, when it dies, or\
when you dismiss it as a Bonus Action. The brazier\
can’t be used this way again until the next dawn.\

Original layout: [[../../srd-5.2.1/pages/214|page 214]].
