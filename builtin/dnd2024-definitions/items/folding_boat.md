---
splat_schema: 1
splat_id: 840f9425-7fe7-5ec3-be15-31386b8e8a32
splat_kind: item
splat_key: dnd2024:items/folding_boat
splat_name: Folding Boat
splat_origin:
  key: dnd2024:reference/magic_item/folding_boat
splat_sources:
- title: SRD 5.2.1
  locator: pp. 223
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/folding_boat
  implementation: dnd2024:item_folding_boat
  parameters:
    box:
      dimensions_inches:
      - 12
      - 6
      - 6
      weight_lb: 4
      floats: true
      openable_storage: true
    transformation:
      action: magic
      command_words:
        first: rowboat
        second: keelboat
        third: box
      box_requires_no_creatures_aboard: true
      objects_on_folding:
        fits_box: inside_box
        does_not_fit_box: outside_box
      objects_on_unfolding: remain_inside_vessel
      vessel_weight: normal_vessel_weight
      vessel_statistics:
        rowboat:
          ac: 11
          hp: 50
          speed_mph: 1.5
          crew: 1
          passengers: 3
          cargo_tons: 0
          damage_threshold: 0
          weight_lb: 100
        keelboat:
          ac: 15
          hp: 100
          speed_mph: 1
          crew: 1
          passengers: 6
          cargo_tons: 0.5
          damage_threshold: 10
      zero_vessel_hp: item_destroyed
      vehicle_rules:
      - key: dnd2024:rules/mounts_and_vehicles_large_vehicles
      - key: dnd2024:rules/mounts_and_vehicles_speed
      - key: dnd2024:rules/mounts_and_vehicles_crew
    attunement_required: false
splat_contracts:
- id: dnd2024:item_folding_boat
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
  - item.hp_changed
  capabilities:
  - item_effects
  - inventory_containers
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 7bcb2d6d489781aea838a53cf6b455c64d524a7e0080dc3ef367242571320074
  source_path: builtin/srd-5.2.1/magic_item/folding_boat.md
  pages:
  - 223
  engine_gaps: &id001
  - dnd2024:item_folding_boat
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 4b2ac636f4193cbce076ee79726c3e20f4095c4d693397cec7d1f4e416e2e06b
splat_clauses:
- key: source_rules
  text: '# Folding Boat Wondrous Item, Rare This object appears as a wooden box that measures 12 inches
    long, 6 inches wide, and 6 inches deep. It weighs 4 pounds and floats. It can be opened to store items
    inside. This item also has three command words, each requiring a Magic action to use: First Command
    Word. The box unfolds into a Rowboat. Second Command Word. The box unfolds into a Keelboat. Third
    Command Word. The Folding Boat folds back into a box if no creatures are aboard. Any objects in the
    vessel that can’t fit inside the box remain outside the box as it folds. Any objects in the vessel
    that can fit inside the box do so. When the box becomes a vessel, its weight becomes that of a normal
    vessel its size, and anything that was stored in the box remains in the boat. Statistics for the Rowboat
    and Keelboat appear in “Equipment.” If either vessel is reduced to 0 Hit Points, the Folding Boat
    is destroyed.'
  sha256: 1c85efd8d1e426af884a91023defda566668fb3f96f97562747c5cfc4f31af75
  status: contract_specified
splat_requires_mechanics: *id001
---

# Folding Boat

<!-- SRD page 223 -->

Wondrous Item, Rare\
This object appears as a wooden box that measures\
12 inches long, 6 inches wide, and 6 inches deep.\
It weighs 4 pounds and floats. It can be opened to\
store items inside. This item also has three com-\
mand words, each requiring a Magic action to use:\
First Command Word. The box unfolds into a\
Rowboat.\
Second Command Word. The box unfolds into a\
Keelboat.\
Third Command Word. The Folding Boat folds back\
into a box if no creatures are aboard. Any objects\
in the vessel that can’t fit inside the box remain\
outside the box as it folds. Any objects in the ves-\
sel that can fit inside the box do so.\
When the box becomes a vessel, its weight becomes\
that of a normal vessel its size, and anything that\
was stored in the box remains in the boat.\
Statistics for the Rowboat and Keelboat appear\
in “Equipment.” If either vessel is reduced to 0 Hit\
Points, the Folding Boat is destroyed.\

Original layout: [[../../srd-5.2.1/pages/223|page 223]].
