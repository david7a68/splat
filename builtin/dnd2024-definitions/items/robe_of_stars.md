---
splat_schema: 1
splat_id: 6e0e1b04-7b5d-545d-ba2b-5be1adbcc269
splat_kind: item
splat_key: dnd2024:items/robe_of_stars
splat_name: Robe of Stars
splat_origin:
  key: dnd2024:reference/magic_item/robe_of_stars
splat_sources:
- title: SRD 5.2.1
  locator: pp. 240
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/robe_of_stars
  implementation: dnd2024:item_robe_of_stars
  parameters:
    equipment_state: worn
    saving_throw_bonus: 1
    stars:
      maximum: 6
      initial: 6
      recovery:
        event: dusk
        amount:
          expression: 1d6
          scope: use
        cap: maximum
    spell_grants:
    - spell:
        key: dnd2024:spells/magic_missile
      charge_cost: 1
      casting_time: magic
      method: item
      slot_cost: 0
      components: []
      level: 5
      range: spell_default
      duration: spell_default
      concentration: spell_default
      activation: remove_and_expend_star
    astral_travel:
      depart_action: magic
      include: all_worn_and_carried
      destination_plane: astral
      duration: until_return_action
      return_action: magic
      return_plane: prior_plane
      return_location: last_occupied_space
      if_occupied: nearest_unoccupied_space
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_robe_of_stars
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
  - action.magic_pending
  - time.dusk
  - saving_throw.pending
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - spellcasting
  - resources_and_time
  - spatial_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 1f7d097a80034c4a5d8bba86025a044dd0121884f6a2ff1749682328f5dffdaa
  source_path: builtin/srd-5.2.1/magic_item/robe_of_stars.md
  pages:
  - 240
  engine_gaps: &id001
  - dnd2024:item_robe_of_stars
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: cdf3bfa982d811e88364a54637ba024927425d190e75f5d300b0ea794d0df2a4
splat_clauses:
- key: source_rules
  text: '# Robe of Stars Wondrous Item, Very Rare (Requires Attunement) This black or dark-blue robe is
    embroidered with small white or silver stars. You gain a +1 bonus to saving throws while you wear
    it. Six stars, located on the robe’s upper-front portion, are particularly large. While wearing this
    robe, you can take a Magic action to remove one of the stars and expend it to cast the level 5 version
    of Magic Missile. Daily at dusk, 1d6 removed stars reappear on the robe. While you wear the robe,
    you can take a Magic action to enter the Astral Plane along with everything you are wearing and carrying.
    You remain there until you take a Magic action to return to the plane you were on. You reappear in
    the last space you occupied or, if that space is occupied, the nearest unoccupied space.'
  sha256: 7d0dfe558dc4921904ea4587a48880e788ac57a902832b173db29cf91882d645
  status: contract_specified
splat_requires_mechanics: *id001
---

# Robe of Stars

<!-- SRD page 240 -->

Wondrous Item, Very Rare (Requires Attunement)\
This black or dark-blue robe is embroidered with\
small white or silver stars. You gain a +1 bonus to\
saving throws while you wear it.\
Six stars, located on the robe’s upper-front por-\
tion, are particularly large. While wearing this\
robe, you can take a Magic action to remove one of\
the stars and expend it to cast the level 5 version of\
Magic Missile. Daily at dusk, 1d6 removed stars re-\
appear on the robe.\
While you wear the robe, you can take a Magic ac-\
tion to enter the Astral Plane along with everything\
you are wearing and carrying. You remain there\
until you take a Magic action to return to the plane\
you were on. You reappear in the last space you\
occupied or, if that space is occupied, the nearest\
unoccupied space.\

Original layout: [[../../srd-5.2.1/pages/240|page 240]].
