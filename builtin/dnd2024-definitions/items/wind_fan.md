---
splat_schema: 1
splat_id: ea3231b8-486e-50f6-b3a8-0802bac13882
splat_kind: item
splat_key: dnd2024:items/wind_fan
splat_name: Wind Fan
splat_origin:
  key: dnd2024:reference/magic_item/wind_fan
splat_sources:
- title: SRD 5.2.1
  locator: pp. 253
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/wind_fan
  implementation: dnd2024:item_wind_fan
  parameters:
    equipment_state: held
    spells:
    - spell:
        key: dnd2024:spells/gust_of_wind
      save_dc: 13
    failure:
      usage_count_reset: dawn
      first_use_failure_chance: 0
      subsequent_cumulative_increment: 0.2
      probability:
        minimum: 0
        maximum: 1
        operation: multiply
        operands:
        - value_from: previous_uses_since_dawn
        - 0.2
      on_failure:
      - no_spell
      - destroy_magic
      - tear_item_unusable
      roll_before_cast: true
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_wind_fan
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
    campaign:
      required: false
      type: campaign
  outputs:
  - effects
  - item_state
  events:
  - item.used
  - spell.cast_completed
  capabilities:
  - equipment
  - item_resources
  - effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 9ddd3f0febb3b8a6f48e7d84849f7497649c87cc6addedd414410b54314e03eb
  source_path: builtin/srd-5.2.1/magic_item/wind_fan.md
  pages:
  - 253
  engine_gaps: &id001
  - dnd2024:item_wind_fan
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 12d81712733e46f0677b11c42b892ecf51bb6cd940b37fe19b2b4dffdd90db10
splat_clauses:
- key: source_rules
  text: '# Wind Fan Wondrous Item, Uncommon While holding this fan, you can cast Gust of Wind (save DC
    13) from it. Each subsequent time the fan is used before the next dawn, it has a cumulative 20 percent
    chance of not working; if the fan fails to work, it tears into useless, nonmagical tatters.'
  sha256: 5a8178ce34a2b32ba47b79017ffd493c20ce5041acfc9476c4455ccd7f50dcdd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wind Fan

<!-- SRD page 253 -->

Wondrous Item, Uncommon\
While holding this fan, you can cast Gust of Wind\
(save DC 13) from it. Each subsequent time the fan\
is used before the next dawn, it has a cumulative\
20 percent chance of not working; if the fan fails to\
work, it tears into useless, nonmagical tatters.\

Original layout: [[../../srd-5.2.1/pages/253|page 253]].
