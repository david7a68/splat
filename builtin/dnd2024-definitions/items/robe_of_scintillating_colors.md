---
splat_schema: 1
splat_id: bfb6e27f-62a6-5c0a-a5d3-6a8fee8c2009
splat_kind: item
splat_key: dnd2024:items/robe_of_scintillating_colors
splat_name: Robe of Scintillating Colors
splat_origin:
  key: dnd2024:reference/magic_item/robe_of_scintillating_colors
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
  mechanic: dnd2024:items/robe_of_scintillating_colors
  implementation: dnd2024:item_robe_of_scintillating_colors
  parameters:
    equipment_state: worn
    charges:
      maximum: 3
      initial: 3
      recovery:
        event: dawn
        amount:
          dice: 1
          sides: 3
          bonus: 0
    activation:
      action: magic
      cost: 1
      duration: until_user_next_turn_end
      light:
        bright_radius_ft: 30
        additional_dim_ft: 30
      attacks_against_user:
        disadvantage_if: attacker_can_see_user
      activation_save:
        targets: creatures_in_bright_light_that_see_user
        ability: wisdom
        dc: 15
        failure:
          condition: stunned
          duration: until_effect_ends
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_robe_of_scintillating_colors
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
  outputs:
  - effects
  - item_state
  events:
  - item.used
  capabilities:
  - equipment
  - item_resources
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 283b3a4279df9dba7fe94c78727bcf3e538a1a5c44ee0bf17e1d19324ae932d7
  source_path: builtin/srd-5.2.1/magic_item/robe_of_scintillating_colors.md
  pages:
  - 240
  engine_gaps: &id001
  - dnd2024:item_robe_of_scintillating_colors
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 6d00ff5e5bd97f79c721a5b09cbce83133947c8a131b88981ae8548a37df0e32
splat_clauses:
- key: source_rules
  text: '# Robe of Scintillating Colors Wondrous Item, Very Rare (Requires Attunement) This robe has 3
    charges, and it regains 1d3 expended charges daily at dawn. While you wear it, you can take a Magic
    action and expend 1 charge to cause the garment to display a shifting pattern of dazzling hues until
    the end of your next turn. During this time, the robe sheds Bright Light in a 30-foot radius and Dim
    Light for an additional 30 feet, and creatures that can see you have Disadvantage on attack rolls
    against you. Any creature in the Bright Light that can see you when the robe’s power is activated
    must succeed on a DC 15 Wisdom saving throw or have the Stunned condition until the effect ends.'
  sha256: 9aaf2b7979e371600a05059db6a4db1eba494e49610342c41e46430d295f9961
  status: contract_specified
splat_requires_mechanics: *id001
---

# Robe of Scintillating Colors

<!-- SRD page 240 -->

Wondrous Item, Very Rare (Requires Attunement)\
This robe has 3 charges, and it regains 1d3 ex-\
pended charges daily at dawn. While you wear it,\
you can take a Magic action and expend 1 charge\
to cause the garment to display a shifting pattern\
of dazzling hues until the end of your next turn.\
During this time, the robe sheds Bright Light in a\
30-foot radius and Dim Light for an additional 30\
feet, and creatures that can see you have Disadvan-\
tage on attack rolls against you. Any creature in the\
Bright Light that can see you when the robe’s power\
is activated must succeed on a DC 15 Wisdom sav-\
ing throw or have the Stunned condition until the\
effect ends.\

Original layout: [[../../srd-5.2.1/pages/240|page 240]].
