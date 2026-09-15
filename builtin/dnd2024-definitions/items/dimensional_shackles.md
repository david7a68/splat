---
splat_schema: 1
splat_id: a7e4f139-6804-548c-bcd7-a863aa5b3776
splat_kind: item
splat_key: dnd2024:items/dimensional_shackles
splat_name: Dimensional Shackles
splat_origin:
  key: dnd2024:reference/magic_item/dimensional_shackles
splat_sources:
- title: SRD 5.2.1
  locator: pp. 218
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/dimensional_shackles
  implementation: dnd2024:item_dimensional_shackles
  parameters:
    apply:
      action: utilize
      target_condition: incapacitated
      sizes:
      - small
      - medium
      - large
      designate_removers: true
    bound:
      prevent_movement:
      - teleportation
      - planar_travel
      - other_extradimensional
      allow: passing_through_interdimensional_portal
    remove:
      action: utilize
      allowed_actors:
      - applier
      - designated_creatures
    break_free:
      interval:
        amount: 30
        unit: day
      actor: bound_creature
      check:
        ability: strength
        skill: athletics
        dc: 30
      success:
        freed: true
        item_destroyed: true
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_dimensional_shackles
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
  - action.utilize_pending
  - check.pending
  - time.day
  - movement.planar_pending
  capabilities:
  - inventory
  - effects
  - roll_resolution
  - spatial_state
  - participant_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 014ad8ae1aa4933aafd49fbfd61ebbb97189346cbc88a963136d66a288de2ec5
  source_path: builtin/srd-5.2.1/magic_item/dimensional_shackles.md
  pages:
  - 218
  engine_gaps: &id001
  - dnd2024:item_dimensional_shackles
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: ae38953cc54fea565f80d97d5c650d98de957bc76e68fa1e17379b15aa3e5a8a
splat_clauses:
- key: source_rules
  text: '# Dimensional Shackles Wondrous Item, Rare You can take a Utilize action to place these shackles
    on a creature that has the Incapacitated condition. The shackles adjust to fit a creature of Small
    to Large size. The shackles prevent a creature bound by them from using any method of extradimensional
    movement, including teleportation or travel to a different plane of existence. They don’t prevent
    the creature from passing through an interdimensional portal. You and any creature you designate when
    you use the shackles can take a Utilize action to remove them. Once every 30 days, the bound creature
    can make a DC 30 Strength (Athletics) check. On a successful check, the creature breaks free and destroys
    the shackles.'
  sha256: 80018defe2a62bc03fac848b4bdaa6ee58fb7b3dbeada181b5c6e0530b6501b5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dimensional Shackles

<!-- SRD page 218 -->

Wondrous Item, Rare\
You can take a Utilize action to place these shackles\
on a creature that has the Incapacitated condition.\
The shackles adjust to fit a creature of Small to\
Large size. The shackles prevent a creature bound\
by them from using any method of extradimen-\
sional movement, including teleportation or travel\
to a different plane of existence. They don’t prevent\
the creature from passing through an interdimen-\
sional portal.\
You and any creature you designate when you\
use the shackles can take a Utilize action to remove\
them. Once every 30 days, the bound creature can\
make a DC 30 Strength (Athletics) check. On a suc-\
cessful check, the creature breaks free and destroys\
the shackles.\

Original layout: [[../../srd-5.2.1/pages/218|page 218]].
