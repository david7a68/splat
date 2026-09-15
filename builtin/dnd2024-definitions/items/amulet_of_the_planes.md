---
splat_schema: 1
splat_id: dcf124f1-da50-58b9-a70f-54dd63b8302d
splat_kind: item
splat_key: dnd2024:items/amulet_of_the_planes
splat_name: Amulet of the Planes
splat_origin:
  key: dnd2024:reference/magic_item/amulet_of_the_planes
splat_sources:
- title: SRD 5.2.1
  locator: pp. 209
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/amulet_of_the_planes
  implementation: dnd2024:item_amulet_of_the_planes
  parameters:
    equipped: worn
    action: magic
    destination:
      type: location
      familiar: true
      plane: other_than_current
    check:
      ability: intelligence
      skill: arcana
      dc: 15
      success:
        spell:
          key: dnd2024:spells/plane_shift
      failure:
        transport:
        - actor
        - creatures
        - objects
        radius_ft: 15
        destination_roll:
          die: 100
          rows:
          - range:
            - 1
            - 60
            plane: named_plane
            location: random
          - range:
            - 61
            - 70
            plane_roll:
              die: 6
              values:
              - air
              - earth
              - fire
              - water
              - feywild
              - shadowfell
            location: random
          - range:
            - 71
            - 80
            plane_roll:
              die: 8
              values:
              - arborea
              - arcadia
              - beastlands
              - bytopia
              - elysium
              - mechanus
              - mount_celestia
              - ysgard
            location: random
          - range:
            - 81
            - 90
            plane_roll:
              die: 8
              values:
              - abyss
              - acheron
              - carceri
              - gehenna
              - hades
              - limbo
              - nine_hells
              - pandemonium
            location: random
          - range:
            - 91
            - 100
            plane: astral
            location: random
    attunement_required: true
splat_contracts:
- id: dnd2024:item_amulet_of_the_planes
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
  - item.activation_requested
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - item_spellcasting
  - planar_travel
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 7cd7eb0bca097a46beb9d0a3ef5a20a8312eebbc18d1810aa0a4cf2fe2bc5ec8
  source_path: builtin/srd-5.2.1/magic_item/amulet_of_the_planes.md
  pages:
  - 209
  engine_gaps: &id001
  - dnd2024:item_amulet_of_the_planes
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 647b159d8aa23f8ef47e9c4b014ea4a378bfbff83ba7d1494f08c4385a5b246c
splat_clauses:
- key: source_rules
  text: '# Amulet of the Planes Wondrous Item, Very Rare (Requires Attunement) While wearing this amulet,
    you can take a Magic action to name a location that you are familiar with on another plane of existence.
    Then make a DC 15 Intelligence (Arcana) check. On a successful check, you cast Plane Shift. On a failed
    check, you and each creature and object within 15 feet of you travel to a random destination determined
    by rolling 1d100 and consulting the following table. 1d100 Destination 01–60 Random location on the
    plane you named 61–70 Random location on an Inner Plane determined by rolling 1d6: on a 1, the Plane
    of Air; on a 2, the Plane of Earth; on a 3, the Plane of Fire; on a 4, the Plane of Water; on a 5,
    the Feywild; on a 6, the Shadowfell 71–80 Random location on an Outer Plane determined by rolling
    1d8: on a 1, Arborea; on a 2, Arcadia; on a 3, the Beastlands; on a 4, Bytopia; on a 5, Elysium; on
    a 6, Mechanus; on a 7, Mount Celestia; on an 8, Ysgard 81–90 Random location on an Outer Plane determined
    by rolling 1d8: on a 1, the Abyss; on a 2, Acheron; on a 3, Carceri; on a 4, Gehenna; on a 5, Hades;
    on a 6, Limbo; on a 7, the Nine Hells; on an 8, Pandemonium 91–00 Random location on the Astral Plane'
  sha256: 5a10e2750d6ebf8d4d3a6b01a7066f6a9d089174bc310de3727400579502e708
  status: contract_specified
splat_requires_mechanics: *id001
---

# Amulet of the Planes

<!-- SRD page 209 -->

Wondrous Item, Very Rare (Requires Attunement)\
While wearing this amulet, you can take a Magic\
action to name a location that you are familiar with\
on another plane of existence. Then make a DC 15\
Intelligence (Arcana) check. On a successful check,\
you cast Plane Shift. On a failed check, you and each\
creature and object within 15 feet of you travel to\
a random destination determined by rolling 1d100\
and consulting the following table.\
1d100 Destination\
01–60 Random location on the plane you named\
61–70 Random location on an Inner Plane deter-\
mined by rolling 1d6: on a 1, the Plane of Air;\
on a 2, the Plane of Earth; on a 3, the Plane of\
Fire; on a 4, the Plane of Water; on a 5, the\
Feywild; on a 6, the Shadowfell\
71–80 Random location on an Outer Plane deter-\
mined by rolling 1d8: on a 1, Arborea; on a 2,\
Arcadia; on a 3, the Beastlands; on a 4, Byto-\
pia; on a 5, Elysium; on a 6, Mechanus; on a 7,\
Mount Celestia; on an 8, Ysgard\
81–90 Random location on an Outer Plane deter-\
mined by rolling 1d8: on a 1, the Abyss; on a\
2, Acheron; on a 3, Carceri; on a 4, Gehenna;\
on a 5, Hades; on a 6, Limbo; on a 7, the Nine\
Hells; on an 8, Pandemonium\
91–00 Random location on the Astral Plane\

Original layout: [[../../srd-5.2.1/pages/209|page 209]].
