---
splat_schema: 1
splat_id: 289ed0e7-e108-5dd9-912f-bbf5385bdee9
splat_kind: item
splat_key: dnd2024:items/necklace_of_fireballs
splat_name: Necklace of Fireballs
splat_origin:
  key: dnd2024:reference/magic_item/necklace_of_fireballs
splat_sources:
- title: SRD 5.2.1
  locator: pp. 233
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/necklace_of_fireballs
  implementation: dnd2024:item_necklace_of_fireballs
  parameters:
    beads:
      initial:
        expression: 1d6+3
        scope: use
    activate:
      action: magic
      throw_maximum_ft: 60
      beads_expended:
        minimum: 1
        maximum: remaining
        whole_necklace_allowed: true
      detonate_at: trajectory_end
      spell:
        key: dnd2024:spells/fireball
      spell_level: 3
      save_dc: 15
      damage_override:
        dice:
          count:
            op: min
            args:
            - 12
            - op: add
              args:
              - 8
              - op: subtract
                args:
                - binding: beads_expended
                - 1
          sides: 6
          add: 0
        type: fire
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_necklace_of_fireballs
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
    beads_expended:
      required: false
      type: integer
  outputs:
  - item_state
  - effects
  - resolutions
  events:
  - action.magic_pending
  capabilities:
  - inventory
  - effects
  - spellcasting
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: cd46b4ef2e06d0afa3f6cb8e320b5f31e41562bf907d563ee9c245b91898d811
  source_path: builtin/srd-5.2.1/magic_item/necklace_of_fireballs.md
  pages:
  - 233
  engine_gaps: &id001
  - dnd2024:item_necklace_of_fireballs
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 7eca7124460e390eb610fdfc1386a2e3b54c3a35c0f8df54ab2b40cbcd5da18a
splat_clauses:
- key: source_rules
  text: '# Necklace of Fireballs Wondrous Item, Rare This necklace has 1d6 + 3 beads hanging from it.
    You can take a Magic action to detach a bead and throw it up to 60 feet away. When it reaches the
    end of its trajectory, the bead detonates as a level 3 Fireball (save DC 15). You can hurl multiple
    beads, or even the whole necklace, at one time. When you do so, increase the damage of the Fireball
    by 1d6 for each bead after the first (maximum 12d6).'
  sha256: 274db40349e81700a7d86bce476eb4acae735bd0adbe84971dbd2d23cb22edc6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Necklace of Fireballs

<!-- SRD page 233 -->

Wondrous Item, Rare\
This necklace has 1d6 + 3 beads hanging from it.\
You can take a Magic action to detach a bead and\
throw it up to 60 feet away. When it reaches the end\
of its trajectory, the bead detonates as a level 3 Fire-\
ball (save DC 15).\
You can hurl multiple beads, or even the whole\
necklace, at one time. When you do so, increase the\
damage of the Fireball by 1d6 for each bead after\
the first (maximum 12d6).\

Original layout: [[../../srd-5.2.1/pages/233|page 233]].
