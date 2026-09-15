---
splat_schema: 1
splat_id: 3073ecdd-0616-53e2-bf40-6218a391d1f3
splat_kind: item
splat_key: dnd2024:items/wand_of_secrets
splat_name: Wand of Secrets
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_secrets
splat_sources:
- title: SRD 5.2.1
  locator: pp. 251
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wand
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/wand_of_secrets
  implementation: dnd2024:item_wand_of_secrets
  parameters:
    equipped: held
    charges:
      initial: 3
      maximum: 3
      recovery:
        event: time.dawn
        amount:
          roll:
            count: 1
            sides: 3
        cap_at_maximum: true
    action: magic
    charge_cost: 1
    detection:
      types:
      - secret_door
      - trap
      range_ft: 60
      target_selection: nearest
      indication:
      - pulse
      - point_to_target
      none_in_range: no_indication
    attunement_required: false
splat_contracts:
- id: dnd2024:item_wand_of_secrets
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
  - time.dawn
  capabilities:
  - item_effects
  - dice
  - item_charges
  - spatial_targeting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: e5e9fe543574d641883004adaceb4e6206a7e33d5a62d20f198db73c836a986e
  source_path: builtin/srd-5.2.1/magic_item/wand_of_secrets.md
  pages:
  - 251
  engine_gaps: &id001
  - dnd2024:item_wand_of_secrets
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 3cb027e960d388fded986d977f12b41d81c74087c2d7ecbdf9279dedd864153d
splat_clauses:
- key: source_rules
  text: '# Wand of Secrets Wand, Uncommon This wand has 3 charges and regains 1d3 expended charges daily
    at dawn. While holding it, you can take a Magic action to expend 1 charge, and if a secret door or
    trap is within 60 feet of you, the wand pulses and points at the one nearest to you.'
  sha256: 9c9a19721de42c42de19d8ee03844bab5d37217dc3e4823d238edbe54784265d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wand of Secrets

<!-- SRD page 251 -->

Wand, Uncommon\
This wand has 3 charges and regains 1d3 expended\
charges daily at dawn. While holding it, you can\
take a Magic action to expend 1 charge, and if a se-\
cret door or trap is within 60 feet of you, the wand\
pulses and points at the one nearest to you.\

Original layout: [[../../srd-5.2.1/pages/251|page 251]].
