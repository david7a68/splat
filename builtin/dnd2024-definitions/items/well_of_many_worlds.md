---
splat_schema: 1
splat_id: 06b372a2-f70b-5b3c-adbf-af31ca9529bf
splat_kind: item
splat_key: dnd2024:items/well_of_many_worlds
splat_name: Well of Many Worlds
splat_origin:
  key: dnd2024:reference/magic_item/well_of_many_worlds
splat_sources:
- title: SRD 5.2.1
  locator: pp. 253
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: legendary
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/well_of_many_worlds
  implementation: dnd2024:item_well_of_many_worlds
  parameters:
    folded_size: handkerchief
    unfolded:
      shape: circle
      diameter_ft: 6
    open:
      action: magic
      unfold_and_place_on: solid_surface
      portal:
        two_way: true
        diameter_ft: 6
        shape: circle
        destination:
          choice: gm_each_opening
          options:
          - another_world
          - another_plane
    close:
      action: magic
      actor: any_creature
      maximum_range_ft: 5
      take_edges_and_fold: true
    cooldown:
      starts: portal_opened
      duration:
        amount:
          roll:
            count: 1
            sides: 8
        unit: hours
    attunement_required: false
splat_contracts:
- id: dnd2024:item_well_of_many_worlds
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
  - time.elapsed
  capabilities:
  - item_effects
  - dice
  - effect_duration
  - item_recharge
  - spatial_targeting
  - planar_travel
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 28f26bab03510cc958508a8f3f4021ba37e4d518120afd375d2906cab47b3cb0
  source_path: builtin/srd-5.2.1/magic_item/well_of_many_worlds.md
  pages:
  - 253
  engine_gaps: &id001
  - dnd2024:item_well_of_many_worlds
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 068fc60e0ec27d65ee57ad4e001d9dfa5d3c92dd873283a7c852bd5614520025
splat_clauses:
- key: source_rules
  text: '# Well of Many Worlds Wondrous Item, Legendary This fine black cloth, soft as silk, is folded
    up to the dimensions of a handkerchief. It unfolds into a circular sheet 6 feet in diameter. You can
    take a Magic action to unfold the Well of Many Worlds and place it on a solid surface, whereupon it
    forms a two-way, 6-foot-diameter, circular portal to another world or plane of existence. Each time
    the item opens a portal, the GM decides where it leads. The portal remains open until a creature within
    5 feet of it takes a Magic action to close it by taking hold of the edges of the cloth and folding
    it up. Once the Well of Many Worlds has opened a portal, it can’t do so again for 1d8 hours.'
  sha256: c2594b2a52bbe754654b15b3967ce660f0a7bcd040c6678f3a79f27588db8736
  status: contract_specified
splat_requires_mechanics: *id001
---

# Well of Many Worlds

<!-- SRD page 253 -->

Wondrous Item, Legendary\
This fine black cloth, soft as silk, is folded up to the\
dimensions of a handkerchief. It unfolds into a cir-\
cular sheet 6 feet in diameter.\
You can take a Magic action to unfold the Well of\
Many Worlds and place it on a solid surface, where-\
upon it forms a two-way, 6-foot-diameter, circular\
portal to another world or plane of existence. Each\
time the item opens a portal, the GM decides where\
it leads. The portal remains open until a creature\
within 5 feet of it takes a Magic action to close it\
by taking hold of the edges of the cloth and folding\
it up.\
Once the Well of Many Worlds has opened a portal,\
it can’t do so again for 1d8 hours.\

Original layout: [[../../srd-5.2.1/pages/253|page 253]].
