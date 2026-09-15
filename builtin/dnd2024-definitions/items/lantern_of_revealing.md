---
splat_schema: 1
splat_id: 92510b1f-60d7-5070-bbfd-bb845774eda2
splat_kind: item
splat_key: dnd2024:items/lantern_of_revealing
splat_name: Lantern of Revealing
splat_origin:
  key: dnd2024:reference/magic_item/lantern_of_revealing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 228, 229
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/lantern_of_revealing
  implementation: dnd2024:item_lantern_of_revealing
  parameters:
    fuel:
      oil_pints: 1
      burn_duration:
        value: 6
        unit: hour
    lit:
      bright_radius_ft: 30
      additional_dim_ft: 30
      reveal_invisible:
        targets:
        - creatures
        - objects
        within: bright_light
    lower_hood:
      action: utilize
      replace_light:
        dim_radius_ft: 5
        bright_radius_ft: 0
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_lantern_of_revealing
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
  source_sha256: b138d33db9068984c4b8347eff69b122f14f40095cc8d3017dc481eb108baca3
  source_path: builtin/srd-5.2.1/magic_item/lantern_of_revealing.md
  pages:
  - 228
  - 229
  engine_gaps: &id001
  - dnd2024:item_lantern_of_revealing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: f5321ba73f7ab78ba4942f0cefb22f09dca56be6d9751d44b82615987f5d012d
splat_clauses:
- key: source_rules
  text: '# Lantern of Revealing Wondrous Item, Uncommon While lit, this hooded lantern burns for 6 hours
    on 1 pint of oil, shedding Bright Light in a 30-foot radius and Dim Light for an additional 30 feet.
    Invisible creatures and objects are visible as long as they are in the lantern’s Bright Light. You
    can take a Utilize action to lower the hood, reducing the lantern’s light to Dim Light in a 5-foot
    radius.'
  sha256: d87a7254706f1b2aa72b36c95d1835b855fb39376a8baa5d16b11aeb8e249707
  status: contract_specified
splat_requires_mechanics: *id001
---

# Lantern of Revealing

<!-- SRD page 228 -->

Wondrous Item, Uncommon\
While lit, this hooded lantern burns for 6 hours on 1\
pint of oil, shedding Bright Light in a 30-foot radius\
and Dim Light for an additional 30 feet. Invisible\
creatures and objects are visible as long as they are\
in the lantern’s Bright Light. You can take a Utilize\

<!-- SRD page 229 -->

action to lower the hood, reducing the lantern’s light\
to Dim Light in a 5-foot radius.\

Original layout: [[../../srd-5.2.1/pages/228|page 228]], [[../../srd-5.2.1/pages/229|page 229]].
