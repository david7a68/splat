---
splat_schema: 1
splat_id: 9f360395-d296-5962-a00e-3baef5a65710
splat_kind: item
splat_key: dnd2024:items/bag_of_devouring
splat_name: Bag of Devouring
splat_origin:
  key: dnd2024:reference/magic_item/bag_of_devouring
splat_sources:
- title: SRD 5.2.1
  locator: pp. 212
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: very_rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/bag_of_devouring
  implementation: dnd2024:item_bag_of_devouring
  parameters:
    appearance:
      key: dnd2024:items/bag_of_holding
    orifice:
      inside_out: closed
      senses_contents: true
    organic_contents:
      types:
      - animal_matter
      - vegetable_matter
      wholly_inserted: devoured_permanently
    partial_living_insertion:
      pull_inside_probability:
      - 1
      - 2
    escape:
      action: action
      check:
        ability: strength
        skill: athletics
        dc: 15
    rescue:
      action: action
      reach_inside_triggers_pull: true
      check_only_if_rescuer_not_pulled: true
      check:
        ability: strength
        skill: athletics
        dc: 20
    turn_start_inside:
      devoured: true
      body_destroyed: true
    storage:
      inanimate_volume_cubic_ft: 1
      daily_ejection:
        frequency_per_day: 1
        time_choice: gm
        plane_choice: gm
        contents: all_inanimate_objects
    on_damage_kind:
      pierced_or_torn:
        destroy_item: true
        transport_contents:
          plane: astral
          location: random
    attunement_required: false
splat_contracts:
- id: dnd2024:item_bag_of_devouring
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
  - item.contents_changed
  - item.turned_inside_out
  - turn.started
  - item.damaged
  - time.day_changed
  - item.activation_requested
  capabilities:
  - item_effects
  - planar_travel
  - inventory_containers
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 6016468eddc7b7eaa91206f9668602937562412a1a1c4d6eb87e41844821c515
  source_path: builtin/srd-5.2.1/magic_item/bag_of_devouring.md
  pages:
  - 212
  engine_gaps: &id001
  - dnd2024:item_bag_of_devouring
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 78ec0177a8a6ef2e013667e318a4d9c0c32464176f4ec8cea18d519b1fbea85c
splat_clauses:
- key: source_rules
  text: '# Bag of Devouring Wondrous Item, Very Rare This bag resembles a Bag of Holding but is a feeding
    orifice for a gigantic extradimensional creature. Turning the bag inside out closes the orifice. The
    extradimensional creature attached to the bag can sense whatever is placed inside the bag. Animal
    or vegetable matter placed wholly in the bag is devoured and lost forever. When part of a living creature
    is placed in the bag, as happens when someone reaches inside it, there is a 50 percent chance that
    the creature is pulled inside the bag. A creature inside the bag can take an action to try to escape,
    doing so with a successful DC 15 Strength (Athletics) check. Another creature can take an action to
    reach into the bag to pull a creature out, doing so with a successful DC 20 Strength (Athletics) check,
    provided the puller isn’t pulled inside the bag first. Any creature that starts its turn inside the
    bag is devoured, its body destroyed. Inanimate objects can be stored in the bag, which can hold a
    cubic foot of such material. However, once each day, the bag swallows any objects inside it and spits
    them out into another plane of existence. The GM determines the time and plane. If the bag is pierced
    or torn, it is destroyed, and anything contained within it is transported to a random location on
    the Astral Plane.'
  sha256: 198cf26aa477fa23298aa582c642addbec95b1fce8bc025a512b8856ca72f60f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bag of Devouring

<!-- SRD page 212 -->

Wondrous Item, Very Rare\
This bag resembles a Bag of Holding but is a feeding\
orifice for a gigantic extradimensional creature.\
Turning the bag inside out closes the orifice.\
The extradimensional creature attached to the\
bag can sense whatever is placed inside the bag.\
Animal or vegetable matter placed wholly in the\
bag is devoured and lost forever. When part of a liv-\
ing creature is placed in the bag, as happens when\
someone reaches inside it, there is a 50 percent\
chance that the creature is pulled inside the bag. A\
creature inside the bag can take an action to try to\
escape, doing so with a successful DC 15 Strength\
(Athletics) check. Another creature can take an ac-\
tion to reach into the bag to pull a creature out, do-\
ing so with a successful DC 20 Strength (Athletics)\
check, provided the puller isn’t pulled inside the bag\
first. Any creature that starts its turn inside the bag\
is devoured, its body destroyed.\
Inanimate objects can be stored in the bag, which\
can hold a cubic foot of such material. However,\
once each day, the bag swallows any objects inside\
it and spits them out into another plane of existence.\
The GM determines the time and plane.\
If the bag is pierced or torn, it is destroyed, and\
anything contained within it is transported to a\
random location on the Astral Plane.\

Original layout: [[../../srd-5.2.1/pages/212|page 212]].
