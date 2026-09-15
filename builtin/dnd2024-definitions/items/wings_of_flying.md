---
splat_schema: 1
splat_id: 0070177d-b32f-5469-a218-258baba75211
splat_kind: item
splat_key: dnd2024:items/wings_of_flying
splat_name: Wings of Flying
splat_origin:
  key: dnd2024:reference/magic_item/wings_of_flying
splat_sources:
- title: SRD 5.2.1
  locator: pp. 253
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/wings_of_flying
  implementation: dnd2024:item_wings_of_flying
  parameters:
    equipped: worn
    activate:
      action: magic
      transform: cloak_to_wings
    effect:
      fly_speed_ft: 60
      duration:
        amount: 1
        unit: hours
      early_end_action: magic
      aloft_on_end: fall
    cooldown:
      starts: wings_disappear
      duration:
        amount:
          roll:
            count: 1
            sides: 12
        unit: hours
    attunement_required: true
splat_contracts:
- id: dnd2024:item_wings_of_flying
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
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - effect_duration
  - item_recharge
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 4fe8dbcbe5bb079b6c97b02c1822cbb664a428ed4c06b948752ee416b10adb14
  source_path: builtin/srd-5.2.1/magic_item/wings_of_flying.md
  pages:
  - 253
  engine_gaps: &id001
  - dnd2024:item_wings_of_flying
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 24fa6dc04999ada63731ef3c4535cafe0da01f98b7b046da79881345b3f17b12
splat_clauses:
- key: source_rules
  text: '# Wings of Flying Wondrous Item, Rare (Requires Attunement) While wearing this cloak, you can
    take a Magic action to turn the cloak into a pair of wings on your back. The wings lasts for 1 hour
    or until you end the effect early as a Magic action. The wings give you a Fly Speed of 60 feet. If
    you are aloft when the wings disappear, you fall. When the wings disappear, you can’t use them again
    for 1d12 hours.'
  sha256: a18fa25da5f19fd0015b6f0b4e57ffbbd6f810f5a1233cc90273c0738c7ba35d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wings of Flying

<!-- SRD page 253 -->

Wondrous Item, Rare (Requires Attunement)\
While wearing this cloak, you can take a Magic ac-\
tion to turn the cloak into a pair of wings on your\
back. The wings lasts for 1 hour or until you end the\
effect early as a Magic action. The wings give you a\
Fly Speed of 60 feet. If you are aloft when the wings\
disappear, you fall. When the wings disappear, you\
can’t use them again for 1d12 hours.\

Original layout: [[../../srd-5.2.1/pages/253|page 253]].
